import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Ed25519 } from "../target/types/ed25519";

describe("ed25519", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Ed25519 as Program<Ed25519>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
