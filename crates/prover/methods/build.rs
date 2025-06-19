//! Build script for prover-methods crate
//! 
//! This build script generates the methods.rs file that is included in lib.rs

fn main() {
    // Add documentation to the generated constants
    std::env::set_var("RISC0_BUILD_DOCS", "1");
    risc0_build::embed_methods();
}