//! This module has numerous helpers for creating
//! some basic encoders.

#![allow(dead_code)]
use super::Encoding;

fn add_num(e: &mut Encoding) {
    for i in 0..9 {
        e.insert_char(char_num('1', i));
    }
}

fn add_az(e: &mut Encoding) {
    for i in 0..26 {
        e.insert_char(char_num('a', i));
    }
}

fn add_space(e: &mut Encoding) {
    e.insert_char(' ');
}

#[allow(non_snake_case)]
fn map_AZ(e: &mut Encoding) {
    for i in 0..26 {
        e.insert_map(char_num('A', i), char_num('a', i));
    }
}

fn map_whitespace(e: &mut Encoding) {
    e.insert_map('\n', ' ');
    e.insert_map('\t', ' ');
}

fn char_num(base: char, offset: u8) -> char {
    (offset + base as u8) as char
}

pub fn alphanumeric() -> Encoding {
    let mut e: Encoding = Encoding::new();
    add_az(&mut e);
    map_AZ(&mut e);
    add_num(&mut e);
    e
}

pub fn alphanumeric_space() -> Encoding {
    let mut e: Encoding = Encoding::new();
    add_az(&mut e);
    map_AZ(&mut e);
    add_num(&mut e);
    add_space(&mut e);
    map_whitespace(&mut e);
    e
}

pub fn alpha() -> Encoding {
    let mut e: Encoding = Encoding::new();
    add_az(&mut e);
    map_AZ(&mut e);
    e
}

pub fn alpha_space() -> Encoding {
    let mut e: Encoding = Encoding::new();
    add_az(&mut e);
    map_AZ(&mut e);
    add_space(&mut e);
    map_whitespace(&mut e);
    e
}
