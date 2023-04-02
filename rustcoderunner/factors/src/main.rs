
use methods::{MULTIPLY_ELF, MULTIPLY_ID}; // TODO Dynamically load and access guest files like: mothods[`${name.toUpperCase()}_ELF`]
use risc0_zkvm::Prover;
use risc0_zkvm::serde::{from_slice, to_vec};

use std::any;
use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, BufWriter, BufRead, Read, Write};
use serde_json::{Value, from_str};

// struct RunParams {
//     name: String,
//     paramType: String,
//     value: any
// }

// type RunData = any;

fn main() {
    // Listens to api requests to run code.
    let listener = TcpListener::bind("localhost:4000").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }

}
// parameters:  [ params: Vec<RunParams>, data: Vec<RunData> ]
fn run_prover() {
    let mut prover = Prover::new(MULTIPLY_ELF).unwrap();
    // let p = serialize_params(&params);
    // let d = serialize_data(&data);
    // prover.add_input_u32_slice(&to_vec(&p).unwrap());
    // prover.add_input_u32_slice(&to_vec(&d).unwrap());

    let receipt = prover.run().unwrap();
    let result: u64 = from_slice(&receipt.journal).unwrap();
    // return result { result, proof };
    // (&result.to_string(), &result.to_string())
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
    let mut body = String::new();

    reader.read_to_string(&mut body).unwrap();
    // let body_json: Value = from_str(&body).unwrap();

    let response = match (method, path) {
        ("POST", "/run") => {
            // let code = body_json["code"].as_str().unwrap();
            // let params = body_json["params"].as_array().unwrap();
            // let data = body_json["data"].as_array().unwrap();

            run_prover();
            // todo returns result of run_prover back to client.
            format!("{{\"result\": \"{}\", \"proof\": \"{}\" }}", "result", "proof")
        },
        _ => String::from("{\"error\": \"invalid request\"}")
    };

    writer.write(response.as_bytes()).unwrap();
    writer.flush().unwrap();
}


// fn serialize_params(vector: &Vec<RunParams>) -> Vec<u8> {
//     let serialized = serde_json::to_string(vector).unwrap();
//     serialized.into_bytes()
// }

// fn serialize_data(vector: &Vec<RunData>) -> Vec<u8> {
//     let serialized = serde_json::to_string(vector).unwrap();
//     serialized.into_bytes()
// }
