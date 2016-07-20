#[macro_use]
extern crate log;
extern crate env_logger;
extern crate itertools;
use itertools::Itertools;

const LETTERS: u8 = 26;
const LOWERCASE: u8 = 'a' as u8 - 'A' as u8;

enum Action {
    Encrypt,
    Decrypt,
}

fn char_to_number(c: &char) -> u8 {
    *c as u8 - 'a' as u8
}

fn number_to_char(n: &u8) -> char {
    (*n % LETTERS + 'a' as u8) as char
}

fn char_in_working_set(c: &char) -> bool {
    match *c {
        'a'...'z' => true,
        _ => false,
    }
}

fn map_char(c: &char) -> char {
    match *c {
        'A'...'Z' => (*c as u8 + LOWERCASE) as char,
        _ => *c,
    }
}

fn reduce_string(s: &String) -> String {
    s.chars()
        .map(|c| map_char(&c))
        .filter(|c| char_in_working_set(&c))
        .join("")
}

fn vectorize_string(s: &String) -> Vec<u8> {
    s.chars()
        .map(|c| char_to_number(&c))
        .collect()
}


fn transform_message(message: &String, keytext: &String, action: Action) -> String {
    let key: Vec<u8> = vectorize_string(keytext);
    let keysize = key.len();
    let direction: i32 = match action {
        Action::Encrypt => 1,
        Action::Decrypt => -1,
    };
    message.chars()
        .enumerate()
        .map(|(i, c)| {
            let m = char_to_number(&c);
            let mut shift = key[i % keysize] as i32 * direction;
            if shift < 0 {
                shift += LETTERS as i32
            }
            let x = m + shift as u8;
            let c = number_to_char(&x);
            trace!("{}: {} +/- {} = {}", i, m, shift, c);
            c
        })
        .join("")
}


fn main() {
    env_logger::init().unwrap();

    info!("starting up");

    let s: String = "abc Hello, World! xyz".to_string();
    let k: String = "This is my key".to_string();
    println!("Raw Message: {}", s);
    println!("Raw Keytext: {}", k);
    let message = reduce_string(&s);
    let keytext = reduce_string(&k);
    println!("Message: {}", message);
    println!("Keytext: {}", keytext);
    let ciphertext = transform_message(&message, &keytext, Action::Encrypt);
    println!("Ciphertext {}", ciphertext);
    let plaintext = transform_message(&ciphertext, &keytext, Action::Decrypt);
    println!("Plaintext {}", plaintext);


}


#[test]
fn it_works() {}

#[test]
fn end_to_end() {
    let s: String = "abc Hello, World! xyz".to_string();
    let k: String = "This is my key".to_string();
    let message = reduce_string(&s);
    let keytext = reduce_string(&k);
    let ciphertext = transform_message(&message, &keytext, Action::Encrypt);
    let plaintext = transform_message(&ciphertext, &keytext, Action::Decrypt);
    assert_eq!(message, plaintext);
}
