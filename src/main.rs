#[macro_use]
extern crate log;
#[macro_use]
extern crate custom_derive;
#[macro_use]
extern crate newtype_derive;
extern crate env_logger;
extern crate itertools;
extern crate toml;
extern crate clap;
mod shifty;
use shifty::Encoding;

fn shut_up_dead_code() {
    let test_string = r#"
    alphabet = ["a", "b", "c"]
    [mapping]
    A = "a"
    B = "b"
    C = "c"
    "#;
    Encoding::parse(test_string).unwrap();
}


fn main() {
    env_logger::init().unwrap();
	shut_up_dead_code();

    let encoding = shifty::alphanumeric_space();

    let s: String = "abc Hello, World123! xyz".to_string();
    let k: String = "This is a key".to_string();
    let message = encoding.map_filter_string(&s);
    let keytext = encoding.map_filter_string(&k);
    println!("Message: {}", message);
    println!("Key: {}", keytext);
    let ciphertext = encoding.encrypt(&message, &keytext).unwrap();
    println!("Ciphertext: {}", ciphertext);
    let plaintext = encoding.decrypt(&ciphertext, &keytext).unwrap();
    println!("Plaintext: {}", plaintext);

}
