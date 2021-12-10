#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use anyhow::{anyhow, Result};
use std::cmp::{min, max};
use std::convert::{TryFrom, TryInto};
use std::collections::*;
use std::hash::Hash;
use std::iter::FromIterator;

pub fn xor_byte(v: &mut [u8], b: u8) {
    for x in v.iter_mut() {
        *x ^= b;
    }
}

fn score_plaintext(v: &[u8]) -> usize {
    let mut res = 0;
    for x in v.iter() {
        if x.is_ascii_alphabetic() {
            res += 1;
        }
    }
    res
}

fn main() -> Result<()> {
    let input = hex::decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736").unwrap();

    let mut bestsco = 0;
    let mut bestres = Vec::new();

    for i in 0u8..=255 {
        let mut xored = input.clone();
        xor_byte(&mut xored, i);

        let sco = score_plaintext(&xored);
        if sco > bestsco {
            bestsco = sco;
            bestres = xored;
        }
    }

    println!("Ok: {}", std::str::from_utf8(&bestres).unwrap());
    Ok(())
}
