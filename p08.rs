#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use anyhow::{anyhow, Result};
use std::cmp::{min, max};
use std::convert::{TryFrom, TryInto};
use std::collections::*;
use std::hash::Hash;
use std::iter::FromIterator;

use cryptopals as c;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.08")?
        .lines()
        .map(|line| hex::decode(line).unwrap())
        .collect::<Vec<_>>();

    for (line, ciphertext) in input.iter().enumerate() {
        // Histogram blocks, to find duplications.
        let mut this_repeats = HashMap::<_, usize>::new();
        for chunk in ciphertext.chunks(16) {
            *this_repeats.entry(chunk.clone()).or_default() += 1;
        }

        // Total duplicates
        let mut count = 0;
        for c in this_repeats.values() {
            count += (*c - 1);
        }

        // Found it.
        if count > 0 {
            dbg!(line, count);
        }
    }

    println!("Ok");
    Ok(())
}
