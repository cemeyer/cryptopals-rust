use lazy_static::lazy_static;
use std::collections::HashMap;
use std::iter::FromIterator;

// From challenge 2
/// "Xor encrypt" a message with a one-time pad.
pub fn xor_bytes(v1: &mut [u8], v2: &[u8]) {
    assert_eq!(v1.len(), v2.len());

    for (x1, x2) in v1.iter_mut().zip(v2.iter()) {
        *x1 ^= *x2;
    }
}

// From challenge 3
/// Obfuscate a message by XOR with a single byte.
pub fn xor_byte(v: &mut [u8], b: u8) {
    for x in v.iter_mut() {
        *x ^= b;
    }
}

// Challenge 4
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
            return f64::NEG_INFINITY;
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
        return f64::NEG_INFINITY;
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
    //for w in v.split(|x| *x == b' ') {
    //    if w.len() > 0 {
    //        score += 0.005;
    //    }
    //}

    score
}

// Challenge 5
/// XOR a message with some repeating key
pub fn repeating_key_xor(cleartext: &mut [u8], key: &[u8]) {
    for (cb, kb) in cleartext.iter_mut().zip(key.iter().cycle()) {
        *cb ^= *kb;
    }
}

// Challenge 6
/// Compute hamming distance between two bytestrings.
pub fn hamming_distance(a: &[u8], b: &[u8]) -> u32 {
    let mut res = 0;
    assert_eq!(a.len(), b.len());

    for (&aa, &bb) in a.iter().zip(b.iter()) {
        res += (aa ^ bb).count_ones();
    }

    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hamming() {
        assert_eq!(37, hamming_distance(b"this is a test", b"wokka wokka!!!"));
    }
}

pub fn roundup(x: usize, y: usize) -> usize {
    ((x + (y - 1)) / y) * y
}

// dbg!(), but not in verbose mode...
#[macro_export]
macro_rules! dbg2 {
    // NOTE: We cannot use `concat!` to make a static string as a format argument
    // of `eprintln!` because `file!` could contain a `{` or
    // `$val` expression could be a block (`{ .. }`), in which case the `eprintln!`
    // will be malformed.
    () => {
        std::eprintln!("[{}:{}]", std::file!(), std::line!())
    };
    ($val:expr $(,)?) => {
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {
                std::eprintln!("[{}:{}] {} = {:?}",
                    std::file!(), std::line!(), std::stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::dbg2!($val)),+,)
    };
}
