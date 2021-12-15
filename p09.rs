#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use block_padding::{Padding, Pkcs7};

use anyhow::{anyhow, Result};
use std::cmp::{min, max};
use std::convert::{TryFrom, TryInto};
use std::collections::*;
use std::hash::Hash;
use std::iter::FromIterator;

use cryptopals as c;

fn main() -> Result<()> {
    let input = b"YELLOW SUBMARINE";

    let mut output = [0; 20];
    output[..16].copy_from_slice(input);
    Pkcs7::pad_block(&mut output, 16).unwrap();

    assert_eq!(&output, b"YELLOW SUBMARINE\x04\x04\x04\x04");

    println!("Ok");
    Ok(())
}
