import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Amm } from "../target/types/amm";
import {
  createMint,
  getAssociatedTokenAddress,
  createAssociatedTokenAccount,
  mintTo,
  getAccount,
} from "@solana/spl-token";
import { assert } from "chai";
import { BN } from "bn.js";

describe("withdraw_liquidity", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Amm as Program<Amm>;

  let admin = provider.wallet;
  let user = anchor.web3.Keypair.generate();
  let mintA: anchor.web3.PublicKey;
  let mintB: anchor.web3.PublicKey;
  let lpMint: anchor.web3.PublicKey;
  let userTokenA: anchor.web3.PublicKey;
  let userTokenB: anchor.web3.PublicKey;
  let userLPToken: anchor.web3.PublicKey;

  let amm: anchor.web3.PublicKey;
  let pool: anchor.web3.PublicKey;

  it("Sets up liquidity pool and withdraws", async () => {
    const connection = provider.connection;

    // Airdrop SOL to user
    await connection.confirmTransaction(
      await connection.requestAirdrop(user.publicKey, anchor.web3.LAMPORTS_PER_SOL)
    );

    // Create mint A and B
    mintA = await createMint(connection, admin.payer, admin.publicKey, null, 6);
    mintB = await createMint(connection, admin.payer, admin.publicKey, null, 6);

    // Create user's associated token accounts
    userTokenA = await getAssociatedTokenAddress(mintA, user.publicKey);
    userTokenB = await getAssociatedTokenAddress(mintB, user.publicKey);

    await createAssociatedTokenAccount(connection, user, mintA, user.publicKey);
    await createAssociatedTokenAccount(connection, user, mintB, user.publicKey);

    // Mint token A and B to user
    await mintTo(connection, admin.payer, mintA, userTokenA, admin.publicKey, 1_000_000_000);
    await mintTo(connection, admin.payer, mintB, userTokenB, admin.publicKey, 1_000_000_000);

    // Create AMM & Pool (simplified; you need to call your create_amm & create_pool instructions)
    // Assume amm and pool accounts are derived or returned

    // Deposit liquidity (should call your deposit_liquidity method here)

    // Get LP token address
    userLPToken = await getAssociatedTokenAddress(lpMint, user.publicKey);

    // Assume user deposited and received LP tokens

    // Call withdraw_liquidity
    await program.methods
      .withdrawLiquidity(new BN(100_000_000)) // 100 LP tokens
      .accounts({
        pool,
        poolAuthority: anchor.web3.PublicKey.findProgramAddressSync(
          [/* same seeds as in program */],
          program.programId
        )[0],
        mintA,
        mintB,
        mintLiquidity: lpMint,
        poolAccountA: /* your pool ATA A */,
        poolAccountB: /* your pool ATA B */,
        depositorAccountA: userTokenA,
        depositorAccountB: userTokenB,
        depositorAccountLiquidity: userLPToken,
        depositor: user.publicKey,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        associatedTokenProgram: anchor.utils.token.ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    const userA = await getAccount(connection, userTokenA);
    const userB = await getAccount(connection, userTokenB);

    console.log("Withdrawn A:", Number(userA.amount));
    console.log("Withdrawn B:", Number(userB.amount));

    assert(Number(userA.amount) > 0);
    assert(Number(userB.amount) > 0);
  });
});
