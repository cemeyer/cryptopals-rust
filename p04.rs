#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use std::cmp::{min, max};
use std::convert::{TryFrom, TryInto};
use std::collections::*;
use std::hash::Hash;
use std::iter::FromIterator;

use cryptopals as c;

lazy_static! {
    static ref ENGLISH_FREQ_TAB: HashMap<u8, f64> =
        HashMap::from_iter([
            (b'a', 8.167/100.0),
            (b'b', 1.492/100.0),
            (b'c', 2.782/100.0),
            (b'd', 4.253/100.0),
            (b'e', 12.70/100.0),
            (b'f', 2.228/100.0),
            (b'g', 2.015/100.0),
            (b'h', 6.094/100.0),
            (b'i', 6.966/100.0),
            (b'j', 0.153/100.0),
            (b'k', 0.772/100.0),
            (b'l', 4.025/100.0),
            (b'm', 2.406/100.0),
            (b'n', 6.749/100.0),
            (b'o', 7.507/100.0),
            (b'p', 1.929/100.0),
            (b'q', 0.095/100.0),
            (b'r', 5.987/100.0),
            (b's', 6.327/100.0),
            (b't', 9.056/100.0),
            (b'u', 2.758/100.0),
            (b'v', 0.978/100.0),
            (b'w', 2.360/100.0),
            (b'x', 0.150/100.0),
            (b'y', 1.974/100.0),
            (b'z', 0.074/100.0),
        ]);
}

pub fn score_english_plaintext(v: &[u8]) -> f64 {
    let mut histo = HashMap::new();
    let mut num_letters = 0;
    let mut num_spaces = 0;

    // Build frequency histogram
    for x in v.iter() {
        // We're ignoring the problem of utf-8 for now.
        if !x.is_ascii() {
            return 0.;
        }
        if *x == b' ' {
            num_spaces += 1;
            continue;
        }
        if !x.is_ascii_alphabetic() {
            continue;
        }
        num_letters += 1;
        histo.insert(*x, histo.get(x).unwrap_or(&0) + 1);
    }

    let num_letters = num_letters;
    if num_letters == 0 {
        return 0.;
    }

    let mut score = 0.;
    for (letter, count) in histo.iter() {
        let freq = (*count as f64) / (num_letters as f64);
        score += ENGLISH_FREQ_TAB[&letter.to_ascii_lowercase()] * freq;
    }

    let symbol_freq = 1.0 -
        (((num_letters + num_spaces) as f64) / (v.len() as f64));
    // Favor non-symbol ASCII characters
    score -= 0.5 * symbol_freq;

    // Prefer "words"
    for w in v.split(|x| *x == b' ') {
        if w.len() > 0 {
            score += 0.005;
        }
    }

    score
}

fn main() -> Result<()> {
    let mut best = Vec::new();
    let mut bestsco = 0.;

    for line in std::fs::read_to_string("input.04")?.lines() {
        let mut xored = hex::decode(line).unwrap();
        for b in 0u8..=255u8 {
            c::xor_byte(&mut xored, b);
            let sco = score_english_plaintext(&xored);
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
