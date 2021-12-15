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

/// AES-CBC-128 encrypt some plaintext blocks.
fn cbc_encrypt(inout: &mut [u8], key: &[u8], iv: &[u8]) {
    assert_eq!(key.len(), 16);
    assert_eq!(iv.len(), 16);
    assert!(inout.len() % 16 == 0);

    let mut prev_ct = iv;
    let cipher = Aes128Ecb::new_from_slices(key, b"").unwrap();

    for block in inout.chunks_exact_mut(16) {
        c::xor_bytes(block, prev_ct);
        cipher.clone().encrypt(block, 16).unwrap();
        prev_ct = block;
    }
}

/// AES-CBC-128 decrypt some ciphertext blocks.
fn cbc_decrypt(inout: &mut [u8], key: &[u8], iv: &[u8]) {
    assert_eq!(key.len(), 16);
    assert_eq!(iv.len(), 16);
    assert!(inout.len() % 16 == 0);

    let mut prev_ct = iv.to_owned();
    let cipher = Aes128Ecb::new_from_slices(key, b"").unwrap();

    for block in inout.chunks_exact_mut(16) {
        let next_ct = block.to_owned();
        cipher.clone().decrypt(block).unwrap();
        c::xor_bytes(block, &prev_ct);
        prev_ct = next_ct;
    }
}

fn main() -> Result<()> {
    let b64 = std::fs::read_to_string("input.10")?.replace(&['\r', '\n'][..], "");
    let ciphertext = base64::decode(&b64).unwrap();
    let mut plaintext = ciphertext.clone();

    cbc_decrypt(&mut plaintext, b"YELLOW SUBMARINE", &[0; 16]);
    println!("{}", std::str::from_utf8(&plaintext).unwrap());
    println!("Ok");
    Ok(())
}
