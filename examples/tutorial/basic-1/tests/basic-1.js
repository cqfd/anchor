const assert = require("assert");
const anchor = require("@project-serum/anchor");
const spl = require("@solana/spl-token");
const { SystemProgram } = anchor.web3;

describe("basic-1", () => {
  // Use a local provider.
  const provider = anchor.Provider.local();

  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  const program = anchor.workspace.Basic1;


  it("Can do mint stuff", async () => {

    let mint = anchor.web3.Keypair.generate();
    let [mintAuthority, mintAuthorityBump] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from("mint_authority")], program.programId);

    await program.rpc.initMint(new anchor.BN(mintAuthorityBump), {
      accounts: {
        mint: mint.publicKey,
        mintAuthority: mintAuthority,
        user: program.provider.wallet.publicKey,
        tokenProgram: spl.TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY
      },
      signers: [mint]
    });

    let usersAssociatedTokenAccount = await spl.Token.getAssociatedTokenAddress(
      spl.ASSOCIATED_TOKEN_PROGRAM_ID,
      spl.TOKEN_PROGRAM_ID,
      mint.publicKey,
      program.provider.wallet.publicKey,
    );

    await program.rpc.airdrop(new anchor.BN(mintAuthorityBump), {
      accounts: {
        mint: mint.publicKey,
        mintAuthority: mintAuthority,
        user: program.provider.wallet.publicKey,
        destination: usersAssociatedTokenAccount,
        tokenProgram: spl.TOKEN_PROGRAM_ID
      },
      instructions: [
        spl.Token.createAssociatedTokenAccountInstruction(
          spl.ASSOCIATED_TOKEN_PROGRAM_ID,
          spl.TOKEN_PROGRAM_ID,
          mint.publicKey,
          usersAssociatedTokenAccount,
          provider.wallet.publicKey,
          provider.wallet.publicKey
        )
      ]
    });

    let tokenAccountInfo = spl.AccountLayout.decode(
      (await provider.connection.getAccountInfo(usersAssociatedTokenAccount)).data
    );
    assert.equal(1, spl.u64.fromBuffer(tokenAccountInfo.amount).toNumber());

    await program.rpc.burn({
      accounts: {
        mint: mint.publicKey,
        user: program.provider.wallet.publicKey,
        source: usersAssociatedTokenAccount,
        tokenProgram: spl.TOKEN_PROGRAM_ID
      },
    });

    tokenAccountInfo = spl.AccountLayout.decode(
      (await provider.connection.getAccountInfo(usersAssociatedTokenAccount)).data
    );
    assert.equal(0, spl.u64.fromBuffer(tokenAccountInfo.amount).toNumber());

  });

  it("Creates and initializes an account in a single atomic transaction (simplified)", async () => {
    // #region code-simplified
    // The program to execute.
    // The Account to create.
    const myAccount = anchor.web3.Keypair.generate();

    // Create the new account and initialize it with the program.
    // #region code-simplified
    await program.rpc.initialize(new anchor.BN(1234), {
      accounts: {
        myAccount: myAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [myAccount],
    });
    // #endregion code-simplified

    // Fetch the newly created account from the cluster.
    const account = await program.account.myAccount.fetch(myAccount.publicKey);

    // Check it's state was initialized.
    assert.ok(account.data.eq(new anchor.BN(1234)));

    // Store the account for the next test.
    _myAccount = myAccount;
  });

  it("Updates a previously created account", async () => {
    const myAccount = _myAccount;

    // #region update-test


    // Invoke the update rpc.
    await program.rpc.update(new anchor.BN(4321), {
      accounts: {
        myAccount: myAccount.publicKey,
      },
    });

    // Fetch the newly updated account.
    const account = await program.account.myAccount.fetch(myAccount.publicKey);

    // Check it's state was mutated.
    assert.ok(account.data.eq(new anchor.BN(4321)));

    // #endregion update-test
  });
});
