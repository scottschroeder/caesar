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
mod encoding;





fn main() {
    env_logger::init().unwrap();

    info!("starting up");

    let encoding = encoding::alphanumeric();
    println!("{:?}", encoding);

    let s: String = "abc Hello, World123! xyz".to_string();
    let k: String = "This is a key".to_string();
    let message = encoding.reduce_string(&s);
    let keytext = encoding.reduce_string(&k);
    println!("Message {}", message);
    println!("Key {}", keytext);
    let ciphertext = encoding.encrypt(&message, &keytext);
    println!("Ciphertext {}", ciphertext);
    let plaintext = encoding.decrypt(&ciphertext, &keytext);
    println!("Plaintext {}", plaintext);

}


#[test]
fn it_works() {}

#[test]
fn end_to_end() {
    let encoding = encoding::alphanumeric();
    let s: String = "abc Hello, World123! xyz".to_string();
    let k: String = "This is a key".to_string();
    let message = encoding.reduce_string(&s);
    let keytext = encoding.reduce_string(&k);
    let ciphertext = encoding.encrypt(&message, &keytext);
    let plaintext = encoding.decrypt(&ciphertext, &keytext);
    assert_eq!(message, plaintext);
}
