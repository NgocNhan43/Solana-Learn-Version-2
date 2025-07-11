// Assignment 2 - Minting Fungible Token & NFT on Solana

import {
  Connection,
  Keypair,
  PublicKey,
  SystemProgram,
  Transaction,
  sendAndConfirmTransaction,
  clusterApiUrl,
} from "@solana/web3.js";
import {
  MINT_SIZE,
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAssociatedTokenAddress,
  createInitializeMint2Instruction,
  createMintToInstruction,
  createAssociatedTokenAccountInstruction,
} from "@solana/spl-token";

import {
  PROGRAM_ID as METADATA_PROGRAM_ID,
  createCreateMetadataAccountV3Instruction,
} from "@metaplex-foundation/mpl-token-metadata";

import { payer } from "../lib/vars"; // Replace with your payer keypair
import { Buffer } from "buffer";

(async () => {
  const connection = new Connection(clusterApiUrl("devnet"), "confirmed");
  const receiver = new PublicKey("63EEC9FfGyksm7PkVC6z8uAmqozbQcTzbkWJNsgqjkFs");

  // ========== 1. Mint Fungible Token ==========
  const fungibleMint = Keypair.generate();
  const decimals = 6;
  const rent = await connection.getMinimumBalanceForRentExemption(MINT_SIZE);

  const createFungibleMintIx = SystemProgram.createAccount({
    fromPubkey: payer.publicKey,
    newAccountPubkey: fungibleMint.publicKey,
    space: MINT_SIZE,
    lamports: rent,
    programId: TOKEN_PROGRAM_ID,
  });

  const initFungibleMintIx = createInitializeMint2Instruction(
    fungibleMint.publicKey,
    decimals,
    payer.publicKey,
    payer.publicKey
  );

  // Tính địa chỉ ATA (Associated Token Account) cho payer & receiver
  const payerFungibleATA = await getAssociatedTokenAddress(
    fungibleMint.publicKey,
    payer.publicKey
  );

  const receiverFungibleATA = await getAssociatedTokenAddress(
    fungibleMint.publicKey,
    receiver
  );

  // Tạo ATA nếu chưa tồn tại
  const createPayerATAIx = createAssociatedTokenAccountInstruction(
    payer.publicKey,
    payerFungibleATA,
    payer.publicKey,
    fungibleMint.publicKey
  );

  const createReceiverATAIx = createAssociatedTokenAccountInstruction(
    payer.publicKey,
    receiverFungibleATA,
    receiver,
    fungibleMint.publicKey
  );

  // Mint token cho cả payer (100) và receiver (10)
  const mintToPayerIx = createMintToInstruction(
    fungibleMint.publicKey,
    payerFungibleATA,
    payer.publicKey,
    100 * 10 ** decimals
  );

  const mintToReceiverIx = createMintToInstruction(
    fungibleMint.publicKey,
    receiverFungibleATA,
    payer.publicKey,
    10 * 10 ** decimals
  );

  // ========== 2. Mint NFT ==========
  const nftMint = Keypair.generate();

  const createNftMintIx = SystemProgram.createAccount({
    fromPubkey: payer.publicKey,
    newAccountPubkey: nftMint.publicKey,
    space: MINT_SIZE,
    lamports: rent,
    programId: TOKEN_PROGRAM_ID,
  });

  const initNftMintIx = createInitializeMint2Instruction(
    nftMint.publicKey,
    0, // NFT: không có thập phân
    payer.publicKey,
    payer.publicKey
  );

  const payerNftATA = await getAssociatedTokenAddress(
    nftMint.publicKey,
    payer.publicKey
  );

  const createNftATAIx = createAssociatedTokenAccountInstruction(
    payer.publicKey,
    payerNftATA,
    payer.publicKey,
    nftMint.publicKey
  );

  const mintNftIx = createMintToInstruction(
    nftMint.publicKey,
    payerNftATA,
    payer.publicKey,
    1 // Mint đúng 1 NFT
  );

  // Metadata PDA (chứa name, symbol, URI, v.v.)
  const [nftMetadataPDA] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("metadata"),
      METADATA_PROGRAM_ID.toBuffer(),
      nftMint.publicKey.toBuffer(),
    ],
    METADATA_PROGRAM_ID
  );

  const createMetadataIx = createCreateMetadataAccountV3Instruction(
    {
      metadata: nftMetadataPDA,
      mint: nftMint.publicKey,
      mintAuthority: payer.publicKey,
      payer: payer.publicKey,
      updateAuthority: payer.publicKey,
    },
    {
      createMetadataAccountArgsV3: {
        data: {
          name: "Bootcamp NFT 2024",
          symbol: "BCNFT",
          uri: "https://raw.githubusercontent.com/trankhacvy/solana-bootcamp-autumn-2024/main/assets/nft-metadata.json",
          sellerFeeBasisPoints: 1000, // 10% royalty
          creators: [
            {
              address: payer.publicKey,
              verified: true,
              share: 100,
            },
          ],
          collection: null,
          uses: null,
        },
        isMutable: true,
        collectionDetails: null,
      },
    }
  );

  // ========== 3. Gửi Giao Dịch ==========
  const transaction = new Transaction().add(
    createFungibleMintIx,
    initFungibleMintIx,
    createPayerATAIx,
    createReceiverATAIx,
    mintToPayerIx,
    mintToReceiverIx,
    createNftMintIx,
    initNftMintIx,
    createNftATAIx,
    mintNftIx,
    createMetadataIx
  );

  const sig = await sendAndConfirmTransaction(connection, transaction, [
    payer,
    fungibleMint,
    
    nftMint,
  ]);

  console.log("✅ Transaction Signature:", sig);
  console.log("🔗 Explorer:", `https://explorer.solana.com/tx/${sig}?cluster=devnet`);
})();
