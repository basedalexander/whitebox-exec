#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
#![no_std]  // std support is experimental
use risc0_zkvm::guest::env;
risc0_zkvm::guest::entry!(main);

pub fn main() {
    // step 1. Read all the params and desirealize them into entities.
    // step 2. Pass them as parameters to the code that was compiled.
    // step 3. try to abstract and reuse functionality that reads parameters and deserializes them.

    // const params - read one by one and construct params
    // const data - read one by one and construct data.
    let a: u64 = env::read();
    let b: u64 = env::read();
    let result = a.checked_mul(b).expect("Integer overflow");
    env::commit(&result);

    // let n: usize = env::read();

    // // in a loop
    // let vec_len = env::read();
    // let vec = en::read_slice(vec_len);
}
