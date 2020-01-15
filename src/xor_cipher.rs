use super::distribution::score;
use ordered_float::OrderedFloat;
use rayon::prelude::*;

pub fn encrypt(bytes: &[u8], key: &[u8]) -> Vec<u8> {
  xor(bytes, key)
}

pub fn decrypt(bytes: &[u8], key: &[u8]) -> Vec<u8> {
  xor(bytes, key)
}

fn xor(bytes: &[u8], key: &[u8]) -> Vec<u8> {
  bytes
    .iter()
    .zip(key.iter().cycle())
    .map(|(byte, key)| byte ^ key)
    .collect()
}

#[derive(Debug, Clone)]
pub struct KeyCandidate {
  key: u8,
  score: f32,
  plaintext: Vec<u8>,
}

pub fn find_candidate_keys(ciphertext: &[u8]) -> Vec<KeyCandidate> {
  (0..255u8)
    .into_par_iter()
    .map(|key| {
      let plaintext = decrypt(&ciphertext, &[key]);
      let score = score(&plaintext[..]);
      KeyCandidate {
        key,
        score,
        plaintext,
      }
    })
    .collect()
}

pub fn find_best_key(bytes: &[u8]) -> KeyCandidate {
  find_candidate_keys(&bytes)
    .into_iter()
    .min_by_key(|r| OrderedFloat(r.score))
    .unwrap()
}

#[cfg(test)]
mod tests {
  use super::super::decoder;
  use super::*;
  use std::fs::File;
  use std::io::{prelude::*, BufReader};

  #[test]
  fn test_decrypt() {
    let ciphertext = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let buffer = decoder::from_hex(ciphertext).unwrap();
    let key = "X";
    let plaintext = decrypt(&buffer, key.as_bytes());
    let expected = "Cooking MC\'s like a pound of bacon".as_bytes();
    assert_eq!(plaintext, expected);
  }

  #[test]
  fn test_find_best_key() {
    let ciphertext = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let buffer = decoder::from_hex(ciphertext).unwrap();
    let result = find_best_key(&buffer);
    assert_eq!(result.key, b'X');
  }

  #[test]
  fn test_find_key_from_file() {
    let file = File::open("data/4.txt").unwrap();
    let reader = BufReader::new(file);

    let result: KeyCandidate = reader
      .lines()
      .flat_map(|line| {
        let ciphertext = line.unwrap();
        let buffer = decoder::from_hex(&ciphertext).unwrap();
        find_candidate_keys(&buffer)
      })
      .min_by_key(|result| OrderedFloat(result.score))
      .unwrap();

    let expected = "Now that the party is jumping\n".as_bytes();
    assert_eq!(result.key, 53);
    assert_eq!(result.plaintext, expected);
  }

  #[test]
  fn test_encrypt_repeating_key_xor() {
    let plaintext = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let key = "ICE";
    let expected = decoder::from_hex("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f").unwrap();
    let ciphertext = encrypt(plaintext.as_bytes(), key.as_bytes());
    assert_eq!(ciphertext, expected)
  }
}
