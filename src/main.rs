use std::env::args;
use std::i64;

mod errors;

pub struct FoundError {
    domain: String,
    message: String,
    code: String,
}

fn main() {
    let hex_code = args().skip(1).next();

    match hex_code {
        Some(code) => {
            let without_prefix = code.trim_start_matches("0x");
            let err_code = i64::from_str_radix(without_prefix, 16);
            match err_code {
                Ok(code) => {
                    if code >= 6000 {
                        println!("Lol! You defined this error bruh 🦀 :)");
                        println!("It's {}st error, that you defined on your program", code.wrapping_sub(6000).wrapping_add(1));
                        println!("Probably, You've defined this error at lib.rs or error.rs file");
                        println!("Go check and fix it ASAP, else I'm afarid you NGMI 😈");

                    } else {
                        let errors = find_errors(&code.to_string());
                        if errors.is_none() {
                            println!("No errors found for code: {}", code);
                            std::process::exit(0);
                        }
                        for error in errors {
                            println!("{}", error.domain);
                            println!("{} | {}", error.code, error.message);
                        }
                    }
                }
                Err(e) => println!("Invalid Error code: {:?}", e),
            }
        }
        None => {
            println!("No hex code provided!");
            std::process::exit(1);
        }
    }
}

pub fn find_errors(hex_code: &str) -> Option<FoundError> {
    match errors::ANCHOR_PROGRAM.get(hex_code).cloned() {
        Some(e) => Some(FoundError {
            domain: "Anchor Program".to_string(),
            message: e.to_string(),
            code: hex_code.to_string(),
        }),
        None => None,
    }
}
