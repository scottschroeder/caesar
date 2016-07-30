use std::collections::HashMap;
use std::fmt;
use toml::{self, Value, Table};
use itertools::Itertools;
use super::Result;
use super::Error;


#[derive(Debug)]
enum Action {
    Encrypt,
    Decrypt,
}

custom_derive! {
    #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
    #[derive(NewtypeFrom, NewtypeAdd, NewtypeSub, NewtypeRem)]
    struct EncodeNum(u64);
}

fn transform(message: &EncodeNum, key: &EncodeNum, size: &usize, action: &Action) -> EncodeNum {
    let s = *size as i32;
    let m = message.0 as i32;
    let k = key.0 as i32;
    let c = match *action {
        Action::Encrypt => m + k,
        Action::Decrypt => m - k,
    };
    let cipher_num = if c < 0 {
        c + s
    } else if c >= s {
        c % s
    } else {
        c
    };

    EncodeNum(cipher_num as u64)
}

impl fmt::Display for EncodeNum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}


#[derive(Debug)]
pub struct Encoding {
    char_number_map: HashMap<char, EncodeNum>,
    number_char_map: HashMap<EncodeNum, char>,
    char_char_map: HashMap<char, char>,
    size: usize,
}

fn char_from_toml_value(value: &Value) -> Result<char> {
    match *value {
        Value::String(ref s) => {
            if s.len() != 1 {
                Err(Error::NotChar(format!("Invalid config, key 'alphabet' has bad char value '{}'", s)))
            } else {
                let c: char = s.chars().nth(0).unwrap();
                Ok(c)
            }
        }
        ref x => Err(Error::InvalidConfig(format!("Key 'alphabet' has non string value: {:?}", x)))
    }
}

impl Encoding {
    pub fn new() -> Self {
        Encoding {
            char_number_map: HashMap::new(),
            number_char_map: HashMap::new(),
            char_char_map: HashMap::new(),
            size: 0,
        }
    }

    pub fn parse(toml: &str) -> Result<Encoding> {
        let root_table: Table = toml::Parser::new(toml).parse().unwrap();
        Encoding::new_from_toml(root_table)
    }

    pub fn insert_char(&mut self, c: char) {
        let map_number = EncodeNum(self.size as u64);
        self.char_number_map.insert(c, map_number);
        self.number_char_map.insert(map_number, c);
        self.size += 1;
    }

    pub fn new_from_toml(root_table: toml::Table) -> Result<Encoding> {
        let mut new_encoding = Self::new();
        trace!("Root Table: {:?}", root_table);
        let alphabet = match root_table.get("alphabet") {
            Some(&Value::Array(ref abc)) => abc,
            _ => return Err(Error::InvalidConfig("Invaid config, key 'alphabet' did not have array.".to_string())),
        };

        // TODO: This seems like the wrong way to do this
        let chars_result: Result<Vec<char>> = alphabet.iter()
            .map(|x| char_from_toml_value(x))
            .collect();
        let chars = try!(chars_result);

        for c in &chars {
            new_encoding.insert_char(*c);
        }

        let char_mapping = match root_table.get("mapping") {
            Some(&Value::Table(ref mapping)) => mapping,
            _ => return Err(Error::InvalidConfig("Invaid config, key 'mapping' did not have Table.".to_string())),
        };
        trace!("char mapping: {:?}", char_mapping);
        for (pre_map, post_map) in char_mapping {
            let pre_char = pre_map.chars().nth(0).unwrap();
            let post_char = try!(char_from_toml_value(post_map));
            new_encoding.insert_map(pre_char, post_char);
        }

        Ok(new_encoding)
    }


    pub fn insert_map(&mut self, x: char, y: char) {
        self.char_char_map.insert(x, y);
    }

    fn char_in_working_set(&self, c: &char) -> bool {
        self.char_number_map.contains_key(c)
    }

    fn char_to_number(&self, c: &char) -> EncodeNum {
        *self.char_number_map.get(c).unwrap()
    }

    fn number_to_char(&self, n: &EncodeNum) -> char {
        *self.number_char_map.get(n).unwrap()
    }

    fn map_char(&self, c: &char) -> char {
        *self.char_char_map.get(c).unwrap_or(c)
    }

    fn vectorize_string(&self, s: &String) -> Vec<EncodeNum> {
        s.chars()
            .map(|c| self.char_to_number(&c))
            .collect()
    }

    pub fn encrypt(&self, message: &String, keytext: &String) -> String {
        self.transform_message(message, keytext, Action::Encrypt)
    }

    pub fn decrypt(&self, message: &String, keytext: &String) -> String {
        self.transform_message(message, keytext, Action::Decrypt)
    }

    fn transform_message(&self, message: &String, keytext: &String, action: Action) -> String {
        let key: Vec<EncodeNum> = self.vectorize_string(keytext);
        let keysize = key.len();
        message.chars()
            .enumerate()
            .map(|(i, c)| {
                let message_num: EncodeNum = self.char_to_number(&c);
                let key_num: EncodeNum = key[i % keysize];
                let cipher_num = transform(&message_num, &key_num, &self.size, &action);
                trace!("message_num: {:?} key_num: {:?} {:?} -> {:?}",
                       message_num,
                       key_num,
                       action,
                       cipher_num);
                self.number_to_char(&cipher_num)
            })
            .join("")
    }

