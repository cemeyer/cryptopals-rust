#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use anyhow::{anyhow, Result};
use std::cmp::{min, max};
use std::convert::{TryFrom, TryInto};
use std::collections::*;
use std::hash::Hash;
use std::iter::FromIterator;

pub fn xor_bytes(v1: &mut [u8], v2: &[u8]) {
    assert_eq!(v1.len(), v2.len());

    for (x1, x2) in v1.iter_mut().zip(v2.iter()) {
        *x1 ^= *x2;
    }
}

fn main() -> Result<()> {
    let mut input = hex::decode("1c0111001f010100061a024b53535009181c").unwrap();
    let input2 = hex::decode("686974207468652062756c6c277320657965").unwrap();

    xor_bytes(&mut input, &input2);

    assert_eq!(hex::encode(&input), "746865206b696420646f6e277420706c6179");
    println!("Ok");
    Ok(())
}
