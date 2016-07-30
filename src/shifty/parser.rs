use std;
use toml::{self, Array, Value, Table};

use super::Encoding;
use super::error::ConfigError;

pub type Result<T> = std::result::Result<T, ConfigError>;

const ALPHABET_KEY: &'static str = "alphabet";
const MAPPING_KEY: &'static str = "mapping";

fn string_to_char(s: &String) -> Result<char> {
    match s.len() {
        0 => Err(ConfigError::ValueNotChar(format!("Can not get char from empty string"))),
        1 => Ok(s.chars().nth(0).unwrap()),
        _ => {
            Err(ConfigError::ValueNotChar(format!("String '{}' is more than just a single char",
                                                  s)))
        }
    }
}

fn char_from_toml_value(value: &Value) -> Result<char> {
    match *value {
        Value::String(ref s) => string_to_char(s),
        ref x => Err(ConfigError::ValueNotChar(format!("Value {:?} is not a string type", x))),
    }
}


fn parse_alphabet(root_table: &Table) -> Result<Vec<char>> {
    // Get the array, if it exists
    let alphabet: Option<&Array> = match root_table.get(ALPHABET_KEY) {
        Some(&Value::Array(ref abc)) => Some(abc),
        Some(x) => {
            return Err(ConfigError::SchemaError(format!("Key '{}' did not have Array: {:?}",
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
            return Err(ConfigError::SchemaError(format!("Key '{}' did not have Table: {:?}",
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

pub fn new_from_toml(root_table: Table) -> super::Result<Encoding> {
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

    debug!("Encoding: {:?}", new_encoding);
    Ok(new_encoding)
}


pub fn read_toml_string(toml: &str) -> super::Result<Table> {
    toml::Parser::new(toml).parse().ok_or(From::from(ConfigError::InvalidToml))
}


#[cfg(test)]
mod tests {
    use super::super::Error;
    use super::super::ConfigError;
    use super::super::Encoding;

    #[test]
    fn fail_to_parse_bad_toml() {
        let test_string = r#"a = a"#;
        match Encoding::parse(test_string) {
            Ok(_) => panic!("We parsed invalid TOML!"),
            Err(Error::InvalidConfig(ConfigError::InvalidToml)) => (),
            Err(e) => panic!("We failed with the wrong type of error {:?}", e),
        }
    }

    #[test]
    fn fail_to_parse_bad_char_in_alphabet() {
        let test_string = r#"
        alphabet = ["abc"]
        "#;

        match Encoding::parse(test_string) {
            Err(Error::InvalidConfig(ConfigError::ValueNotChar(_))) => (),
            Ok(_) => panic!("We parsed an invalid Alphabet!"),
            Err(e) => panic!("We failed with the wrong type of error {:?}", e),
        }

    }
    #[test]
    fn fail_to_parse_bad_char_in_mapping() {
        let test_string = r#"
        [mapping]
        Abc = "a"
        "#;

        match Encoding::parse(test_string) {
            Err(Error::InvalidConfig(ConfigError::ValueNotChar(_))) => (),
            Ok(_) => panic!("We parsed an invalid Mapping!"),
            Err(e) => panic!("We failed with the wrong type of error {:?}", e),
        }

    }

    // TODO: Test all the various types of errors that we throw
}
