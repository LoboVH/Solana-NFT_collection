use anchor_lang::prelude::*;
use anchor_lang::Discriminator;
use anchor_lang::solana_program::{
    program::{invoke_signed},
    system_instruction,
};
use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, mint_to, MintTo, Token, TokenAccount, Transfer}};
use anchor_lang::solana_program::program::invoke;
    use anchor_spl::token;
use switchboard_v2::AggregatorAccountData;
use mpl_token_metadata::{ 
    accounts::{ MasterEdition, Metadata as MetadataAccount },
    types::{CollectionDetails, DataV2, Creator},
    instructions::{CreateMasterEditionV3, CreateMetadataAccountV3, SetAndVerifySizedCollectionItem, SignMetadata, CreateMasterEditionV3InstructionArgs, CreateMetadataAccountV3InstructionArgs }
};
use spl_token::state::Account as SplTokenAccount;

use std::mem::size_of;
use std::str::FromStr;

#[constant]
pub const SOL_USDC_FEED: &str = "GvDMxPzN1sCj7L26YDK2HnMRXEQmQ2aemov8YBtPS7vR";

declare_id!("C4K3QPmcaN6JSLmwNFvZ5ZwSQV3tyCnMbTDY4tVN94sk");

#[program]
pub mod coll_nft {

    use super::*;
    use anchor_lang::solana_program::program_pack::Pack;

    pub fn initialize_treasury(ctx: Context<InitializeTreasury>, whitelist_mints: Option<WhitelistMintSettings>) -> Result<()> {
        let treasury = &mut ctx.accounts.treasury;
        treasury.authority = ctx.accounts.authority.key();
        treasury.bark_baller_bundle_price = 100; // setting it to basic price in Dollars
        treasury.fur_real_deal_price = 150; // setting it to basic price in Dollars
        treasury.purrmium_pack_price = 200; // setting it to basic price in Dollars
        treasury.bark_baller_bundle_count = 12_000;
        treasury.fur_real_deal_count = 6_000;
        treasury.purrmium_pack_count = 2_000;
        treasury.nomaimai = 15;
        treasury.ridiculous_dragon = 20;
        treasury.nomimai_ridiculous_dragon = 30;

        if let Some(whitelist_mint_settings) = whitelist_mints {
            treasury.whitelist_mint_settings = Some(whitelist_mint_settings);
        }

        Ok(())
    }

    pub fn update_collection_prices(
        ctx: Context<UpdateCollectionPrices>,
        collection_a: Option<u64>,
        collection_b: Option<u64>,
        collection_c: Option<u64>,
     ) -> Result<()> {
        let treasury = &mut ctx.accounts.treasury;

        // Update prices if provided
        if let Some(price) = collection_a {
            treasury.bark_baller_bundle_price = price;
        }

        if let Some(price) = collection_b {
            treasury.fur_real_deal_price = price;
        }

        if let Some(price) = collection_c {
            treasury.purrmium_pack_price = price;
        }

        Ok(())
    }  


    pub fn update_whitelist_settings(
        ctx: Context<UpdateWhitelistSettings>,
        whitelist_mints: Option<WhitelistMintSettings>
     ) -> Result<()> {
        let treasury = &mut ctx.accounts.treasury;

        if let Some(whitelist_mint_settings) = whitelist_mints {
            treasury.whitelist_mint_settings = Some(whitelist_mint_settings);
        }

        Ok(())
    }
    


    pub fn claim_funds(ctx: Context<ClaimFunds>, amount: u64) -> Result<()> {
        let treasury_account_info = &mut ctx.accounts.treasury.to_account_info();
        let admin_account_info = &mut ctx.accounts.authority.to_account_info();
    
        // Ensure that only the admin can claim the funds
        if ctx.accounts.authority.key() != ctx.accounts.treasury.authority {
            return Err(ErrorCode::Unauthorized.into()); // Use custom error
        }
        
        // Ensure that the treasury account has enough SOL to cover the transfer
        let treasury_lamports = treasury_account_info.lamports();
        if treasury_lamports < amount {
            return Err(ErrorCode::InsufficientFunds.into()); // Use custom error
        }


        if let Some(usdt) = &ctx.accounts.user_usdt_wallet {
            let treasury_usdt = ctx.accounts.treasury_usdt_wallet.amount;
            if treasury_usdt < amount {
                return Err(ErrorCode::InsufficientFunds.into());
            }

            let cpi_accounts = Transfer {
                from: ctx.accounts.treasury_usdt_wallet.to_account_info(),
                to: usdt.to_account_info(),
                authority: ctx.accounts.treasury.to_account_info(),
            };
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

            let usd_amt: u64 = amount * 10u64.pow(6);
            token::transfer(cpi_ctx, usd_amt)?;

        } else {
            // Ensure that the treasury account has enough SOL to cover the transfer
            let treasury_lamports = treasury_account_info.lamports();
            if treasury_lamports < amount {
                return Err(ErrorCode::InsufficientFunds.into()); 
            }

            // Perform direct lamport manipulation
            **treasury_account_info.try_borrow_mut_lamports()? -= amount;
            **admin_account_info.try_borrow_mut_lamports()? += amount;

        }
    
        Ok(())
    }

