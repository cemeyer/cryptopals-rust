#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use anyhow::{anyhow, Result};
use std::cmp::{min, max};
use std::convert::{TryFrom, TryInto};
use std::collections::*;
use std::hash::Hash;
use std::iter::FromIterator;

use cryptopals as c;

fn main() -> Result<()> {
    let b64 = std::fs::read_to_string("input.06")?.replace(&['\r', '\n'][..], "");
    let ciphertext = base64::decode(&b64).unwrap();

    // First, identify likely key size by XOR'ing chunks of each size.  I think the assumption here
    // is that the plaintext likely has low edit distance, and the right size XOR will eliminate
    // the XOR key.  Would not work if the plaintext was compressed (high entropy -- high likely
    // edit distance), I think.
    let mut distances = Vec::new();
    for guess_keysize in 2..41 {
        // It turns out this parameter is super important for choosing the right block size, and
        // '2' is really bad at identifying the correct key size (29).  10 is enough to put 29
        // first.
        let sample_blocks = 10;
        let prefixes = ciphertext.chunks_exact(guess_keysize).take(sample_blocks).collect::<Vec<_>>();
        let distance = prefixes.windows(2).map(|win| c::hamming_distance(win[0], win[1])).sum::<u32>();
        let normalized_dist = (distance as f64) / ((sample_blocks * guess_keysize) as f64);
        distances.push((normalized_dist, guess_keysize));
    }
    // (f64 technically isn't Ord)
    distances.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

    // First 2-3 sizes (if we only look at the first 2 chunks) end up being 5 bytes, 3 bytes, and 2
    // bytes WHICH ARE ALL WRONG.
    println!("{:?}", &distances[..6]);

    let mut best_res = Vec::new();
    let mut best_rkey = Vec::new();
    let mut best_rkey_sco = f64::NEG_INFINITY;

    // We could actually just try them all?  Instead of artificially ruling out the real size
    // (depending on block sampling above).  Whatever.
    let consider_candidates = 3;
    // For each keysize candidate...
    for (_, guess_keysize) in distances[..consider_candidates].iter().cloned() {
        // Transpose ciphertext by key byte.
        let mut transposed = Vec::new();
        for _ in 0..guess_keysize {
            transposed.push(Vec::new());
        }
        for chunk in ciphertext.chunks(guess_keysize) {
            for (i, b) in chunk.iter().enumerate() {
                transposed[i].push(*b);
            }
        }

        // Find each key byte one at a time (e.g., O(3 * 256) instead of O(256*256*256)).
        let mut key = Vec::new();
        for i in 0..guess_keysize {
            let mut best = 0;
            let mut bestsco = f64::NEG_INFINITY;
            for b in 0u8..=255u8 {
                c::xor_byte(&mut transposed[i], b);
                let sco = c::score_english_plaintext(&transposed[i]);
                if sco > bestsco {
                    bestsco = sco;
                    best = b;
                }
                c::xor_byte(&mut transposed[i], b);
            }
            key.push(best);
        }

        // Compute (possible) plaintext for this repeating key and score it.
        let mut ct_copy = ciphertext.clone();
        c::repeating_key_xor(&mut ct_copy, &key);
        let sco = c::score_english_plaintext(&ct_copy);
        if sco > best_rkey_sco {
            best_rkey = key.clone();
            best_res = ct_copy;
            best_rkey_sco = sco;
        }
    }

    println!("KEY: {} (KEYSIZE={}) PLAINTEXT:", hex::encode(&best_rkey), best_rkey.len());
    println!("{}", String::from_utf8(best_res).unwrap());
    Ok(())
}
