#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
#![no_std]  // std support is experimental
use risc0_zkvm::guest::env;
risc0_zkvm::guest::entry!(main);

pub fn main() {
    // step 1. Read all the params and desirealize them into entities.
    // step 2. Pass them as parameters to the code that was compiled.
    // step 3. try to abstract and reuse functionality that reads parameters and deserializes them.


    // let paramsRaw: u64 = env::read();
    // let dataRaw: u64 = env::read();
    // let params = deserialize_params(paramsRaw);
    // let data = deserialize_data(dataRaw);

    // let result = runnder(params, data);

    let result = "mockResult";
    env::commit(&result);
}

// fn deserialize_params(vector: &Vec<RunParams>) -> Vec<u8> {
//     let serialized = serde_json::to_string(vector).unwrap();
//     serialized.into_bytes()
// }

// fn deserialize_data(vector: &Vec<RunData>) -> Vec<u8> {
//     let serialized = serde_json::to_string(vector).unwrap();
//     serialized.into_bytes()
// }

// Function to be dynamically appended to the file below (taken from client)
// fn runner(params, data) {
//     const result = data.filter(d => d.content.contains(params[0].topics[0]));

//     env::commit(&result);
// }
