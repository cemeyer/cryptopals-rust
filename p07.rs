#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use aes::Aes128;
use block_modes::{BlockMode, Ecb, block_padding::NoPadding};

use anyhow::{anyhow, Result};
use std::cmp::{min, max};
use std::convert::{TryFrom, TryInto};
use std::collections::*;
use std::hash::Hash;
use std::iter::FromIterator;

use cryptopals as c;

type Aes128Ecb = Ecb<Aes128, NoPadding>;

fn main() -> Result<()> {
    let b64 = std::fs::read_to_string("input.07")?.replace(&['\r', '\n'][..], "");
    let ciphertext = base64::decode(&b64).unwrap();

    let cipher = Aes128Ecb::new_from_slices(b"YELLOW SUBMARINE", b"").unwrap();
    let plaintext = cipher.decrypt_vec(&ciphertext).unwrap();

    println!("Ok");
    println!("{}", String::from_utf8(plaintext).unwrap());
    Ok(())
}
