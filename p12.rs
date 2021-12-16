#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use aes::Aes128;
use block_modes::{BlockMode, Cbc, Ecb};
use block_padding::ZeroPadding;

use anyhow::{anyhow, Result};
use std::cmp::{min, max};
use std::convert::{TryFrom, TryInto};
use std::collections::*;
use std::hash::Hash;
use std::iter::FromIterator;
use rand::{Rng, RngCore};

use cryptopals as c;

// Padding choice doesn't really matter for this attack.  We could also truncate to block boundary.
type Aes128Ecb = Ecb<Aes128, ZeroPadding>;

/// Our oracle takes some controlled plaintext, appends an unknown (target) plaintext, and
/// encrypts in ECB mode with some unknown key.
fn ecb_oracle(controlled_pt: &[u8], unknown_suffix_pt: &[u8], unknown_key: &[u8]) -> Vec<u8> {
    let msglen = controlled_pt.len() + unknown_suffix_pt.len();
    let mut output = vec![0; c::roundup(msglen, 16)];
    output[..controlled_pt.len()].copy_from_slice(controlled_pt);
    output[controlled_pt.len()..msglen].copy_from_slice(unknown_suffix_pt);
    Aes128Ecb::new_from_slices(unknown_key, b"")
        .unwrap()
        .encrypt(&mut output, msglen)
        .unwrap();
    output
}

fn main() -> Result<()> {
    let mut rng = rand::thread_rng();
    let unknown_key = {
        let mut key = [0; 16];
        rng.fill_bytes(&mut key);
        key
    };

    let unknown_pt = "\
        Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkg\
        aGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBq\
        dXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUg\
        YnkK";
    let unknown_pt = base64::decode(unknown_pt).unwrap();

    let mut known_buf = vec![b'A'; 16];
    let mut res = Vec::new();

    for byte in 0..unknown_pt.len() {
        // Solve for byte at index 'byte.'

        let byte_in_block = byte % 16;
        let block_offset = byte - byte_in_block;

        // Rotate the partial known block left one.
        known_buf.remove(0);
        // Just reserve the slot.
        known_buf.push(0xff);

        // Build up known 'last byte' oracle blocks.
        let mut known = HashMap::new();
        for b in 0..=255u8 {
            known_buf[15] = b;
            let encr = ecb_oracle(&known_buf, &unknown_pt, &unknown_key)[..16].to_owned();
            known.insert(encr, b);
        }

        // Oracle, what is the next byte of the unknown string?
        let encr = ecb_oracle(&known_buf[..15 - byte_in_block], &unknown_pt, &unknown_key);
        let encr = encr[block_offset..block_offset+16].to_owned();
        let my_byte = known[&encr];
        res.push(my_byte);
        known_buf[15] = my_byte;
    }

    println!("Ok: {}", String::from_utf8(res).unwrap());
    Ok(())
}
