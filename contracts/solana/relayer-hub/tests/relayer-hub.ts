import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {Keypair, PublicKey} from "@solana/web3.js";
import { RelayerHub } from "../target/types/relayer_hub";
import { expect,assert } from 'chai'

describe("relayer-hub", () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.RelayerHub as Program<RelayerHub>;

    const pg = program.provider as anchor.AnchorProvider;

    const [configPDA] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("config")
      ],
      program.programId
    )

    const [poolPDA] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("pool")
      ],
      program.programId
    )

    const [relayerInfoPDA] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("relayer")
      ],
      program.programId
    )

    it("Is initialized!", async () => {
        const tx = await program.methods.initialize().accounts({
          config:configPDA,
          pool:poolPDA,
          relayer_info:relayerInfoPDA,
          payer: pg.wallet.publicKey,
        }).rpc();
        console.log("Your transaction signature", tx);
        try{
          await program.methods.initialize().accounts({
              config:configPDA,
              pool:poolPDA,
              relayer_info:relayerInfoPDA,
              payer: pg.wallet.publicKey,
          }).rpc();
        } catch (error) {
          return;
        }
    });

    it("Register relayer", async () => {
        const [relayerPDA] = PublicKey.findProgramAddressSync(
            [Buffer.from('relayer'), pg.wallet.publicKey.toBuffer()],
            program.programId
        )
        const tx = await program.methods.register().accounts({
            relayer_info:relayerInfoPDA,
            relayer:relayerPDA,
            payer: pg.wallet.publicKey,
        }).rpc();
    });

    it("Submit transaction", async () => {
        const [transactionPDA] = PublicKey.findProgramAddressSync(
            [Buffer.from('tx'),pg.wallet.publicKey.toBuffer()],
            program.programId
        )
        const tx = await program.methods.pushTransaction(Buffer.from([1,2])).accounts({
            relayer: pg.wallet.publicKey,
            transaction:transactionPDA,
            pool:poolPDA,
        }).rpc();
    });
});
