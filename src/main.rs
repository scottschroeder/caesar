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
use encoding::Encoding;



const ALPHANUMERIC: &'static str = r#"
        alphabet =[
            "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m",
            "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z",
            "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", " ",
        ]
        [mapping]
        A = "a"
        B = "b"
        C = "c"
        D = "d"
        E = "e"
        F = "f"
        G = "g"
        H = "h"
        I = "i"
        J = "j"
        K = "k"
        L = "l"
        M = "m"
        N = "n"
        O = "o"
        P = "p"
        Q = "q"
        R = "r"
        S = "s"
        T = "t"
        U = "u"
        V = "v"
        W = "w"
        X = "x"
        Y = "y"
        Z = "z"
    "#;


fn alphanumeric() -> Encoding {
    let alphanumeric_encoding: Encoding = Encoding::new_from_toml_string(ALPHANUMERIC);
    println!("Alphanumeric Encoding {:?}", alphanumeric_encoding);
    alphanumeric_encoding
}

fn main() {
    env_logger::init().unwrap();


    let encoding = alphanumeric();

    let s: String = "abc Hello, World123! xyz".to_string();
    let k: String = "This is a key".to_string();
    let message = encoding.map_filter_string(&s);
    let keytext = encoding.map_filter_string(&k);
    println!("Message {}", message);
    println!("Key {}", keytext);
    let ciphertext = encoding.encrypt(&message, &keytext);
    println!("Ciphertext {}", ciphertext);
    let plaintext = encoding.decrypt(&ciphertext, &keytext);
    println!("Plaintext {}", plaintext);

}
