// TODO: Update the name of the method loaded by the prover. E.g., if the method
// is `multiply`, replace `METHOD_NAME_ELF` with `MULTIPLY_ELF` and replace
// `METHOD_NAME_ID` with `MULTIPLY_ID`
use methods::{MULTIPLY_ELF, MULTIPLY_ID};
use risc0_zkvm::Prover;
use risc0_zkvm::serde::{from_slice, to_vec};

use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, BufWriter, BufRead, Read, Write};
use serde_json::{Value, from_str};

struct RunParams {

}

struct RunData {

}

fn main() {

    let listener = TcpListener::bind("localhost:4000").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
    // Make the prover.
    // TODO: Implement communication with the guest here

    // let n:usize = 4;
    // prover.add_input_u32_slice(&to_vec(&n).unwrap());

    // in a loop, n times
    // let vec1 = vec![1,2,3];
}

fn run_prover(a: u64, b: u64) -> (String, String) {
    let mut prover = Prover::new(MULTIPLY_ELF).unwrap();
    prover.add_input_u32_slice(&to_vec(&a).unwrap());
    prover.add_input_u32_slice(&to_vec(&b).unwrap());
    let receipt = prover.run().unwrap();
    let c: u64 = from_slice(&receipt.journal).unwrap();
    println!("Hello, world! I know the factors of {}, and I can prove it!", c);
    ("result".to_string(), "proof".to_string())
}

fn handle_connection(mut stream: TcpStream) {
    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);
    let mut buffer = String::new();
    reader.read_line(&mut buffer).unwrap();
    let request = buffer.split_whitespace().collect::<Vec<&str>>();
    if request.len() < 2 {
        let response = String::from("{\"error\": \"invalid request\"}");
        writer.write(response.as_bytes()).unwrap();
        writer.flush().unwrap();
        return;
    }
    let method = request[0];
    let path = request[1];
    // let mut body = String::new();
    // reader.read_to_string(&mut body).unwrap();
    // let body_json: Value = from_str(&body).unwrap();
    let response = match (method, path) {
        ("POST", "/run") => {
            // let code = body_json["code"].as_str().unwrap();
            // let params = body_json["params"].as_array().unwrap();
            // let data = body_json["data"].as_array().unwrap();
            let (result, proof) = run_prover(14, 25);
            format!("{{\"result\": \"{}\", \"proof\": \"{}\" }}", result, proof)
        },
        _ => String::from("{\"error\": \"invalid request\"}")
    };
    writer.write(response.as_bytes()).unwrap();
    writer.flush().unwrap();
}
