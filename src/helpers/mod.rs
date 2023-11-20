use solana_sdk::signature::Keypair;
use std::env;

pub fn keypair() -> Keypair {
    let bytes = env::var("KEYPAIR").unwrap();
    Keypair::from_bytes(string_u8(bytes.as_str()).as_slice()).unwrap()
}

pub fn parse_pubkey(slice: &[u8]) -> [u8; 32] {
    slice.try_into().expect("incorrect slice length")
}

pub fn string_u8(string: &str) -> Vec<u8> {
    let trim = string
        .replace("[", "")
        .replace("]", "")
        .replace(" ", "")
        .replace("\n", "");

    let split: Vec<&str> = trim.split(",").collect();

    let mut result: Vec<u8> = Vec::new();

    for x in split {
        if x.len() > 0 {
            result.push(x.to_owned().parse::<u8>().unwrap())
        }
    }

    // println!("result : {:#?}", result);

    result
}
