
#![feature(box_syntax)]
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
mod util;
use std::error;
use shifty::{Action, Encoding};
use clap::{Arg, ArgMatches, ArgGroup, App, SubCommand};

pub type Result<T> = std::result::Result<T, Box<error::Error>>;

/// Prevent a large amount of dead code from throwing warnings.
#[allow(dead_code)]
fn shut_up_dead_code() -> String {
    let test_string = "";
    let e = Encoding::parse(test_string).unwrap();
    e.encrypt("foo", "bar").unwrap();
    e.decrypt("foo", "bar").unwrap()
}

fn transcode(action: Action, cmd: &ArgMatches) -> Result<()> {
    debug!("Running {:?} -> {:?}\n", action, cmd);
    let encoding = shifty::alphanumeric_space();


    let mut key = if cmd.is_present("keystring") {
        cmd.value_of("keystring").unwrap().to_string()
    } else if cmd.is_present("keyfile") {
        let raw_path = cmd.value_of("keyfile").unwrap();
        try!(util::read_path(raw_path))
    } else {
        // clap should force the user to pick one or the other
        panic!("Attempted to transcode without either keystring or keyfile set!")
    };

    let mut input = if cmd.is_present("inputstring") {
        cmd.value_of("inputstring").unwrap().to_string()
    } else if cmd.is_present("inputfile") {
        let raw_path = cmd.value_of("inputfile").unwrap();
        try!(util::read_path(raw_path))
    } else {
        // clap should force the user to pick one or the other
        panic!("Attempted to transcode without either inputstring or inputfile set!")
    };


    if !cmd.is_present("strict") {
        key = encoding.map_filter_string(&key);
        input = encoding.map_filter_string(&input);
    }

    info!("Input: {}", input);
    info!("Key: {}", key);
    let output = try!(encoding.transform_message(&input, &key, action));
    println!("{}", output);
    Ok(())
}

fn main() {
    env_logger::init().unwrap();
    let cli_context = App::new("caesar")
        .version("0.1.0")
        .author("Scott Schroeder <scottschroeder@sent.com>")
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
            .arg(Arg::with_name("strict")
                .long("strict")
                .help("Fail if unknown characters are encountered"))
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
        .arg(Arg::with_name("strict")
            .long("strict")
            .help("Fail if unknown characters are encountered"))
        .get_matches();

    debug!("{:?}", cli_context);

    let result = match cli_context.subcommand() {
        ("encrypt", Some(cmd)) => transcode(Action::Encrypt, cmd),
        ("decrypt", Some(cmd)) => transcode(Action::Decrypt, cmd),
        (unkown_cmd, Some(_)) => panic!("Unknown command '{}'", unkown_cmd),
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
}
