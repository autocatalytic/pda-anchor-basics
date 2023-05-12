import * as anchor from "@coral-xyz/anchor";
import { PdaTreasure } from "../target/types/pda_treasure";
import { PublicKey, SystemProgram } from "@solana/web3.js";

const TREASURY_PDA_SEED = "treasure";
const ENCODED_TPDA_SEED = anchor.utils.bytes.utf8.encode(TREASURY_PDA_SEED);

describe("pda_treasure", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider); 
  const connection = provider.connection;

  const program = anchor.workspace.PdaTreasure as anchor.Program<PdaTreasure>;

  it("Prints Funds!", async () => {
    const [treasury_pda, bump_seed] = PublicKey.findProgramAddressSync([ENCODED_TPDA_SEED], program.programId);
  
    const tx = await program.methods.printFunds().rpc();

    // Stupid simple, juse use logs or explorer to see if it worked.
    console.log("Your transaction signature", tx);
  });

  it("can withdraw!", async () => {
    const [treasury_pda, bump_seed] = PublicKey.findProgramAddressSync([ENCODED_TPDA_SEED], program.programId);

    // go ahead and hard code the receiver, easy enough to make dynamic on client side
    const receiver = new anchor.web3.PublicKey("CUyV6rtEueziXaWFi1x5SnMe6T5oJcvUTHRSFDZBNgTA");
    const lamport_withdrawal = new anchor.BN(1000000000);

    const tx = await program.methods.withdrawalHandler(bump_seed, lamport_withdrawal)
      .accounts({
        treasury: treasury_pda,
        destination: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
    console.log("Withdrawal tx signature: ", tx);
     
  })

});