    pub fn create_collection_nft(
        ctx: Context<CreateCollectionNft>,
        uri: String,
        name: String,
        symbol: String
    ) -> Result<()> {
        let signer_seeds: &[&[&[u8]]] = &[&[
            "gululu_collection".as_bytes(),
            &[ctx.bumps["collection_mint"]],
        ]];
        msg!("SIGNER SEEDS GENERATED");

        // mint collection nft
        mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    mint: ctx.accounts.collection_mint.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info(),
                    authority: ctx.accounts.collection_mint.to_account_info(),
                },
                signer_seeds,
            ),
            1,
        )?;

        let account_info = vec![
            ctx.accounts.metadata_account.to_account_info(),
            ctx.accounts.collection_mint.to_account_info(),
            ctx.accounts.authority.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];

        let create_token_instruction_data = &CreateMetadataAccountV3{
            metadata: ctx.accounts.metadata_account.key(),
            mint: ctx.accounts.collection_mint.key(),
            mint_authority: ctx.accounts.collection_mint.key(), // use pda mint address as mint authority
            update_authority: (ctx.accounts.collection_mint.key(), true), // use pda mint as update authority
            payer: ctx.accounts.authority.key(),
            system_program: ctx.accounts.system_program.key(),
            rent: Some(ctx.accounts.rent.key()),
        }
        .instruction(CreateMetadataAccountV3InstructionArgs {
            data: DataV2 {
                name: name,
                symbol: symbol,
                uri: uri,
                seller_fee_basis_points: 0,
                creators: Some(vec![Creator {
                    address: ctx.accounts.authority.key(),
                    verified: false,
                    share: 100,
                }]),
                collection: None,
                uses: None,
            },
            is_mutable: true,
            collection_details: Some(CollectionDetails::V1 { size: 0 }), 
        });
        invoke_signed(
            create_token_instruction_data,
            account_info.as_slice(),
            &signer_seeds,
        )?;

        let account_info_master_edition = vec![
            ctx.accounts.metadata_account.to_account_info(),
            ctx.accounts.master_edition.to_account_info(),
            ctx.accounts.collection_mint.to_account_info(),
            ctx.accounts.authority.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];

        let create_master_edition_v3 = &CreateMasterEditionV3 {
            payer: ctx.accounts.authority.key(),
            mint: ctx.accounts.collection_mint.key(),
            edition: ctx.accounts.master_edition.key(),
            mint_authority: ctx.accounts.collection_mint.key(),
            update_authority: ctx.accounts.collection_mint.key(),
            metadata: ctx.accounts.metadata_account.key(),
            token_program: ctx.accounts.token_program.key(),
            system_program: ctx.accounts.system_program.key(),
            rent: Some(ctx.accounts.rent.key()),
        }.instruction(CreateMasterEditionV3InstructionArgs{
            max_supply: Some(0),
        });
        invoke_signed(
            create_master_edition_v3,
            account_info_master_edition.as_slice(),
            &signer_seeds,
        )?;


        let account_info_sign_data = vec![
            ctx.accounts.metadata_account.to_account_info(),
            ctx.accounts.authority.to_account_info(),
        ];

        let sign_metadata_instruction = &SignMetadata {
            creator: ctx.accounts.authority.key(),
            metadata: ctx.accounts.metadata_account.key()
        }.instruction();
        invoke(
            sign_metadata_instruction,
            account_info_sign_data.as_slice(),
        )?;

        Ok(())
    }

    pub fn buy_collection_nft<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, CreateNftInCollection<'info>>,
        category: Category,
    ) -> Result<()> {
        let remaining_accounts: &'c [AccountInfo<'info>] = &ctx.remaining_accounts;

        let metadata_account_info = &ctx.remaining_accounts[0];
        let master_edition_info = &ctx.remaining_accounts[1];
        let collection_metadata_account_info = &ctx.remaining_accounts[2];
        let collection_master_edition_info = &ctx.remaining_accounts[3];

        // Validate metadata_account
        let expected_metadata_account_key = MetadataAccount::find_pda(&ctx.accounts.nft_mint.key()).0;
        if metadata_account_info.key != &expected_metadata_account_key {
            return Err(ErrorCode::InvalidMetadataAccount.into());
        }

        // Validate master_edition
        let expected_master_edition_key = MasterEdition::find_pda(&ctx.accounts.nft_mint.key()).0;
        if master_edition_info.key != &expected_master_edition_key {
            return Err(ErrorCode::InvalidMasterEditionAccount.into());
        }

        // Validate collection_metadata_account
        let expected_collection_metadata_account_key = MetadataAccount::find_pda(&ctx.accounts.collection_mint.key()).0;
        if collection_metadata_account_info.key != &expected_collection_metadata_account_key {
            return Err(ErrorCode::InvalidCollectionMetadataAccount.into());
        }

        // Validate collection_master_edition
        let expected_collection_master_edition_key = MasterEdition::find_pda(&ctx.accounts.collection_mint.key()).0;
        if collection_master_edition_info.key != &expected_collection_master_edition_key {
            return Err(ErrorCode::InvalidCollectionMasterEditionAccount.into());
        }


        let treasury = &mut ctx.accounts.treasury;
        let category_count: u16;
        let symbol: String;

        let mut transfer_amount = match category {
            Category::BarkBallerBundle => {
                if treasury.bark_baller_bundle_count == 0 {
                    // If the count has exceeded the limit, return an error
                    return Err(ErrorCode::BarkBallerBundleMintEnded.into());
                } else {
                    // If not exceeded, increment the count
                    let amount = treasury.bark_baller_bundle_price;
                    treasury.bark_baller_bundle_count -= 1;
                    category_count = treasury.bark_baller_bundle_count;
                    symbol = String::from("BBB");
                    amount
                }
            },
            Category::FurRealDeal => {
                if treasury.fur_real_deal_count == 0 {
                    // If the count has exceeded the limit, return an error
                    return Err(ErrorCode::FurRealDealMintEnded.into());
                } else {
                    // If not exceeded, increment the count
                    let amount = treasury.fur_real_deal_price;
                    treasury.fur_real_deal_count -= 1;
                    category_count = treasury.fur_real_deal_count;
                    symbol = String::from("FRD");
                    amount
                }
            },
            Category::PurrmiumPack => {
                if treasury.purrmium_pack_count == 0 {
                    // If the count has exceeded the limit, return an error
                    return Err(ErrorCode::PurrmiumPackMintEnded.into());
                } else {
                    // If not exceeded, increment the count
                    let amount = treasury.purrmium_pack_price;
                    treasury.purrmium_pack_count -= 1;
                    category_count = treasury.purrmium_pack_count;
                    symbol = String::from("PRP");
                    amount
                }
            },
        };


        if let Some(whitelist_settings) = &treasury.whitelist_mint_settings {
            msg!("Inside Whitelist Settings");
            
            
            let nomaimai_token = remaining_accounts.get(4)
                .and_then(|account| account.try_borrow_data().ok())
                .and_then(|data| SplTokenAccount::unpack(&data).ok())
                .filter(|token| token.mint == whitelist_settings.nomaimai_mint);
    
            let ridiculous_dragon_token = remaining_accounts.get(5)
                .and_then(|account| account.try_borrow_data().ok())
                .and_then(|data| SplTokenAccount::unpack(&data).ok())
                .filter(|token| token.mint == whitelist_settings.ridiculous_dragon_mint);
    
            let nominai_ridiculous_token = remaining_accounts.get(6)
                .and_then(|account| account.try_borrow_data().ok())
                .and_then(|data| SplTokenAccount::unpack(&data).ok())
                .filter(|token| token.mint == whitelist_settings.nomaimai_ridiculous_mint);
    
            if let Some(token) = nominai_ridiculous_token {
                if token.amount > 0 {
                    let disc = treasury.nomimai_ridiculous_dragon;
                    let disc_amt = transfer_amount * disc / 100;
                    transfer_amount = transfer_amount.saturating_sub(disc_amt);
                    msg!("Transfer amount after Nominai Ridiculous discount: {}", transfer_amount);
                } else {
                    msg!("Nominai Ridiculous token account empty, hence user not eligible for discount");
                }
            } else if let Some(token) = nomaimai_token {
                if token.amount > 0 {
                    let disc = treasury.nomaimai;
                    let disc_amt = transfer_amount * disc / 100;
                    transfer_amount = transfer_amount.saturating_sub(disc_amt);
                    msg!("Transfer amount after Nomaimai discount: {}", transfer_amount);
                } else {
                    msg!("Nomaimai token account empty, hence user not eligible for discount");
                }
            } else if let Some(token) = ridiculous_dragon_token {
                if token.amount > 0 {
                    let disc = treasury.ridiculous_dragon;
                    let disc_amt = transfer_amount * disc / 100;
                    transfer_amount = transfer_amount.saturating_sub(disc_amt);
                    msg!("Transfer amount after Ridiculous Dragon discount: {}", transfer_amount);
                } else {
                    msg!("Ridiculous Dragon token account empty, hence user not eligible for discount");
                }
            } else {
                msg!("No valid whitelist token accounts provided");
            }
        }

        msg!("Transfer amount after discount: {}", transfer_amount);


        let feed = &ctx.accounts.feed_aggregator.load()?;
        
         let price: f64 = feed.get_result()?.try_into()?;
 
         // check whether the feed has been updated in the last 300 seconds
         feed.check_staleness(Clock::get().unwrap().unix_timestamp, 300)
         .map_err(|_| {
             msg!("Price feed data is too stale");
             anchor_lang::error::Error::from(ErrorCode::PriceFeedIsDown)
         })?;


         let mut transfer_amt_digts = transfer_amount * LAMPORTS_PER_SOL;
         msg!("Transfer amount digits:{}", transfer_amt_digts);

         if let Some(referrer) = &ctx.accounts.referrer {
            msg!("Handling referrer logic");
            let commission_amount = transfer_amt_digts as f64 * 0.1;
            transfer_amt_digts -= commission_amount as u64;
            msg!("transfer amount f64 after commision disc: {}", transfer_amt_digts);
            msg!("commission_amount: {}", commission_amount);

            let seeds = &[b"referral".as_ref(), referrer.key.as_ref()];

            let (referrer_user_key, bump) = Pubkey::find_program_address(seeds, ctx.program_id);
            if let Some(referrer_user) = &mut ctx.accounts.referrer_user {
                msg!("initialising refferrer");
                if referrer_user.to_account_info().data_is_empty() {
                    invoke_signed(
                        &system_instruction::create_account(
                            &ctx.accounts.user.key,
                            &referrer_user_key,
                            Rent::get()?.minimum_balance(size_of::<ReferrerUser>() + 100),
                            size_of::<ReferrerUser>() as u64 + 8,  
                            //size_of::<Mint>() as u64
                            &ctx.program_id,
                        ),
                        &[
                            ctx.accounts.user.to_account_info(),
                            referrer_user.to_account_info(),
                            ctx.accounts.system_program.to_account_info(),
                        ],
                        &[&[b"referral".as_ref(), referrer.key.as_ref(), &[bump]]],
                    )?;

                    let data = &mut referrer_user.data.borrow_mut()[..];
                    for (index, value) in ReferrerUser::DISCRIMINATOR.iter().enumerate() {
                        data[index] = *value;
                    }
                }

                let state_res: Result<Account<ReferrerUser>> = Account::try_from(referrer_user);

                if let Some(ussdt) = &ctx.accounts.user_usdt_wallet {
                    if let Some(referrer_usdt) = &ctx.accounts.user_usdt_wallet {
                        msg!("Transferring USDT commission to referrer");
                        let cpi_accounts = Transfer {
                            from: ussdt.to_account_info(),
                            to: referrer_usdt.to_account_info(),
                            authority: ctx.accounts.user.to_account_info(),
                        };
                        let cpi_program = ctx.accounts.token_program.to_account_info();
                        let usd_commission_f64 = commission_amount / 1000.00;
                        let usd_commission = usd_commission_f64.round() as u64;
                        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
                        token::transfer(cpi_ctx, usd_commission)?;
                        msg!("USD REFERRAL COMMISSTION: {}", usd_commission);

                        if let Ok(mut state) = state_res {
                            state.referral_usdt += usd_commission; // update
                                                                   // update on chain data (actual)
                            let data = &mut &mut referrer_user.data.borrow_mut()[..];
                            state.try_serialize(data)?;
                        }
                    }
                } else {
                    let commission_f64 = commission_amount / price;
                    let final_commision_amount = commission_f64.round() as u64;
                    msg!("Transferring SOL commission: {} ,to referrer", final_commision_amount);
                    let transfer_ix = system_instruction::transfer(
                        &ctx.accounts.user.key(),
                        &referrer.key(),
                        final_commision_amount,
                    );

                    // Invoke the transfer instruction
                    anchor_lang::solana_program::program::invoke_signed(
                        &transfer_ix,
                        &[
                            ctx.accounts.user.to_account_info(),
                            referrer.to_account_info(),
                            ctx.accounts.system_program.to_account_info(),
                        ],
                        &[],
                    )?;

                    if let Ok(mut state) = state_res {
                        state.referral_sol += final_commision_amount; // update
                                                                      // update on chain data (actual)
                        let data = &mut &mut referrer_user.data.borrow_mut()[..];
                        state.try_serialize(data)?;
                    }
                }
            }
        }

         if let Some(usdt) = &ctx.accounts.user_usdt_wallet {
            let cpi_accounts = Transfer {
                from: usdt.to_account_info(),
                to: ctx.accounts.treasury_usdt_wallet.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            };
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

            let usd_amt: u64 = transfer_amt_digts / 1000;
            msg!("USD transfer amount: {}", usd_amt);
            token::transfer(cpi_ctx, usd_amt)?;

        } else {
            let final_sol_transfer_amt_f64 = transfer_amt_digts as f64 / price;
            let final_sol_transfer_amt = final_sol_transfer_amt_f64.round() as u64;

            let transfer_instruction =
                system_instruction::transfer(&ctx.accounts.user.key(), &treasury.key(), final_sol_transfer_amt);

            // Invoke the transfer instruction
            anchor_lang::solana_program::program::invoke_signed(
                &transfer_instruction,
                &[
                    ctx.accounts.user.to_account_info(),
                    treasury.to_account_info(),
                    ctx.accounts.system_program.to_account_info(),
                ],
                &[],
            )?;
        }

        // Generating signer seeds
        let signer_seeds: &[&[&[u8]]] = &[&[
            "gululu_collection".as_bytes(),
            &[ctx.bumps["collection_mint"]],
        ]];

        let name = category.to_string();

        let uri = format!(" https://ipfs.io/ipfs{}/{}.json", name, category_count);


        // mint nft in collection
        mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    mint: ctx.accounts.nft_mint.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info(),
                    authority: ctx.accounts.collection_mint.to_account_info(),
                },
                signer_seeds,
            ),
            1,
        )?;

        let account_info = vec![
            metadata_account_info.to_account_info(),
            ctx.accounts.nft_mint.to_account_info(),
            ctx.accounts.collection_mint.to_account_info(),
            ctx.accounts.user.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];

        let create_token_instruction_data = &CreateMetadataAccountV3{
            metadata: metadata_account_info.key(),
            mint: ctx.accounts.nft_mint.key(),
            mint_authority: ctx.accounts.collection_mint.key(), // use pda mint address as mint authority
            update_authority: (ctx.accounts.collection_mint.key(), true), // use pda mint as update authority
            payer: ctx.accounts.user.key(),
            system_program: ctx.accounts.system_program.key(),
            rent: Some(ctx.accounts.rent.key()),
        }
        .instruction(CreateMetadataAccountV3InstructionArgs {
            data: DataV2 {
                name: name,
                symbol: symbol,
                uri: uri,
                seller_fee_basis_points: 0,
                creators: None,
                collection: None,
                uses: None,
            },
            is_mutable: true,
            collection_details: None, 
        });
        invoke_signed(
            create_token_instruction_data,
            account_info.as_slice(),
            &signer_seeds,
        )?;


        let account_info_master_edition = vec![
            metadata_account_info.to_account_info(),
            master_edition_info.to_account_info(),
            ctx.accounts.nft_mint.to_account_info(),
            ctx.accounts.collection_mint.to_account_info(),
            ctx.accounts.user.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];

        let create_master_edition_v3 = &CreateMasterEditionV3 {
            payer: ctx.accounts.user.key(),
            mint: ctx.accounts.nft_mint.key(),
            edition: master_edition_info.key(),
            mint_authority: ctx.accounts.collection_mint.key(),
            update_authority: ctx.accounts.collection_mint.key(),
            metadata: metadata_account_info.key(),
            token_program: ctx.accounts.token_program.key(),
            system_program: ctx.accounts.system_program.key(),
            rent: Some(ctx.accounts.rent.key()),
        }.instruction(CreateMasterEditionV3InstructionArgs{
            max_supply: Some(0),
        });
        invoke_signed(
            create_master_edition_v3,
            account_info_master_edition.as_slice(),
            &signer_seeds,
        )?;


        let account_info_set_and_verify_sized_collection = vec![
            metadata_account_info.to_account_info(),
            collection_metadata_account_info.to_account_info(),
            ctx.accounts.collection_mint.to_account_info(),
            ctx.accounts.user.to_account_info(),
            collection_master_edition_info.to_account_info()
        ];

        let set_and_verify_sized_collection_item = &SetAndVerifySizedCollectionItem {
            metadata: metadata_account_info.key(),
            collection_authority: ctx.accounts.collection_mint.key(),
            payer: ctx.accounts.user.key(),
            update_authority: ctx.accounts.collection_mint.key(),
            collection_mint: ctx.accounts.collection_mint.key(),
            collection: collection_metadata_account_info.key(),
            collection_master_edition_account: collection_master_edition_info.key(),
            collection_authority_record: None
        }.instruction();

        invoke_signed(
            set_and_verify_sized_collection_item,
            account_info_set_and_verify_sized_collection.as_slice(),
            &signer_seeds,
        )?;


        Ok(())
    }

}


