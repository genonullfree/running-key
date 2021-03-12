use clap::{App, Arg};
use std::io;

fn to_run(input: &String) -> Vec<u8> {
    input
        .chars()
        .filter(|b| b.is_ascii_alphabetic())
        .map(|c| {
            if c.is_ascii_uppercase() {
                c as u8 - 0x41
            } else {
                c as u8 - 0x61
            }
        })
        .collect()
}

fn from_run(input: Vec<u8>) -> String {
    input
        .iter()
        .map(|c| (c + 0x41) as char)
        .collect::<String>()
        .to_string()
}
fn decrypt(input: Vec<u8>, key: Vec<u8>) -> Vec<u8> {
    input
        .iter()
        .zip(key.iter())
        .map(|(a, b)| {
            let mut c = 0;
            if b > a {
                c += a + 26 as u8;
            } else {
                c += a;
            }
            c - b
        })
        .collect()
}

fn main() {
    let matches = App::new("running-key")
        .version("0.1.0")
        .author("geno")
        .about("Encrypt or decrypt with running key encryption.")
        .arg(
            Arg::with_name("encrypt")
                .short("e")
                .long("encrypt")
                .help("Encrypt with running key encryption")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("decrypt")
                .short("d")
                .long("decrypt")
                .help("Decrypt with running key encryption")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("key")
                .short("k")
                .long("key")
                .help("Key to use for encrypt/decrypt")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("text")
                .short("t")
                .long("text")
                .help("Text to encrypt/decrypt")
                .takes_value(true),
        )
        .get_matches();

    let mut line = String::new();
    let mut key = String::new();

    if matches.is_present("text") {
        line = matches.value_of("text").unwrap().to_string();
    } else {
        println!("Enter the ciphertext: ");
        io::stdin()
            .read_line(&mut line)
            .expect("Error reading in ciphertext");
    }

    if matches.is_present("key") {
        key = matches.value_of("key").unwrap().to_string();
    } else {
        println!("Enter the key: ");
        io::stdin()
            .read_line(&mut key)
            .expect("Error reading in key");
    }

    let l = to_run(&line);
    let k = to_run(&key);

    if matches.is_present("encrypt") {
        println!("ToDo");
    } else if matches.is_present("decrypt") {
        let d = decrypt(l, k);
        let o = from_run(d);
        println!("decrypted: [{}]", o);
    }
}
