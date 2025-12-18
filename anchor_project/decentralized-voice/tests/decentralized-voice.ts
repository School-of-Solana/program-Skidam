import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { DecentralizedVoice } from "../target/types/decentralized_voice";

describe("decentralized-voice", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  
  const program = anchor.workspace.decentralizedVoice as Program<DecentralizedVoice>;
  let name = "pooler"; 
  let candidates = "to be changed"

  it("Is initialized!", async () => {
    
    // Add your test here.
    const tx = await program.methods.poolCreate(name,candidates).rpc();
    console.log("Your transaction signature", tx);
  });
});