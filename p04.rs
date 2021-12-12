#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use std::cmp::{min, max};
use std::convert::{TryFrom, TryInto};
use std::collections::*;
use std::hash::Hash;
use std::iter::FromIterator;

use cryptopals as c;

fn main() -> Result<()> {
    let mut best = Vec::new();
    let mut bestsco = 0.;

    for line in std::fs::read_to_string("input.04")?.lines() {
        let mut xored = hex::decode(line).unwrap();
        for b in 0u8..=255u8 {
            c::xor_byte(&mut xored, b);
            let sco = c::score_english_plaintext(&xored);
            if sco > bestsco {
                best = xored.clone();
                bestsco = sco;
            }
            c::xor_byte(&mut xored, b);
        }
    }

    println!("Ok: {:?}", std::str::from_utf8(&best));
    Ok(())
}