    pub fn map_string(&self, s: &String) -> String {
        s.chars()
            .map(|c| self.map_char(&c))
            .join("")
    }

    pub fn filter_string(&self, s: &String) -> String {
        s.chars()
            .filter(|c| self.char_in_working_set(&c))
            .join("")
    }

    pub fn map_filter_string(&self, s: &String) -> String {
        let mapped_string = self.map_string(s);
        let filtered_string = self.filter_string(&mapped_string);
        filtered_string
    }
}


#[test]
fn create_empty_encoding() {
    let e = Encoding::new();
    assert_eq!(e.size, 0);
    assert_eq!(e.char_number_map.len(), 0);
    assert_eq!(e.number_char_map.len(), 0);
    assert_eq!(e.char_char_map.len(), 0);
}

#[test]
fn insert_new_char() {
    let mut e = Encoding::new();
    e.insert_char('a');
    assert_eq!(e.size, 1);
    assert_eq!(e.char_number_map.len(), 1);
    assert_eq!(e.number_char_map.len(), 1);
    assert_eq!(e.char_char_map.len(), 0);
}

#[test]
fn insert_new_map() {
    let mut e = Encoding::new();
    e.insert_map('A', 'a');
    assert_eq!(e.size, 0);
    assert_eq!(e.char_number_map.len(), 0);
    assert_eq!(e.number_char_map.len(), 0);
    assert_eq!(e.char_char_map.len(), 1);
}

#[test]
fn translate_with_map() {
    let mut e = Encoding::new();
    e.insert_map('A', 'a');
    assert_eq!(e.map_char(&'A'), 'a')
}

#[test]
fn encode() {
    let mut e = Encoding::new();
    e.insert_char('a');
    assert_eq!(e.char_to_number(&'a'), EncodeNum(0))
}

#[test]
fn decode() {
    let mut e = Encoding::new();
    e.insert_char('a');
    assert_eq!(e.number_to_char(&EncodeNum(0)), 'a')
}

#[test]
fn transform_num_simple_add() {
    let m = EncodeNum(2);
    let k = EncodeNum(3);
    let c = EncodeNum(5);
    let abc_size: usize = 100;
    assert_eq!(transform(&m, &k, &abc_size, &Action::Encrypt), c)
}

#[test]
fn transform_num_simple_sub() {
    let m = EncodeNum(2);
    let k = EncodeNum(3);
    let c = EncodeNum(5);
    let abc_size: usize = 100;
    assert_eq!(transform(&c, &k, &abc_size, &Action::Decrypt), m)
}


#[test]
fn transform_num_wrapping_add() {
    let m = EncodeNum(2);
    let k = EncodeNum(3);
    let c = EncodeNum(1);
    let abc_size: usize = 4;
    assert_eq!(transform(&m, &k, &abc_size, &Action::Encrypt), c)
}

#[test]
fn transform_num_wrapping_sub() {
    let m = EncodeNum(2);
    let k = EncodeNum(3);
    let c = EncodeNum(1);
    let abc_size: usize = 4;
    assert_eq!(transform(&c, &k, &abc_size, &Action::Decrypt), m)
}

#[test]
fn transform_char_identity() {
    let mut e = Encoding::new();
    e.insert_char('a');
    e.insert_char('b');
    let m = "b".to_string();
    let k = "a".to_string();
    let c = "b".to_string();

    assert_eq!(e.transform_message(&m, &k, Action::Encrypt), c);
    assert_eq!(e.transform_message(&c, &k, Action::Decrypt), m);

}

#[test]
fn transform_char() {
    let mut e = Encoding::new();
    e.insert_char('a');
    e.insert_char('b');
    e.insert_char('c');
    e.insert_char('d');
    let m = "b".to_string();
    let k = "c".to_string();
    let c = "d".to_string();

    assert_eq!(e.transform_message(&m, &k, Action::Encrypt), c);
    assert_eq!(e.transform_message(&c, &k, Action::Decrypt), m);

}

#[test]
fn encrypt_decrypt() {
    let mut e = Encoding::new();
    e.insert_char('a');
    e.insert_char('b');
    e.insert_char('c');
    e.insert_char('d');
    let m = "add".to_string();
    let k = "bad".to_string();
    let c = "bdc".to_string();

    assert_eq!(e.encrypt(&m, &k), c);
    assert_eq!(e.decrypt(&c, &k), m);
}

#[test]
fn map_string() {
    let mut e = Encoding::new();
    e.insert_char('a');
    e.insert_map('A', 'a');
    let pre = "aAa".to_string();
    let post = "aaa".to_string();
    assert_eq!(e.map_string(&pre), post);
}

#[test]
fn filter_string() {
    let mut e = Encoding::new();
    e.insert_char('a');
    e.insert_char('b');
    e.insert_map('A', 'a');
    let pre = "Abc".to_string();
    let post = "b".to_string();
    assert_eq!(e.filter_string(&pre), post);
}
