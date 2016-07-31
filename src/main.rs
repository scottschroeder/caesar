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
use clap::{Arg, ArgGroup, App, SubCommand};

#[allow(dead_code)]
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

fn text_key_args<'a>(app: App<'a, 'a>, action: &str) -> App<'a, 'a> {

        app.arg(Arg::with_name("keystring")
            .short("k")
            .long("key")
            .value_name("KEYSTRING")
            .takes_value(true)
            .help("The secret key"))
        .arg(Arg::with_name("keyfile")
            .long("key-file")
            .value_name("KEYFILE")
            .takes_value(true)
            .help("Path to file with secret key"))
        .group(ArgGroup::with_name("key_source")
            .arg("keystring")
            .arg("keyfile")
            .required(true))
        .arg(Arg::with_name("inputstring")
            .short("i")
            .long("input")
            .takes_value(true)
            .help(&format!("The text to be {}", action)))
        .arg(Arg::with_name("inputfile")
            .long("input-file")
            .takes_value(true)
            .help(&format!("Path to file with text to be {}", action)))
        .group(ArgGroup::with_name("input_source")
            .arg("inputstring")
            .arg("inputfile")
            .required(true))
}

fn main() {
    env_logger::init().unwrap();
    let mut app = App::new("caesar")
        .version("0.1.0")
        .author("Scott Schroeder <scott19904@gmail.com>")
        .about("A CLI tool for working with the Vigenère cipher.")
        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"))
        .subcommand(SubCommand::with_name("test")
            .about("controls testing features")
            .version("1.3")
            .author("Someone E. <someone_else@other.com>")
            .arg(Arg::with_name("debug")
                .short("d")
                .help("print debug information verbosely")));
    app = text_key_args(app, "encrypted");

    let matches = app.get_matches();
    println!("{:?}", matches);
    return;


















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
