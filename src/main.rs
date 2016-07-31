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
use std::error;
use shifty::{Action, Encoding};
use clap::{Arg, ArgMatches, ArgGroup, App, SubCommand};

pub type Result<T> = std::result::Result<T, Box<error::Error>>;

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

fn transcode(action: Action, cmd: &ArgMatches) -> Result<()> {
    debug!("Running {:?} -> {:?}\n", action, cmd);
    let encoding = shifty::alphanumeric_space();
    let keytext = cmd.value_of("keystring").unwrap();
    let input = cmd.value_of("inputstring").unwrap();
    println!("Input: {}", input);
    println!("Key: {}", keytext);
    let output = try!(encoding.transform_message(input, keytext, action));
    println!("Output: {}", output);
    Ok(())
}

fn main() {
    env_logger::init().unwrap();
    let cli_context = App::new("caesar")
        .version("0.1.0")
        .author("Scott Schroeder <scott19904@gmail.com>")
        .about("A CLI tool for working with the Vigenère cipher.")
        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"))
        .subcommand(SubCommand::with_name("encrypt")
            .arg(Arg::with_name("keystring")
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
                .help("Text to be encrypted"))
            .arg(Arg::with_name("inputfile")
                .long("input-file")
                .takes_value(true)
                .help("Path to file with text to be encrypted"))
            .group(ArgGroup::with_name("input_source")
                .arg("inputstring")
                .arg("inputfile")
                .required(true)))
        .subcommand(SubCommand::with_name("decrypt")
            .arg(Arg::with_name("keystring")
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
                .help("Text to be decrypted"))
            .arg(Arg::with_name("inputfile")
                .long("input-file")
                .takes_value(true)
                .help("Path to file with text to be decrypted"))
            .group(ArgGroup::with_name("input_source")
                .arg("inputstring")
                .arg("inputfile")
                .required(true)))
        .get_matches();

    // debug!("{:?}", cli_context);

    let result = match cli_context.subcommand() {
        ("encrypt", Some(cmd)) => transcode(Action::Encrypt, cmd),
        ("decrypt", Some(cmd)) => transcode(Action::Decrypt, cmd),
        (unkown_cmd, Some(cmd)) => panic!("Unknown command '{}'", unkown_cmd),
        _ => {
            println!("{}", cli_context.usage());
            println!("Run with '-h' or '--help' for more information.");
            Ok(())
        }
    };

    match result {
        Ok(_) => (),
        Err(e) => println!("Encountered an error: {}", e),
    }
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
