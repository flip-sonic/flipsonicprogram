import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Flipsonicprogram } from "../target/types/flipsonicprogram";
import { ASSOCIATED_TOKEN_PROGRAM_ID, createAccount, createMint, getOrCreateAssociatedTokenAccount, mintTo, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { PublicKey } from "@solana/web3.js";
import { assert } from "chai";


describe("flipsonicprogram", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Flipsonicprogram as Program<Flipsonicprogram>;

  const signer = anchor.web3.Keypair.fromSecretKey(
    new Uint8Array(JSON.parse(require('fs').readFileSync('./7e8EC4BuPiEnajQ6V86Y8pWs4TJuiz5xCFxD9uXYMQaL.json', 'utf8')))
  );
  const tokenA = new PublicKey("BtvVH5ipBmdTNj7iQhHh9mFnBE4qCX8mwS1Wx7NSw1ug")
  const tokenB = new PublicKey("2v1RXk976nnXwt9wgLatyALvYYxvZUF36yKDG3MCH7Hz")

  // pool Account
  const [poolAccount, poolBump] = PublicKey.findProgramAddressSync(
    [Buffer.from("pool"), tokenA.toBuffer(), tokenB.toBuffer(), signer.publicKey.toBuffer()],
    program.programId
  );
  // Master Contract
  const [liquidityTokenMint] = PublicKey.findProgramAddressSync(
    [Buffer.from("pool"), poolAccount.toBuffer()],
    program.programId
  );


  // it("Create Token and Mint", async () => {

  //   const tokenMint = await createMint(
  //     provider.connection,
  //     signer,
  //     signer.publicKey,
  //     null,
  //     6,
  //   );

  //   // Create token accounts
  //   const userTokenAccount = await createAccount(
  //     provider.connection,
  //     signer,
  //     tokenMint,
  //     signer.publicKey
  //   );

  //   // Mint some tokens to user
  //   const signatureMint = await mintTo(
  //     provider.connection,
  //     signer,
  //     tokenMint,
  //     userTokenAccount,
  //     signer.publicKey,
  //     1000000000000 * 1e6
  //   );
  //   console.log("Minted tokens to user", tokenMint.toBase58());
  // });

  // it("Initializes a pool", async () => {
 
  //   console.log("Pool Account", poolAccount.toBase58());

  //   console.log("Liquidity Token Mint", liquidityTokenMint.toBase58());

  //   const accountData = {
  //     pool: poolAccount,
  //     mintA: tokenA,
  //     mintB: tokenB,
  //     liquidityTokenMint: liquidityTokenMint,
  //     user: signer.publicKey,
  //     systemProgram: anchor.web3.SystemProgram.programId,
  //     token_program: TOKEN_PROGRAM_ID,
  //   }

  //   const fee = 30;
  //   const signature = await program.methods.initializePool(poolBump, fee)
  //     .accounts(accountData)
  //     .signers([])
  //     .rpc();

  //   console.log("Signature", signature);

  //   // Fetch the pool account to verify its state
  //   const fetchedAccount = await program.account.pool.fetch(poolAccount);
  //   console.log("Fetched Pool Account:", fetchedAccount);

  //   // Verify the pool's state
  //   assert.equal(fetchedAccount.mintA.toBase58(), tokenA.toBase58());
  //   assert.equal(fetchedAccount.mintB.toBase58(), tokenB.toBase58());
  //   assert.equal(fetchedAccount.fee, fee);
  //   assert.equal(fetchedAccount.liquidityTokenMint.toBase58(), liquidityTokenMint.toBase58());
  //   assert.equal(fetchedAccount.reserveA.toNumber(), 0);
  //   assert.equal(fetchedAccount.reserveB.toNumber(), 0);
  //   assert.equal(fetchedAccount.totalLiquidity.toNumber(), 0);

  // });


  it("add Liquidity to the pool", async () => {
    // Fetch the pool account
    const fetchedAccount = await program.account.pool.fetch(poolAccount);

    const tokenA_amount = new anchor.BN(10000 * 1e6);
    const tokenB_amount = new anchor.BN(1000000 * 1e6);

    // get or Create user's associated token account for user Liquidity Token
    const userLiquidityTokenAccount = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      signer, // Fee payer
      fetchedAccount.liquidityTokenMint,
      signer.publicKey
    );

    // get or Create user's associated token account for token A
    const userTokenA = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      signer, // Fee payer
      tokenA,
      signer.publicKey,
    );

    // get or Create user's associated token account for token B
    const userTokenB = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      signer, // Fee payer
      tokenB,
      signer.publicKey,
    );

    // get or Create pool's associated token account for token A
    const poolTokenA = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      signer, // Fee payer
      tokenA,
      poolAccount,
      true
    );
    
    // get or Create pool's associated token account for token B
    const poolTokenB = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      signer, // Fee payer
      tokenB,
      poolAccount,
      true
    );


    const accountData ={
      liquidityTokenMint,
      pool: poolAccount,
      userLiquidityTokenAccount: userLiquidityTokenAccount.address,
      user: signer.publicKey,
      userTokenA: userTokenA.address,
      userTokenB: userTokenB.address,
      poolTokenA: poolTokenA.address,
      poolTokenB: poolTokenB.address,
      userLiquidityToken: userLiquidityTokenAccount.address,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: anchor.web3.SystemProgram.programId,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
    }

    const signature = await program.methods.addLiquidity(tokenA_amount, tokenB_amount, poolBump)
      .accounts(accountData)
      .signers([])
      .rpc();

    console.log("Signature", signature);

  });

  it("Withdraw Liquidity to the pool", async () => {
    // Fetch the pool account
    const fetchedAccount = await program.account.pool.fetch(poolAccount);

    const liquidityTokens = new anchor.BN(10 * 1e9);

    // get or Create user's associated token account for user Liquidity Token
    const userLiquidityTokenAccount = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      signer, // Fee payer
      fetchedAccount.liquidityTokenMint,
      signer.publicKey
    );

    // get or Create user's associated token account for token A
    const userTokenA = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      signer, // Fee payer
      tokenA,
      signer.publicKey,
    );

    // get or Create user's associated token account for token B
    const userTokenB = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      signer, // Fee payer
      tokenB,
      signer.publicKey,
    );

    // get or Create pool's associated token account for token A
    const poolTokenA = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      signer, // Fee payer
      tokenA,
      poolAccount,
      true
    );
    
    // get or Create pool's associated token account for token B
    const poolTokenB = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      signer, // Fee payer
      tokenB,
      poolAccount,
      true
    );


    const accountData ={
      liquidityTokenMint,
      pool: poolAccount,
      userLiquidityTokenAccount: userLiquidityTokenAccount.address,
      user: signer.publicKey,
      userTokenA: userTokenA.address,
      userTokenB: userTokenB.address,
      poolTokenA: poolTokenA.address,
      poolTokenB: poolTokenB.address,
      tokenProgram: TOKEN_PROGRAM_ID,
    }

    const signature = await program.methods.removeLiquidity(liquidityTokens, poolBump)
      .accounts(accountData)
      .signers([])
      .rpc();

    console.log("Signature", signature);

  });

});
