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
fn crypt(input: Vec<u8>, key: Vec<u8>) -> Vec<u8> {
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
    let mut line = String::new();
    let mut key = String::new();
    println!("Enter the ciphertext: ");
    io::stdin()
        .read_line(&mut line)
        .expect("Error reading in ciphertext");
    println!("Enter the key: ");
    io::stdin()
        .read_line(&mut key)
        .expect("Error reading in key");
    let l = to_run(&line);
    let k = to_run(&key);
    let d = crypt(l, k);
    let o = from_run(d);
    println!("decrypted: [{}]", o);
}
