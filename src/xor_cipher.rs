use ordered_float::OrderedFloat;
use rayon::prelude::*;
use super::buffer::Buffer;
use super::distribution::Distribution;

pub fn decrypt(buffer: &Buffer, key: &u8) -> String {
  let out: Vec<u8> = buffer.bytes.iter().map(|byte| byte ^ key).collect();
  String::from_utf8_lossy(out.as_slice()).to_string()
}

#[derive(Debug)]
pub struct FindKeyResult {
  key: u8,
  score: f32,
  plaintext: String
}

pub fn find_key(buffer: &Buffer) -> FindKeyResult {
  (0..256).into_par_iter().map(|i| {
    let key = i as u8;
    let plaintext = decrypt(&buffer, &key);
    let score = score(&plaintext);
    FindKeyResult { key, score, plaintext }
  })
  .min_by_key(|result| OrderedFloat(result.score))
  .unwrap()
}

fn score(sample: &str) -> f32 {
  let english = Distribution::english();
  let distribution = Distribution::from(&sample);
  let len = sample.len() as f32;
  let observed: Vec<f32> = distribution.table.values().map(|val| val * len).collect();
  let expected: Vec<f32> = english.table.values().map(|val| val * len).collect();
  chi_squared(observed, expected)
}

fn chi_squared(observed: Vec<f32>, expected: Vec<f32>) -> f32 {
  observed
    .iter()
    .zip(expected.iter())
    .fold(0.0, |acc, (o, e)| acc + (o - e).powi(2) / e)
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs::File;
  use std::io::{prelude::*, BufReader};

  #[test]
  fn test_decrypt() {
    let text = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let buffer = Buffer::from_hex(text).unwrap();
    let key = b'X';
    let result = decrypt(&buffer, &key);
    assert_eq!(result, "Cooking MC\'s like a pound of bacon");
  }

  #[test]
  fn test_find_key() {
    let ciphertext = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let buffer = Buffer::from_hex(ciphertext).unwrap();
    let result = find_key(&buffer);
    assert_eq!(result.key, b'X');
  }
}