#[derive(Accounts)]
#[instruction(uri: String, name: String, symbol: String)]
pub struct CreateCollectionNft<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        seeds = [b"gululu_collection"],
        bump,
        payer = authority,
        mint::decimals = 0,
        mint::authority = collection_mint,
        mint::freeze_authority = collection_mint
    )]
    pub collection_mint: Account<'info, Mint>,

    /// CHECK:
    #[account(
        mut,
        address=MetadataAccount::find_pda(&collection_mint.key()).0
    )]
    pub metadata_account: UncheckedAccount<'info>,

    /// CHECK:
    #[account(
        mut,
        address=MasterEdition::find_pda(&collection_mint.key()).0
    )]
    pub master_edition: UncheckedAccount<'info>,

    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = collection_mint,
        associated_token::authority = authority
    )]
    pub token_account: Box<Account<'info, TokenAccount>>,
    // #[account(mut, seeds = [b"counter"], bump)]
    // pub counter: Account<'info, Counter>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// CHECK: account constraint checked in account trait
    #[account(address = mpl_token_metadata::ID)]
    pub token_metadata_program: UncheckedAccount<'info>,
    pub rent: Sysvar<'info, Rent>,
}


#[derive(Accounts)]
#[instruction(category: Category)]
pub struct CreateNftInCollection<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"treasury"],
        bump
    )]
    pub treasury: Box<Account<'info, Treasury>>,
    #[account(
        mut,
        seeds = [b"gululu_collection"],
        bump,
    )]
    pub collection_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = user,
        mint::decimals = 0,
        mint::authority = collection_mint,
        mint::freeze_authority = collection_mint
    )]
    pub nft_mint: Account<'info, Mint>,
    #[account(
        address = Pubkey::from_str(SOL_USDC_FEED).unwrap() @ ErrorCode::InvalidPriceFeed
    )]
    pub feed_aggregator: AccountLoader<'info, AggregatorAccountData>,
    #[account(mut)]
    pub user_usdt_wallet: Option<Account<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = usdt,
        associated_token::authority = treasury,
    )]
    pub treasury_usdt_wallet: Box<Account<'info, TokenAccount>>,
    /// CHECK:
    #[account(mut)]
    pub usdt: AccountInfo<'info>,
    #[account(mut)]
    pub referrer_user: Option<AccountInfo<'info>>,
    #[account(mut)]
    pub referrer: Option<AccountInfo<'info>>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = usdt,
        associated_token::authority = referrer,
    )]
    pub referrer_usdt_wallet: Option<Account<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = nft_mint,
        associated_token::authority = user
    )]
    pub token_account: Box<Account<'info, TokenAccount>>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// CHECK: account constraint checked in account trait
    #[account(address = mpl_token_metadata::ID)]
    pub token_metadata_program: UncheckedAccount<'info>,
    pub rent: Sysvar<'info, Rent>,
}


