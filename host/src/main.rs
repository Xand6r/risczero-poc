use methods::{HELLO_GUEST_ELF, HELLO_GUEST_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};
pub mod types;


fn main() {
    let env = ExecutorEnv::builder().build().unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove(env, HELLO_GUEST_ELF).unwrap();

    // Extract journal of receipt
    let output: String= receipt.journal.decode().unwrap();

    // Print, notice, after committing to a journal, the private input became public
    println!("Hello, world! I generated a proof of guest execution! {:?} is a public output from journal ", output);
}