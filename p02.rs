#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use anyhow::{anyhow, Result};
use std::cmp::{min, max};
use std::convert::{TryFrom, TryInto};
use std::collections::*;
use std::hash::Hash;
use std::iter::FromIterator;

use cryptopals as c;

fn main() -> Result<()> {
    let mut input = hex::decode("1c0111001f010100061a024b53535009181c").unwrap();
    let input2 = hex::decode("686974207468652062756c6c277320657965").unwrap();

    c::xor_bytes(&mut input, &input2);

    assert_eq!(hex::encode(&input), "746865206b696420646f6e277420706c6179");
    println!("Ok");
    Ok(())
}
