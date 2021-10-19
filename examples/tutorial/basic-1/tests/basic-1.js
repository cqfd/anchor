const assert = require("assert");
const anchor = require("@project-serum/anchor");
const spl = require("@solana/spl-token");
const { SystemProgram } = anchor.web3;

describe("basic-1", () => {
  // Use a local provider.
  const provider = anchor.Provider.local();
  const program = anchor.workspace.Basic1;
  // const connection = new anchor.web3.Connection("http://localhost:8899");
  // const wallet = anchor.web3.Keypair.generate();
  // const provider = new anchor.Provider(connection, {
  //   publicKey: wallet.publicKey,
  //   async signTransaction(tx) {
  //     tx.sign(wallet);
  //     return tx;
  //   },
  //   async signAllTransactions(txs) {
  //     return txs.map(tx => { tx.sign(wallet); return tx; });
  //   }
  // }, anchor.Provider.defaultOptions());

  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  it("Creates and initializes an account in a single atomic transaction (simplified)", async () => {
    // #region code-simplified
    // The program to execute.

    // The Account to create.
    const myAccount = anchor.web3.Keypair.generate();

    // Create the new account and initialize it with the program.
    // #region code-simplified
    await program.rpc.initialize(new anchor.BN(1234), {
      accounts: {
        myAccount: myAccount.publicKey.toString(),
        user: provider.wallet.publicKey.toString(),
        systemProgram: SystemProgram.programId.toString(),
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

    // The program to execute.
    // const program = anchor.workspace.Basic1;

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

  it("Can initialize a mint", async () => {
    let mint = anchor.web3.Keypair.generate();
    let [us, _bump] = await anchor.web3.PublicKey.findProgramAddress([], program.programId);
    debugger;
    await program.rpc.initMint({
      accounts: {
        mint: mint.publicKey,
        user: program.provider.wallet.publicKey,
        us: us,
        tokenProgram: spl.TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY
      },
      signers: [mint]
    });

    let mintAccount = await program.provider.connection.getAccountInfo(mint.publicKey);
    console.log(mintAccount.owner.toString());

    // let destination = anchor.web3.Keypair.generate();
    let destinationTokenAccount = await spl.Token.getAssociatedTokenAddress(
      spl.ASSOCIATED_TOKEN_PROGRAM_ID,
      spl.TOKEN_PROGRAM_ID,
      mint.publicKey,
      program.provider.wallet.publicKey
    );

    debugger;
    await program.rpc.mintSomeTokens(new anchor.BN(_bump), {
      accounts: {
        mint: mint.publicKey,
        destination: destinationTokenAccount,
        user: program.provider.wallet.publicKey,
        us: us,
        tokenProgram: spl.TOKEN_PROGRAM_ID,
        associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY
      },
    });

    let destinationTokenAccountInfo = await program.provider.connection.getAccountInfo(destinationTokenAccount);

    debugger;
    console.log(destinationTokenAccountInfo);

  });
});
