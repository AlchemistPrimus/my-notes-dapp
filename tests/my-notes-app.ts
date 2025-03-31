import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MyNotesApp } from "../target/types/my_notes_app";
import { assert } from "chai";

describe("my-notes-app", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.MyNotesApp as Program<MyNotesApp>;
  const author = program.provider as anchor.AnchorProvider;

  it("Is initialized!", async () => {
    // Add your test here.
    const note = anchor.web3.Keypair.generate();
    const tx = await program.methods.addNote("Introduction", "This is my life in solitude...")
    .accounts({ note: note.publicKey, systemProgram: anchor.web3.SystemProgram.programId })
    .signers([note]).rpc();
    console.log("Your transaction signature", tx);

    const noteAccount = await program.account.note.fetch(note.publicKey);
    console.log("Your note", noteAccount);

    assert.equal(noteAccount.author.toBase58(), author.wallet.publicKey.toBase58());
    assert.equal(noteAccount.text, "This is my life in solitude...");
    assert.equal(noteAccount.title, "Introduction");
    assert.ok(noteAccount.createdAt);
    assert.ok(noteAccount.createdAt);
  });
});
