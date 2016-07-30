use toml::{self, Array, Value, Table};

use super::Result;
use super::Error;
use super::Encoding;

const ALPHABET_KEY: &'static str = "alphabet";
const MAPPING_KEY: &'static str = "mapping";

fn string_to_char(s: &String) -> Result<char> {
    match s.len() {
        0 => Err(Error::NotChar(format!("Can not get char from empty string"))),
        1 => Ok(s.chars().nth(0).unwrap()),
        _ => Err(Error::NotChar(format!("String '{}' is more than just a single char", s))),
    }
}

fn char_from_toml_value(value: &Value) -> Result<char> {
    match *value {
        Value::String(ref s) => string_to_char(s),
        ref x => Err(Error::NotChar(format!("Value {:?} is not a string type", x))),
    }
}


fn parse_alphabet(root_table: &Table) -> Result<Vec<char>> {
    // Get the array, if it exists
    let alphabet: Option<&Array> = match root_table.get(ALPHABET_KEY) {
        Some(&Value::Array(ref abc)) => Some(abc),
        Some(x) => {
            return Err(Error::InvalidConfig(format!("Key '{}' did not have Array: {:?}",
                                                    ALPHABET_KEY,
                                                    x)))
        }
        None => None,
    };

    // Transform Option(&Array) into Vec<char>
    let chars: Vec<char> = match alphabet {
        Some(abc) => {
            try!(abc.iter()
                .map(|x| char_from_toml_value(x))
                .collect())
        }
        None => vec![],
    };
    Ok(chars)
}

fn parse_mapping(root_table: &Table) -> Result<Vec<(char, char)>> {

    let user_mapping: Option<&Table> = match root_table.get(MAPPING_KEY) {
        Some(&Value::Table(ref mapping)) => Some(mapping),
        Some(x) => {
            return Err(Error::InvalidConfig(format!("Key '{}' did not have Table: {:?}",
                                                    MAPPING_KEY,
                                                    x)))
        }
        None => None,
    };


    match user_mapping {
        Some(char_map) => {
            char_map.iter()
                .map(|(pre_map, post_map)| {
                    Ok((try!(string_to_char(pre_map)), try!(char_from_toml_value(post_map))))
                })
                .collect()
        }
        None => Ok(vec![]),
    }
}

pub fn new_from_toml(root_table: Table) -> Result<Encoding> {
    let mut new_encoding = Encoding::new();
    trace!("Root Table: {:?}", root_table);

    let chars = try!(parse_alphabet(&root_table));

    for c in &chars {
        new_encoding.insert_char(*c);
    }

    let mapping = try!(parse_mapping(&root_table));

    for (pre_char, post_char) in mapping {
        new_encoding.insert_map(pre_char, post_char);
    }

    Ok(new_encoding)
}


pub fn read_toml_string(toml: &str) -> Result<Table> {
    match toml::Parser::new(toml).parse() {
        Some(table) => Ok(table),
        None => Err(Error::InvalidToml),
    }
}
