#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use aes::Aes128;
use block_modes::{BlockMode, Cbc, Ecb};
use block_padding::NoPadding;

use anyhow::{anyhow, Result};
use std::cmp::{min, max};
use std::convert::{TryFrom, TryInto};
use std::collections::*;
use std::hash::Hash;
use std::iter::FromIterator;
use rand::{Rng, RngCore};

use cryptopals as c;

type Aes128Cbc = Cbc<Aes128, NoPadding>;
type Aes128Ecb = Ecb<Aes128, NoPadding>;

fn roundup(x: usize, y: usize) -> usize {
    ((x + (y - 1)) / y) * y
}

/// Encrypts under a random key, random IV, and 50-50 ECB or CBC.
fn random_encrypter(data: &[u8]) -> Vec<u8> {
    let mut rng = rand::thread_rng();

    let mut key = [0; 16];
    rng.fill_bytes(&mut key);

    let pad1 = (rng.next_u32() as usize % 6) + 5;
    let pad2 = (rng.next_u32() as usize % 6) + 5;
    let totallen = roundup(pad1 + data.len() + pad2, 16);

    let mut output = vec![0; totallen];
    let msglen = output.len();
    output[pad1..(pad1 + data.len())].copy_from_slice(data);

    let is_ecb_mode = (rng.next_u32() % 2) == 0;
    if is_ecb_mode {
        Aes128Ecb::new_from_slices(&key, b"")
            .unwrap()
            .encrypt(&mut output, msglen)
            .unwrap();
    } else {
        let mut iv = [0; 16];
        rng.fill_bytes(&mut iv);

        Aes128Cbc::new_from_slices(&key, &iv)
            .unwrap()
            .encrypt(&mut output, msglen)
            .unwrap();
    }

    output
}

#[derive(Debug, Eq, PartialEq, Hash)]
enum Mode {
    Cbc,
    Ecb,
}

fn main() -> Result<()> {
    let chosen_pt = vec![b'A'; 16*100];

    let mut distrib = HashMap::<_, usize>::new();
    for _trial in 0..100 {
        let enc = random_encrypter(&chosen_pt);

        // Detect ECB through chosen plaintext with at least 98 identical blocks.  The odds of
        // getting so many repeated blocks with CBC are zero.
        let mut detect_dupes = HashMap::<_, usize>::new();
        let mut detect_mode = Mode::Cbc;
        for block in enc.chunks_exact(16) {
            let x = detect_dupes.entry(block.to_owned()).or_default();
            *x += 1;
            if *x >= 98 {
                detect_mode = Mode::Ecb;
                break;
            }
        }

        *distrib.entry(detect_mode).or_default() += 1;
    }
    dbg!(distrib);

    println!("Ok");
    Ok(())
}