#[derive(Accounts)]
pub struct InitializeTreasury<'info> {
    //32 + 8 + 8 + 8
    #[account(init, seeds = [b"treasury"], payer = authority, space = 8 + size_of::<Treasury>() , bump)] 
    pub treasury: Box<Account<'info, Treasury>>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct UpdateCollectionPrices<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        has_one = authority,
        seeds = [b"treasury"],
        bump
    )]
    pub treasury: Box<Account<'info, Treasury>>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct UpdateWhitelistSettings<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        has_one = authority,
        seeds = [b"treasury"],
        bump
    )]
    pub treasury: Box<Account<'info, Treasury>>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct ClaimFunds<'info> {
    #[account(
        mut,
        has_one = authority,
        seeds = [b"treasury"],
        bump
    )]
    pub treasury: Box<Account<'info, Treasury>>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub user_usdt_wallet: Option<Account<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = usdt,
        associated_token::authority = treasury,
    )]
    pub treasury_usdt_wallet: Account<'info, TokenAccount>,
    ///CHECK:
    #[account(mut)]
    pub usdt: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}



#[account]
pub struct Treasury {
    pub authority: Pubkey,
    pub bark_baller_bundle_price: u64,
    pub fur_real_deal_price: u64,
    pub purrmium_pack_price: u64,
    pub bark_baller_bundle_count: u16,
    pub fur_real_deal_count: u16,
    pub purrmium_pack_count: u16,
    pub nomaimai: u64,
    pub ridiculous_dragon: u64,
    pub nomimai_ridiculous_dragon: u64,
    pub whitelist_mint_settings: Option<WhitelistMintSettings>,
}

