import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CollNft } from "../target/types/coll_nft";
import {
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  Transaction,
} from "@solana/web3.js";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAssociatedTokenAddress,
  TOKEN_PROGRAM_ID
} from "@solana/spl-token";
import {
  findMetadataPda,
  findMasterEditionPda,
  mplTokenMetadata,
  fetchDigitalAssetWithTokenByMint,
} from "@metaplex-foundation/mpl-token-metadata";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { walletAdapterIdentity } from "@metaplex-foundation/umi-signer-wallet-adapters";
import { publicKey, transactionBuilder } from "@metaplex-foundation/umi";
import {
  MPL_TOKEN_METADATA_PROGRAM_ID as METADATA_PROGRAM_ID,
  Metadata
} from "@metaplex-foundation/mpl-token-metadata";
import { assert } from "chai";


describe("anchor-nft-collection", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.CollNft as Program<CollNft>;

  const wallet = provider.wallet as anchor.Wallet;
  const connection = provider.connection;

  const testMetadata = {
    uri: "https://arweave.net/h19GMcMz7RLDY7kAHGWeWolHTmO83mLLMNPzEkF32BQ",
    name: "NAME",
    symbol: "SYMBOL"
  };

  const testCollectionMetadata = {
    uri: "https://arweave.net/h19GMcMz7RLDY7kAHGWeWolHTmO83mLLMNPzEkF32BQ",
    name: "Gul",
    symbol: "SYMBOL"
  };

  const signer = provider.wallet;
  const umi = createUmi("https://api.devnet.solana.com")
    .use(walletAdapterIdentity(signer))
    .use(mplTokenMetadata());

  const usdt = new PublicKey("Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr");
  //SWITCHBOARD PRICE FEED
  const solUsedSwitchboardFeed = new anchor.web3.PublicKey("GvDMxPzN1sCj7L26YDK2HnMRXEQmQ2aemov8YBtPS7vR");


  const [collectionPDA] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("gululu_collection")],
    program.programId
  );

  const [treasuryPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("treasury")],
    program.programId
  );

  // it("Initilaizie Treasury ", async () => {
  //   let acccInfo = await provider.connection.getAccountInfo(treasuryPda);

  //   if(acccInfo) {
  //     console.log("TREAsury PDA acc INFO:", acccInfo);
  //     console.log("TREAsury PDA is already initialized:");
  //     return
  //   }

  //       let whitelist = null;  
  //       let whitelistMints = null;

  //       if (whitelist) {
  //         whitelistMints = {
  //           nomaimaiMint: new PublicKey("<MINT_KEY_GOES_HERE>"),   // discount spl token mint keys
  //           nomaimaiRidiculousMint: new PublicKey("<MINT_KEY_GOES_HERE>") ,
  //           ridiculousDragonMint: new PublicKey("<MINT_KEY_GOES_HERE>")
  //         }
  //       }
        

  //   try {
  //     const tx = await program.methods
  //     .initializeTreasury(whitelistMints)
  //     .accounts({
  //       treasury: treasuryPda,
  //       authority: provider.wallet.publicKey  // admin wallet
  //     })
  //     .rpc();
  //   //.transaction();

  //   await connection.confirmTransaction(tx, 'confirmed')
  //   } catch(err) {
  //     console.log("ERROR:", err)
  //   }

  //   let treauryAccount = await program.account.treasury.fetch(treasuryPda);
  
  //   console.log("Treasury Account Details:", treauryAccount.barkBallerBundlePrice)

  //  })


  //  it("Update Collection Prices ", async () => {
  //   const [treasuryPda] = anchor.web3.PublicKey.findProgramAddressSync(
  //     [Buffer.from("treasury")],
  //     program.programId
  //   );

  //   let acccInfo = await provider.connection.getAccountInfo(treasuryPda);

  //   if(!acccInfo) {
  //     console.log("Treasury accoount is not initialized, init treasury");
  //     return;
  //   }

  //   const categoryPrices = {
  //     barkBallerBundle : new anchor.BN(0.03 * LAMPORTS_PER_SOL) ,  // null
  //     FurRealDeal : null,  // new anchor.BN(5 * LAMPORTS_PER_SOL)
  //     PurrmiumPack : new anchor.BN(0.02 * LAMPORTS_PER_SOL),  // null
  //   }

  //   try {
  //     const tx = await program.methods
  //     .updateCollectionPrices(categoryPrices.barkBallerBundle, categoryPrices.FurRealDeal, categoryPrices.PurrmiumPack)
  //     .accounts({
  //       treasury: treasuryPda,
  //       authority: provider.wallet.publicKey  // admin wallet
  //     })
  //     .rpc();
  //   //.transaction();

  //   await connection.confirmTransaction(tx, 'confirmed').catch((e) => { console.log("Error tx:", e)})
  //   } catch(err) {
  //     console.log("ERROR:", err)
  //   }

  //   let treauryAccount = await program.account.treasury.fetch(treasuryPda);
  //   console.log("Treasury Account Details:", treauryAccount)
  //   console.log("Treasury Account Details:", treauryAccount.barkBallerBundlePrice.toNumber())
  //  })



  // it("Update whitelist settings ", async () => {
  //   const [treasuryPda] = anchor.web3.PublicKey.findProgramAddressSync(
  //     [Buffer.from("treasury")],
  //     program.programId
  //   );

  //   let acccInfo = await provider.connection.getAccountInfo(treasuryPda);

  //   if(!acccInfo) {
  //     console.log("Treasury accoount is not initialized, init treasury");
  //     return;
  //   }

  //   let whitelistMints = {
  //               nomaimaiMint: new PublicKey("Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr"),   // discount spl token mint keys
  //               nomaimaiRidiculousMint: new PublicKey("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU") ,
  //               ridiculousDragonMint: new PublicKey("7UqBTQnoswV2bPL3ZYvtLrhwXhX1oz91kSLTeHFiiwY2")
  //             }

  //   try {
  //     const tx = await program.methods
  //     .updateWhitelistSettings(whitelistMints)
  //     .accounts({
  //       treasury: treasuryPda,
  //       authority: provider.wallet.publicKey  // admin wallet
  //     })
  //     .rpc();
  //   //.transaction();

  //   await connection.confirmTransaction(tx, 'confirmed').catch((e) => { console.log("Error tx:", e)})
  //   } catch(err) {
  //     console.log("ERROR:", err)
  //   }

  //   let treauryAccount = await program.account.treasury.fetch(treasuryPda);
  //   console.log("Treasury Account Details:", treauryAccount)
  //   console.log("Treasury Account Details:", treauryAccount.whitelistMintSettings)
  //  })


   


  //  it("Claim Treasury Fund ", async () => {
    
  //   const [treasuryPda] = anchor.web3.PublicKey.findProgramAddressSync(
  //     [Buffer.from("treasury")],
  //     program.programId
  //   );

  //   const treasuryUsdtWallet = await getAssociatedTokenAddress(
  //     usdt,
  //     treasuryPda,
  //     true
  //   );

  //   let userUsdtWallet;
  //   let USDT = false;      // selectedCurrency === "USDT"; // TURN THIS TO TRUE WHEN CLAIMING USDT

  //   if (USDT) {
  //     userUsdtWallet = await getAssociatedTokenAddress(
  //       usdt,
  //       wallet.publicKey
  //     );

  //     let pdaUsdtBalance = (await connection.getTokenAccountBalance(userUsdtWallet)).value.uiAmount;
  //     console.log("PDA balance:", pdaUsdtBalance);
  //     if (pdaUsdtBalance < 0) {
  //       console.log("There is no USDT amount in treasury account to claim");
  //       return;
  //     }
  //   } else {
  //     userUsdtWallet = null;
  //   }

  //   let pdaBalance = await connection.getBalance(treasuryPda);

  //   console.log("PDA balance:", pdaBalance);
  //   if (pdaBalance < 0) {
  //     console.log("There is no amount in treasury account to claim");
  //     return;
  //   }

  //   const claimAmount = new anchor.BN(0.0001 * LAMPORTS_PER_SOL)  // set the amount you want to claim from the treasury
  //   try {
  //     const tx = await program.methods
  //     .claimFunds(claimAmount)
  //     .accounts({
  //       treasury: treasuryPda,
  //       authority: provider.wallet.publicKey , // Must be admin wallet
  //       userUsdtWallet: userUsdtWallet,
  //       treasuryUsdtWallet: treasuryUsdtWallet,
  //       usdt: usdt,
  //       tokenProgram: TOKEN_PROGRAM_ID,
  //       associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
  //     })
  //     .rpc();
  //   //.transaction();

  //   await connection.confirmTransaction(tx, 'confirmed').catch((e) => { console.log("Error tx:", e)})
  //   } catch(err) {
  //     console.log("ERROR:", err)
  //   }

  //   let treauryAccount = await program.account.treasury.fetch(treasuryPda);
  
  //   console.log("Treasury Account Details:", treauryAccount)

  //   let pdaBalanceAfter = await connection.getBalance(treasuryPda);

  //   console.log("PDA balance after tx:", pdaBalanceAfter);

  //  })



  // it("Initalize nft collection", async () => {
  //   console.log("Collection PDA:", collectionPDA)
  //   //derive the master edition pda
  //   let collectionMasterEditionPDA = findMasterEditionPda(umi, {
  //     mint: publicKey(collectionPDA),
  //   })[0];

  //   console.log("Collection master edition pda:", collectionMasterEditionPDA)

  //   // derive the metadata account
  //   let collectionMetadataPDA = findMetadataPda(umi, {
  //     mint: publicKey(collectionPDA),
  //   })[0];

  //   console.log("Collection metada pda:", collectionMetadataPDA)


  //   const collectionTokenAccount = await getAssociatedTokenAddress(
  //     collectionPDA,
  //     wallet.publicKey
  //   );

  //     try {
  //       const modifyComputeUnits =
  //         anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({
  //           units: 300_000,
  //         });

  //       const tx = await program.methods
  //         .createCollectionNft(
  //           testCollectionMetadata.uri,
  //           testCollectionMetadata.name,
  //           testCollectionMetadata.symbol
  //         )
  //         .accounts({
  //           authority: wallet.publicKey,
  //           collectionMint: collectionPDA,
  //           metadataAccount: collectionMetadataPDA,
  //           masterEdition: collectionMasterEditionPDA,
  //           tokenAccount: collectionTokenAccount,
  //           tokenMetadataProgram: METADATA_PROGRAM_ID,
  //         })
  //         .transaction();

  //         let { lastValidBlockHeight, blockhash } = await connection.getLatestBlockhash('finalized');

  //         const transferTransaction = new anchor.web3.Transaction().add(
  //           modifyComputeUnits,
  //           tx
  //         );

  //         transferTransaction.recentBlockhash = blockhash;
  //         transferTransaction.feePayer = provider.wallet.publicKey;
  //         let txFee = await transferTransaction.getEstimatedFee(provider.connection);

  //         const walletBalance = await provider.connection.getBalance(provider.wallet.publicKey);

  //         if(walletBalance < txFee) {
  //             throw new Error(`Insufficient balance for transacation. Required: ${txFee}, Available User wallet balance: ${walletBalance}`);
  //         }

  //         const x = await provider.sendAndConfirm(transferTransaction);

  //         console.log("confrim tx:", x);

  //     } catch(err) {
  //       console.log("ERROR:", err)
  //     }
    
  // });

  it("Buy nft in collection", async () => {
    let treasury = await program.account.treasury.fetch(treasuryPda);
    let acccInfo = await provider.connection.getAccountInfo(treasuryPda);
    let collectionAccInfo = await provider.connection.getAccountInfo(collectionPDA);

    if(!acccInfo) {
      console.log("Treasury acct is not initialized, init Treasury");
      return;
    }

    if(!collectionAccInfo) {
      console.log("Collection NFT is not Generated yet");
      return;
    }

    console.log("Treasury acc:", acccInfo);
    console.log("Contract count before tx:", treasury.barkBallerBundleCount)

    console.log("collection pda:", collectionPDA.toBase58())


    const treasuryUsdtWallet = await getAssociatedTokenAddress(
          usdt,
          treasuryPda,
          true
        );

    let referrer = new PublicKey("CgmEYDERgUvvz6npz1kUCPEUcsZxpmiitEHNSibQBA5Z");
    const [referrerUser] = PublicKey.findProgramAddressSync(
      [Buffer.from("referral"), referrer.toBuffer()],
      program.programId
    ) 

    let userUsdtWallet: anchor.web3.PublicKey | null = null;
    let referrerUsdtWallet: anchor.web3.PublicKey | null = null;
    let USDT = true; // set this as required

    if (USDT) {
      console.log("USDT INSIDE CHECK")
      userUsdtWallet = await getAssociatedTokenAddress(
        usdt,
        wallet.publicKey
      );

      if(referrer) {
        referrerUsdtWallet = await getAssociatedTokenAddress(
          usdt,
          referrer
        )
      }
    }

    console.log("WHITELIST MINT SETTINGS:", treasury.whitelistMintSettings);
    let nomaimaiTokenAccountBalance: number = 0;
    let ridiculousDragonTokenAccountBalance: number = 0;
    let nominaiRidiculousTokenAccountBalance: number = 0;
    let nomaimaiTokenAccount: anchor.web3.PublicKey; 
    let ridiculousDragonTokenAccount: anchor.web3.PublicKey; 
    let nominaiRidiculousTokenAccount: anchor.web3.PublicKey;

    if (treasury.whitelistMintSettings) {
      nomaimaiTokenAccount = await getAssociatedTokenAddress(
        treasury.whitelistMintSettings.nomaimaiMint,
        wallet.publicKey
      );

      let nomaimaiTokenAccountInfo = await provider.connection.getAccountInfo(nomaimaiTokenAccount);
      
      if(nomaimaiTokenAccountInfo) {
        const balance = await provider.connection.getTokenAccountBalance(nomaimaiTokenAccount);
        nomaimaiTokenAccountBalance = balance.value.uiAmount ?? 0;
      }
      
      let ridiculousDragonTokenAccount = await getAssociatedTokenAddress(
        treasury.whitelistMintSettings.ridiculousDragonMint,
        wallet.publicKey
      );
      let ridiculousDragonTokenAccountInfo = await provider.connection.getAccountInfo(ridiculousDragonTokenAccount);
      if(ridiculousDragonTokenAccountInfo) {
        const balance = await provider.connection.getTokenAccountBalance(ridiculousDragonTokenAccount);
        ridiculousDragonTokenAccountBalance = balance.value.uiAmount ?? 0;
      }

      let nominaiRidiculousTokenAccount = await getAssociatedTokenAddress(
        treasury.whitelistMintSettings.nomaimaiRidiculousMint,
        wallet.publicKey
      );
      let nominaiRidiculousTokenAccountInfo = await provider.connection.getAccountInfo(nominaiRidiculousTokenAccount);
      
      if(nominaiRidiculousTokenAccountInfo) {
        const balance = await provider.connection.getTokenAccountBalance(nominaiRidiculousTokenAccount);
        nominaiRidiculousTokenAccountBalance = balance.value.uiAmount ?? 0;
      }
    }


    let amount = 3;

    let categoryItemsAvailable = treasury.barkBallerBundleCount;  // Get the relevant category count  : treasury.furRealDealCount, treasury.purrmiumPackCount
    
    if (amount > categoryItemsAvailable) {
      throw new Error(`${amount} Nfts of this category are not available, Available Items Availabele: ${categoryItemsAvailable}, Try with less amount or another category`);
    }

    try {

      const modifyComputeUnits =
      anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({
        units: 400_000,
      });

    let txs: anchor.web3.Transaction[] = [];
    let mintKeys: anchor.web3.Keypair[] = [];
    // const batchSize = 20; // Adjust based on your transaction size limit
        
    // for (let i = 0; i < ixData.length; i += batchSize) {
    //   const batch = ixData.slice(i, i + batchSize);
  
    for (let i = 0; i < amount; i++) {
      const mint = Keypair.generate();

      mintKeys.push(mint);

      console.log("mint key:", mint.publicKey.toBase58())

      const metadataPDA = findMetadataPda(umi, {
        mint: publicKey(mint.publicKey),
      })[0];

      const masterEditionPDA = findMasterEditionPda(umi, {
        mint: publicKey(mint.publicKey),
      })[0];

      const tokenAccount = await getAssociatedTokenAddress(
        mint.publicKey,
        wallet.publicKey
      );

      let collectionMasterEditionPDA = findMasterEditionPda(umi, {
        mint: publicKey(collectionPDA),
      })[0];

      // derive the metadata account
      let collectionMetadataPDA = findMetadataPda(umi, {
        mint: publicKey(collectionPDA),
      })[0];

      const remainingAccounts: Array<any> = [
        { pubkey: new PublicKey(metadataPDA), isWritable: true, isSigner: false },
        { pubkey: new PublicKey(masterEditionPDA), isWritable: true, isSigner: false },
        { pubkey: new PublicKey(collectionMetadataPDA), isWritable: true, isSigner: false },
        { pubkey: new PublicKey(collectionMasterEditionPDA), isWritable: true, isSigner: false },
      ];

        if (nomaimaiTokenAccountBalance > 0) {
          remainingAccounts.push({
            pubkey: nomaimaiTokenAccount,
            isWritable: true,
            isSigner: false,
          });
        }
        if (ridiculousDragonTokenAccountBalance > 0) {
          remainingAccounts.push({
            pubkey: ridiculousDragonTokenAccount,
            isWritable: true,
            isSigner: false,
          });
        }
        if (nominaiRidiculousTokenAccountBalance > 0) {
          remainingAccounts.push({
            pubkey: nominaiRidiculousTokenAccount,
            isWritable: true,
            isSigner: false,
          });
        }

      console.log("REMAINING ACCOUNTS:", remainingAccounts);

      let  tx = await program.methods
        .buyCollectionNft( 
          { barkBallerBundle: {} }          // category 
        )
        .accounts({
          user: wallet.publicKey,
          treasury: treasuryPda,
          collectionMint: collectionPDA,
          // collectionMetadataAccount: collectionMetadataPDA,
          // collectionMasterEdition: collectionMasterEditionPDA,
          nftMint: mint.publicKey,
          // metadataAccount: metadataPDA,
          // masterEdition: masterEditionPDA,
          feedAggregator: solUsedSwitchboardFeed,
          userUsdtWallet: userUsdtWallet,
          treasuryUsdtWallet: treasuryUsdtWallet,
          usdt:usdt,
          referrerUser: referrerUser,
          referrer: referrer,
          referrerUsdtWallet: referrerUsdtWallet,
          tokenAccount: tokenAccount,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          tokenMetadataProgram: METADATA_PROGRAM_ID,
        })
        .signers([mint])
        .remainingAccounts(remainingAccounts)
        .transaction()
        // .transaction();

        // txBuilder.remainingAccounts(remainingAccounts);

        // const tx = await txBuilder.transaction();

        console.log("REMAINIG ACCOUNTS:", remainingAccounts)

        const transferTransaction = new anchor.web3.Transaction().add(
            modifyComputeUnits,
            tx
          );

        //console.log("Trnasaction TX:", transferTransaction.signature)
        console.log("next")
        txs.push(transferTransaction);
    }
      

      if(wallet.signAllTransactions) {
        const block = await provider.connection.getLatestBlockhash();
        console.log("BLOCK HASH:", block.blockhash)
        let totalFees = 0;
        // txs.forEach(ta => {
        //   ta.recentBlockhash = block.blockhash;
        //   ta.feePayer = wallet.publicKey;
        // })

        const fees = await Promise.all(txs.map(async (tx, index) => {
          tx.recentBlockhash = block.blockhash;
          tx.feePayer = wallet.publicKey;
          if (index >= mintKeys.length) {
            throw new Error("Mint keys array does not have enough elements for each transaction, check your code");
          }
          console.log("MINT KEYS:", mintKeys)
          console.log("INDEX:", index)
          tx.sign(mintKeys[index]);
          return await tx.getEstimatedFee(provider.connection);
        }));

        totalFees = fees.reduce((sum, fee) => sum + fee, 0);
        const walletBalance = await provider.connection.getBalance(provider.wallet.publicKey);

        if(walletBalance < totalFees) {
            throw new Error(`Insufficient balance. Required Balance to buy NFTs: ${totalFees}, Available User wallet balance: ${walletBalance},`);
        }

        const signedTransactions = await wallet.signAllTransactions(txs);  
        console.log("User has signed "+signedTransactions.length+" transactions");

        console.log("Mint Keys Length:", mintKeys.length)
        console.log("Transactions Length:", txs.length)
        
        for(const ta of signedTransactions) {
          const txid  = await provider.connection.sendRawTransaction(
            ta.serialize(),
            {
              skipPreflight: false,
            }
          );

          console.log("TRANSACTION SUCSESSFUL",txid)
          // let treasury = await program.account.treasury.fetch(treasuryPda);
          // console.log("Bark Baller Bundle Collection Items Available", treasury.barkBallerBundleCount)

        }
      }
    //}

      let treasury = await program.account.treasury.fetch(treasuryPda);
      console.log("Bark Baller Bundle Collection Items Available", treasury.barkBallerBundleCount)
      console.log("Fur Real Deal Collection Items Available", treasury.furRealDealCount)
      console.log("Purrmium Pack Collection Items Available", treasury.purrmiumPackCount)

    } catch(Err) {
      console.log("ERROR:", Err);
    }
  
  });


  // Define types from your IDL
  type CategoryType = anchor.IdlTypes<CollNft>["Category"];
  type CryptoType = anchor.IdlTypes<CollNft>["CryptoMon"];

  const mapCategoryName = async(collectionName: string): Promise<CategoryType> => {
    console.log("Collection name inside map:", collectionName)
    switch (collectionName) {
        case "BarkBallerBundle":
          return  { barkBallerBundle: {} };        
        case "PurrmiumPack":
            return { purrmiumPack: {} };  
        case "FurRealDeal":
            return { furRealDeal: {} };
        default:
            throw new Error(`Unsupported eventName: ${collectionName}`);
    }
  }

  const mapCryptoMonName = async(cryptoMonName: string): Promise<CryptoType> => {
    console.log("Collection name inside map:", cryptoMonName)
    switch (cryptoMonName) {
        case "Nomaimai":
            return { nomaimai: {} };
        case "RidiculousDragon":
            return { ridiculousDragon: {} };
        case "NomimaiRidiculousDragon":
            return { nomimaiRidiculousDragon: {} };
        default:
            throw new Error(`Unsupported eventName: ${cryptoMonName}`);
    }
  }


  function toU64Bytes(value) {
    const buffer = Buffer.alloc(8); // Allocate a buffer of 8 bytes
    buffer.writeBigUInt64LE(BigInt(value), 0); // Write the u64 value to the buffer in little-endian format
    return buffer;
}




});

