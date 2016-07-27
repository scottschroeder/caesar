#![feature(try_from)]
#[macro_use]
extern crate log;
#[macro_use]
extern crate custom_derive;
#[macro_use]
extern crate newtype_derive;
extern crate env_logger;
extern crate itertools;
extern crate toml;
use itertools::Itertools;
mod encoding;
use encoding::{Action, Encoding, EncodeNum, transform};

fn reduce_string(s: &String, encoding: &Encoding) -> String {
    s.chars()
        .map(|c| encoding.map_char(&c))
        .filter(|c| encoding.char_in_working_set(&c))
        .join("")
}

fn vectorize_string(s: &String, encoding: &Encoding) -> Vec<EncodeNum> {
    s.chars()
        .map(|c| encoding.char_to_number(&c))
        .collect()
}


fn transform_message(message: &String,
                     keytext: &String,
                     action: Action,
                     encoding: &Encoding)
                     -> String {
    let key: Vec<EncodeNum> = vectorize_string(keytext, encoding);
    let keysize = key.len();
    message.chars()
        .enumerate()
        .map(|(i, c)| {
            let message_num: EncodeNum = encoding.char_to_number(&c);
            let key_num: EncodeNum = key[i % keysize];
            let cipher_num = transform(&message_num, &key_num, &encoding.len(), &action);
            trace!("message_num: {:?} key_num: {:?} {:?} -> {:?}",
                   message_num,
                   key_num,
                   action,
                   cipher_num);
            encoding.number_to_char(&cipher_num)
        })
        .join("")
}


fn main() {
    env_logger::init().unwrap();

    info!("starting up");

    let encoding = encoding::alphanumeric();
    println!("{:?}", encoding);

    let s: String = "AaBbCc".to_string();
    let k: String = "cab".to_string();
    let s: String = "abc Hello, World123! xyz".to_string();
    let k: String = "This is a key".to_string();
    let message = reduce_string(&s, &encoding);
    let keytext = reduce_string(&k, &encoding);
    println!("Message {}", message);
    println!("Key {}", keytext);
    let ciphertext = transform_message(&message, &keytext, Action::Encrypt, &encoding);
    println!("Ciphertext {}", ciphertext);
    let plaintext = transform_message(&ciphertext, &keytext, Action::Decrypt, &encoding);
    println!("Plaintext {}", plaintext);

}


#[test]
fn it_works() {}

#[test]
fn end_to_end() {
    let encoding = encoding::short_abc();
    let s: String = "abc Hello, World! xyz".to_string();
    let k: String = "This is a key".to_string();
    let message = reduce_string(&s, &encoding);
    let keytext = reduce_string(&k, &encoding);
    let ciphertext = transform_message(&message, &keytext, Action::Encrypt, &encoding);
    let plaintext = transform_message(&ciphertext, &keytext, Action::Decrypt, &encoding);
    assert_eq!(message, plaintext);
}
