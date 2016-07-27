use std::collections::HashMap;
use std::fmt;
use toml::{self, Array, Value, Table};


const ALPHANUMERIC: &'static str = r#"
        alphabet =[
            "a",
            "b",
            "c",
            "d",
            "e",
            "f",
            "g",
            "h",
            "i",
            "j",
            "k",
            "l",
            "m",
            "n",
            "o",
            "p",
            "q",
            "r",
            "s",
            "t",
            "u",
            "v",
            "w",
            "x",
            "y",
            "z",
            "1",
            "2",
            "3",
            "4",
            "5",
            "6",
            "7",
            "8",
            "9",
            "0",
            " ",
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


#[derive(Debug)]
pub enum Action {
    Encrypt,
    Decrypt,
}

custom_derive! {
    #[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
    #[derive(NewtypeFrom, NewtypeAdd, NewtypeSub, NewtypeRem)]
    pub struct EncodeNum(u64);
}

impl fmt::Display for EncodeNum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn transform(message: &EncodeNum, key: &EncodeNum, size: &usize, action: &Action) -> EncodeNum {
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

#[derive(Debug)]
pub struct Encoding {
    char_number_map: HashMap<char, EncodeNum>,
    number_char_map: HashMap<EncodeNum, char>,
    char_char_map: HashMap<char, char>,
    size: usize,
}

fn char_from_toml_value(value: &Value) -> char {
    match value {
        &Value::String(ref s) => {
            if s.len() != 1 {
                panic!("Invalid config, key 'alphabet' has bad char value");
            } else {
                let c: char = s.chars().nth(0).unwrap();
                c
            }
        }
        _ => panic!("Invaid config, key 'alphabet' had non string value."),
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

    fn new_from_toml(toml: &str) -> Self {
        // TODO Error handling
        let mut new_encoding = Self::new();
        let root_table: Table = toml::Parser::new(toml).parse().unwrap();
        trace!("Root Table: {:?}", root_table);
        let alphabet = match root_table.get("alphabet") {
            Some(&Value::Array(ref abc)) => abc,
            _ => panic!("Invaid config, key 'alphabet' did not have array."),
        };
        let chars: Vec<char> = alphabet.iter()
            .map(|x| char_from_toml_value(x))
            .collect();
        for c in &chars {
            new_encoding.insert_char(*c);
        }



        new_encoding
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn insert_char(&mut self, c: char) {
        let map_number = EncodeNum(self.size as u64);
        self.char_number_map.insert(c, map_number);
        self.number_char_map.insert(map_number, c);
        self.size = self.size + 1;
    }

    pub fn insert_map(&mut self, x: char, y: char) {
        self.char_char_map.insert(x, y);
    }

    pub fn char_in_working_set(&self, c: &char) -> bool {
        self.char_number_map.contains_key(c)
    }

    pub fn char_to_number(&self, c: &char) -> EncodeNum {
        *self.char_number_map.get(c).unwrap()
    }

    pub fn number_to_char(&self, n: &EncodeNum) -> char {
        *self.number_char_map.get(n).unwrap()
    }

    pub fn map_char(&self, c: &char) -> char {
        *self.char_char_map.get(c).unwrap_or(c)
    }
}


pub fn short_abc() -> Encoding {
    let mut e = Encoding::new();
    e.insert_char('a');
    e.insert_char('b');
    e.insert_char('c');
    e.insert_map('A', 'a');
    e.insert_map('B', 'b');
    e.insert_map('C', 'c');
    e
}


pub fn alphanumeric() -> Encoding {
    let alphanumeric_encoding: Encoding = Encoding::new_from_toml(ALPHANUMERIC);
    println!("Alphanumeric Encoding {:?}", alphanumeric_encoding);
    alphanumeric_encoding
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
