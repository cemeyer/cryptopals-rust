#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use anyhow::{anyhow, Result};
use std::cmp::{min, max};
use std::convert::{TryFrom, TryInto};
use std::collections::*;
use std::hash::Hash;
use std::iter::FromIterator;

use cryptopals as c;

fn main() -> Result<()> {
    let mut message =
        b"Burning 'em, if you ain't quick and nimble\n\
        I go crazy when I hear a cymbal".to_owned();

    c::repeating_key_xor(&mut message, b"ICE");

    assert_eq!(
        hex::encode(&message),
        "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272\
        a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f",
        );

    println!("Ok");
    Ok(())
}
