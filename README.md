# 🌟 Solana NFT Collection Project

Welcome to the **Solana NFT Collection Project**! This project, built with the Solana Anchor framework, enables users to create, buy, and manage NFTs with advanced features like batch minting, referral programs, and discounts for whitelisted token holders.

## 🚀 Features

### 1. **Create NFT Collection**
   - Easily set up an NFT collection on the Solana blockchain.
   - Customize collection metadata such as name, symbol, and description.

### 2. **Buy NFTs**
   - Users can purchase NFTs directly from the collection.
   - Secure transactions and seamless ownership transfer.

### 3. **Batch Minting**
   - Mint multiple NFTs in a single transaction.
   - Efficient and cost-effective for large-scale NFT drops.

### 4. **Referral Program**
   - Encourage community growth with a referral system.
   - Reward users for bringing new collectors to the platform.

### 5. **Discounts for Whitelisted Token Holders**
   - Offer special discounts to holders of specific tokens.
   - Reward your loyal community with exclusive benefits.

## 🛠️ Installation

### Prerequisites

- [Node.js](https://nodejs.org/) 
- [Rust](https://www.rust-lang.org/tools/install)
- [Solana CLI](https://solana.com/docs/intro/installation)
- [Anchor CLI](https://www.anchor-lang.com/docs/installation)  (recommended version 0.28)
  
  **NOTE**: Install **avm** version manager and then install the prefered anchor version.
### Clone the Repository

```bash
git clone https://github.com/LoboVH/Solana-NFT_collection.git
cd solana-nft-collection
```

## 🛠️ Installation

### Anchor

```anchor
anchor keys list
```
This will give the fresh **ProgramId**, update this in **Anchor.toml** and **lib.rs** in **declare_id!("<PROGRAM_ID>")**

then run,
```anchor
anchor build
```
This will generate **/target** folder in the root withn **idl** and **type** folder.

next step is to run **anchor test** , but before that make sure you have enough **SOL** by running,
```solana
solana balance
```
You could airdrop more using **CLI** command or get some SOL from [here](https://faucet.solana.com/)

Once you have enough **SOL** , go to test folder and customize the tests as per requirement and run:
```anchor
anchor test
```

This will deploy the program and run the tests.
