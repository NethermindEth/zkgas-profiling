#![doc = include_str!("../doc/prover-doc.md")]

use prover_methods::MULTIPLY_ELF;
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt, Prover};

/// Compute the product a*b inside the guest
///
/// # Arguments
///
/// * `a` - The first number to multiply
/// * `b` - The second number to multiply
///
/// # Returns
pub fn multiply(a: u64, b: u64) -> (Receipt, u64) {
    let env = ExecutorEnv::builder()
        // Send a & b to the guest
        .write(&a)
        .unwrap()
        .write(&b)
        .unwrap()
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove(env, MULTIPLY_ELF).unwrap().receipt;

    // Extract journal of receipt (i.e. output c, where c = a * b)
    let c: u64 = receipt.journal.decode().expect(
        "Journal output should deserialize into the same types (& order) that it was written",
    );

    // Report the product
    println!("I know the factors of {}, and I can prove it!", c);

    (receipt, c)
}
