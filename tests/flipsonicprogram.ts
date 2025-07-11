import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Flipsonicprogram } from "../target/types/flipsonicprogram";
import { ASSOCIATED_TOKEN_PROGRAM_ID, createAccount, createAssociatedTokenAccountInstruction, createInitializeMint2Instruction, createMint, createMintToCheckedInstruction, getAssociatedTokenAddress, getMinimumBalanceForRentExemptMint, getOrCreateAssociatedTokenAccount, MINT_SIZE, mintTo, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { Commitment, Connection, Keypair, PublicKey, sendAndConfirmTransaction, SystemProgram, Transaction } from "@solana/web3.js";
import { assert } from "chai";


describe("flipsonicprogram", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Flipsonicprogram as Program<Flipsonicprogram>;

  const commitment = 'processed';

  const connection = new Connection('https://api.testnet.v1.sonic.game', {
    commitment,
    wsEndpoint: 'wss://api.testnet.v1.sonic.game'
  });

  const signer = anchor.web3.Keypair.fromSecretKey(
    new Uint8Array(JSON.parse(require('fs').readFileSync('./7e8EC4BuPiEnajQ6V86Y8pWs4TJuiz5xCFxD9uXYMQaL.json', 'utf8')))
  );

  // Sonic Tokens
  const tokenA = new PublicKey("3jRK5ys7vMtMWxYiSTwxgKB9QSDQukNb8dNpDadyqsJo")
  const tokenB = new PublicKey("4mhwo7mxPx3u1TsZRhyj4dFBxvgXsawG8MxcDGt2xpq3")

  // for local solana testing
  // const tokenA = new PublicKey("BtvVH5ipBmdTNj7iQhHh9mFnBE4qCX8mwS1Wx7NSw1ug")
  // const tokenB = new PublicKey("2v1RXk976nnXwt9wgLatyALvYYxvZUF36yKDG3MCH7Hz")

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


  it("Create Token and Mint", async () => {
    const commitment = 'processed';

    const connection = new Connection('https://api.testnet.v1.sonic.game', {
      commitment,
      wsEndpoint: 'wss://api.testnet.v1.sonic.game'
    });

    const tx = new Transaction();

    const tokenMintAccountKeypair = Keypair.generate();
    console.log("token mint account: ", tokenMintAccountKeypair.publicKey.toBase58());

    const decimal = 6;

    // Calculate the rent-exempt minimum balance for the mint account
    const lamports = await getMinimumBalanceForRentExemptMint(connection);

    // Add instruction to create the mint account
    tx.add(
      SystemProgram.createAccount({
        fromPubkey: signer.publicKey,
        newAccountPubkey: tokenMintAccountKeypair.publicKey,
        lamports,
        space: MINT_SIZE,
        programId: TOKEN_PROGRAM_ID,
      })
    );

    // Add instruction to initialize the mint
    tx.add(
      createInitializeMint2Instruction(
        tokenMintAccountKeypair.publicKey,
        decimal,
        signer.publicKey,
        null
      )
    );

    // Get the associated token account address
    const senderTokenAccount = await getAssociatedTokenAddress(
      tokenMintAccountKeypair.publicKey,
      signer.publicKey
    );

    // Check if the associated token account exists
    const accountInfo = await connection.getAccountInfo(senderTokenAccount);
    if (!accountInfo) {
      // Add instruction to create the associated token account if it doesn't exist
      tx.add(
        createAssociatedTokenAccountInstruction(
          signer.publicKey,
          senderTokenAccount,
          signer.publicKey,
          tokenMintAccountKeypair.publicKey,
          TOKEN_PROGRAM_ID,
          ASSOCIATED_TOKEN_PROGRAM_ID
        )
      );
    }

    // Add instruction to mint tokens
    tx.add(
      createMintToCheckedInstruction(
        tokenMintAccountKeypair.publicKey,
        senderTokenAccount,
        signer.publicKey,
        1000000000000 * 10 ** decimal, // Correct amount calculation
        decimal,
        [],
        TOKEN_PROGRAM_ID,
      )
    );

    // Set the fee payer and recent blockhash
    tx.feePayer = signer.publicKey;
    const { blockhash } = await connection.getLatestBlockhash();
    tx.recentBlockhash = blockhash;

    // Sign and send the transaction
    try {
      const txHash = await sendAndConfirmTransaction(connection, tx, [signer, tokenMintAccountKeypair]);
      console.log("tx hash: ", txHash);
    } catch (error) {
      console.error("Error sending transaction: ", error);

      // Log detailed error information
      if (error.logs) {
        console.error("Transaction logs: ", error.logs);
      }
    }

  });

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

  // it("add Liquidity to the pool", async () => {
  //   // Fetch the pool account
  //   const fetchedAccount = await program.account.pool.fetch(poolAccount);

  //   const tokenA_amount = new anchor.BN(10000 * 10e6);
  //   const tokenB_amount = new anchor.BN(1000000 * 10e6);

  //   // get or Create user's associated token account for user Liquidity Token
  //   const userLiquidityTokenAccount = await getOrCreateAssociatedTokenAccount(
  //     connection,
  //     signer, // Fee payer
  //     fetchedAccount.liquidityTokenMint,
  //     signer.publicKey
  //   );

  //   // get or Create user's associated token account for token A
  //   const userTokenA = await getOrCreateAssociatedTokenAccount(
  //     connection,
  //     signer, // Fee payer
  //     tokenA,
  //     signer.publicKey,
  //   );

  //   // get or Create user's associated token account for token B
  //   const userTokenB = await getOrCreateAssociatedTokenAccount(
  //     connection,
  //     signer, // Fee payer
  //     tokenB,
  //     signer.publicKey,
  //   );

  //   // get or Create pool's associated token account for token A
  //   const poolTokenA = await getOrCreateAssociatedTokenAccount(
  //     connection,
  //     signer, // Fee payer
  //     tokenA,
  //     poolAccount,
  //     true
  //   );

  //   // get or Create pool's associated token account for token B
  //   const poolTokenB = await getOrCreateAssociatedTokenAccount(
  //     connection,
  //     signer, // Fee payer
  //     tokenB,
  //     poolAccount,
  //     true
  //   );


  //   const accountData = {
  //     liquidityTokenMint,
  //     pool: poolAccount,
  //     userLiquidityTokenAccount: userLiquidityTokenAccount.address,
  //     user: signer.publicKey,
  //     userTokenA: userTokenA.address,
  //     userTokenB: userTokenB.address,
  //     poolTokenA: poolTokenA.address,
  //     poolTokenB: poolTokenB.address,
  //     userLiquidityToken: userLiquidityTokenAccount.address,
  //     tokenProgram: TOKEN_PROGRAM_ID,
  //     systemProgram: anchor.web3.SystemProgram.programId,
  //     rent: anchor.web3.SYSVAR_RENT_PUBKEY,
  //     associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
  //   }

  //   const signature = await program.methods.addLiquidity(tokenA_amount, tokenB_amount, poolBump)
  //     .accounts(accountData)
  //     .signers([])
  //     .rpc();

  //   console.log("Signature", signature);

  // });

  // it("Withdraw Liquidity from the pool", async () => {
  //   // Fetch the pool account
  //   const fetchedAccount = await program.account.pool.fetch(poolAccount);

  //   const liquidityTokens = new anchor.BN(10 * 1e9);

  //   // get or Create user's associated token account for user Liquidity Token
  //   const userLiquidityTokenAccount = await getOrCreateAssociatedTokenAccount(
  //     connection,
  //     signer, // Fee payer
  //     fetchedAccount.liquidityTokenMint,
  //     signer.publicKey
  //   );

  //   // get or Create user's associated token account for token A
  //   const userTokenA = await getOrCreateAssociatedTokenAccount(
  //     connection,
  //     signer, // Fee payer
  //     tokenA,
  //     signer.publicKey,
  //   );

  //   // get or Create user's associated token account for token B
  //   const userTokenB = await getOrCreateAssociatedTokenAccount(
  //     connection,
  //     signer, // Fee payer
  //     tokenB,
  //     signer.publicKey,
  //   );

  //   // get or Create pool's associated token account for token A
  //   const poolTokenA = await getOrCreateAssociatedTokenAccount(
  //     connection,
  //     signer, // Fee payer
  //     tokenA,
  //     poolAccount,
  //     true
  //   );

  //   // get or Create pool's associated token account for token B
  //   const poolTokenB = await getOrCreateAssociatedTokenAccount(
  //     connection,
  //     signer, // Fee payer
  //     tokenB,
  //     poolAccount,
  //     true
  //   );


  //   const accountData = {
  //     liquidityTokenMint,
  //     pool: poolAccount,
  //     userLiquidityTokenAccount: userLiquidityTokenAccount.address,
  //     user: signer.publicKey,
  //     userTokenA: userTokenA.address,
  //     userTokenB: userTokenB.address,
  //     poolTokenA: poolTokenA.address,
  //     poolTokenB: poolTokenB.address,
  //     tokenProgram: TOKEN_PROGRAM_ID,
  //   }

  //   const signature = await program.methods.removeLiquidity(liquidityTokens, poolBump)
  //     .accounts(accountData)
  //     .signers([])
  //     .rpc();

  //   console.log("Signature", signature);

  // });

  // it("Swap on pool", async () => {

  //   // Fetch the pool account
  //   const fetchedAccount = await program.account.pool.fetch(poolAccount);

  //   // Perform a swap
  //   const slippageTolerance = 0.005; // 0.5%
  //   const amount = 10 * 10e6
  //   const amountIn = new anchor.BN(amount);
  //   const reserveA = fetchedAccount.reserveA.toNumber();
  //   const reserveB = fetchedAccount.reserveB.toNumber();

  //   // Calculate expected output
  //   const expectedAmountOut = (reserveB * amount) / (reserveA + amount);

  //   // Apply slippage tolerance
  //   const minAmountOut = new anchor.BN(Math.floor(expectedAmountOut * (1 - slippageTolerance)));

  //   console.log(9 * 10e6)


  //   // get or Create user's associated token account for token A
  //   const userTokenA = await getOrCreateAssociatedTokenAccount(
  //     connection,
  //     signer, // Fee payer
  //     tokenA,
  //     signer.publicKey,
  //   );

  //   // get or Create user's associated token account for token B
  //   const userTokenB = await getOrCreateAssociatedTokenAccount(
  //     connection,
  //     signer, // Fee payer
  //     tokenB,
  //     signer.publicKey,
  //   );

  //   // get or Create pool's associated token account for token A
  //   const poolTokenA = await getOrCreateAssociatedTokenAccount(
  //     connection,
  //     signer, // Fee payer
  //     tokenA,
  //     poolAccount,
  //     true
  //   );

  //   // get or Create pool's associated token account for token B
  //   const poolTokenB = await getOrCreateAssociatedTokenAccount(
  //     connection,
  //     signer, // Fee payer
  //     tokenB,
  //     poolAccount,
  //     true
  //   );

  //   const accountData = {
  //     pool: poolAccount,
  //     user: signer.publicKey,
  //     userTokenIn: userTokenA.address,
  //     userTokenOut: userTokenB.address,
  //     poolTokenIn: poolTokenA.address,
  //     poolTokenOut: poolTokenB.address,
  //     tokenProgram: TOKEN_PROGRAM_ID,
  //   }

  //   const signature = await program.methods.swap(amountIn, minAmountOut, poolBump)
  //     .accounts(accountData)
  //     .signers([])
  //     .rpc();

  //   console.log("Signature", signature);

  // });

});