#[account]
pub struct ReferrerUser {
    pub referral_sol: u64,
    pub referral_usdt: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct WhitelistMintSettings {
    pub nomaimai_mint: Pubkey,
    pub ridiculous_dragon_mint: Pubkey,
    pub nomaimai_ridiculous_mint: Pubkey
}


#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct InitNFTParams {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}


#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum Category {
    BarkBallerBundle,
    FurRealDeal,
    PurrmiumPack
}

impl ToString for Category {
    fn to_string(&self) -> String {
        match self {
            Category::BarkBallerBundle => "BarkBallerBundle".to_string(),
            Category::FurRealDeal => "FurRealDeal".to_string(),
            Category::PurrmiumPack => "PurrmiumPack".to_string(),
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum CryptoMon {
    Nomaimai,
    RidiculousDragon,
    NomimaiRidiculousDragon
}


#[error_code]
pub enum ErrorCode { 
    #[msg("Invalid Metadata Account provided.")] 
    InvalidMetadataAccount, 
    #[msg("Invalid Master Edition Account provided.")] 
    InvalidMasterEditionAccount, 
    #[msg("Invalid Collection Metadata Account provided.")] 
    InvalidCollectionMetadataAccount, 
    #[msg("Invalid Collection Master Edition Account provided.")] 
    InvalidCollectionMasterEditionAccount,
    #[msg("Unauthorized action.")]
    Unauthorized,
    #[msg("Insufficient funds in the treasury account.")]
    InsufficientFunds, 
    #[msg("Discount can not be more than 100")]
    InvalidDiscount,
    #[msg("Price Feed is down at the moment")]
    PriceFeedIsDown,
    #[msg("Invalid Price Feed Address")]
    InvalidPriceFeed,
    #[msg("BarkBallerBundle Category Mint Limit Exceeded")]
    BarkBallerBundleLimitExceeded,
    #[msg("FurRealDeal Category Mint Limit Exceeded")]
    FurRealDealLimitExceeded,
    #[msg("PurrmiumPack Category Mint Limit Exceeded")]
    PurrmiumPackLimitExceeded,  
    #[msg("BarkBallerBundle collection has ended. No more NFTs are available to mint in this category. Check other categories")]
    BarkBallerBundleMintEnded,
    #[msg("FurRealDeal collection has ended. No more NFTs are available to mint in this category. Check other categories")]
    FurRealDealMintEnded,
    #[msg("PurrmiumPack collection has ended. No more NFTs are available to mint in this category. Check other categories")]
    PurrmiumPackMintEnded, 
    #[msg("Warning: Invalid Whitelist token, Only whitelisted tokens are allowed")]
    InvalidMint,
}




