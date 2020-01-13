use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Key {
  A,
  B,
  C,
  D,
  E,
  F,
  G,
  H,
  I,
  J,
  K,
  L,
  M,
  N,
  O,
  P,
  Q,
  R,
  S,
  T,
  U,
  V,
  W,
  X,
  Y,
  Z,
  WHITESPACE,
  OTHER,
  UNKNOWN,
}

const ENGLISH: [(Key, f32, &str); 29] = [
  (Key::A, 0.05213904, "a"),
  (Key::B, 0.00993984, "b"),
  (Key::C, 0.01738712, "c"),
  (Key::D, 0.02798680, "d"),
  (Key::E, 0.08331536, "e"),
  (Key::F, 0.01583048, "f"),
  (Key::G, 0.01268880, "g"),
  (Key::H, 0.03943104, "h"),
  (Key::I, 0.04464752, "i"),
  (Key::J, 0.00072264, "j"),
  (Key::K, 0.00404232, "k"),
  (Key::L, 0.02651920, "l"),
  (Key::M, 0.01616992, "m"),
  (Key::N, 0.04516104, "n"),
  (Key::O, 0.04770416, "o"),
  (Key::P, 0.01101160, "p"),
  (Key::Q, 0.00068848, "q"),
  (Key::R, 0.03980504, "r"),
  (Key::S, 0.04126080, "s"),
  (Key::T, 0.05834856, "t"),
  (Key::U, 0.01801072, "u"),
  (Key::V, 0.00663224, "v"),
  (Key::W, 0.01370176, "w"),
  (Key::X, 0.00109536, "x"),
  (Key::Y, 0.01167872, "y"),
  (Key::Z, 0.00062688, "z"),
  (Key::WHITESPACE, 0.16666666, "\n\t "),
  (
    Key::OTHER,
    0.0333333,
    "0123456789-_=+()*&^%$#@!~`:;,./<>?|\"\'\\",
  ),
  (Key::UNKNOWN, 0.0, ""),
];

pub struct Distribution {
  pub table: HashMap<Key, f32>,
  lookup: HashMap<char, Key>,
}

impl Distribution {
  pub fn new() -> Self {
    let table = HashMap::new();
    let lookup = HashMap::new();
    Distribution { table, lookup }
  }

  pub fn from(input: &[u8]) -> Self {
    let mut distribution = Distribution::empty();
    let len = input.len() as f32;
    for byte in input {
      let lower_char = (*byte as char).to_ascii_lowercase();
      let key = distribution
        .lookup
        .get(&lower_char)
        .unwrap_or(&Key::UNKNOWN);
      let curr = distribution.table.get(key).unwrap();
      distribution.table.insert(*key, curr + (1.0 / len));
    }
    distribution
  }

  pub fn empty() -> Self {
    let mut distribution = Distribution::new();
    for (key, _freq, chars) in ENGLISH.iter() {
      distribution.table.insert(*key, 0.0);
      for car in chars.chars() {
        distribution.lookup.insert(car, *key);
      }
    }
    distribution
  }

  pub fn english() -> Self {
    let mut distribution = Distribution::new();
    for (key, freq, chars) in ENGLISH.iter() {
      distribution.table.insert(*key, *freq);
      for car in chars.chars() {
        distribution.lookup.insert(car, *key);
      }
    }
    distribution
  }

  pub fn scale(&mut self, n: f32) -> &Self {
    self.table.iter_mut().map(|(_, value)| *value * n);
    self
  }
}

pub fn score(sample: &[u8]) -> f32 {
  let len = sample.len() as f32;
  let mut observed = Distribution::from(&sample);
  let mut english = Distribution::english();
  distance(&observed.scale(len), &english.scale(len))
}

pub fn distance(observed: &Distribution, expected: &Distribution) -> f32 {
  observed.table.keys().fold(0.0, |acc, key| {
    let o = observed.table.get(key).unwrap();
    let e = expected.table.get(key).unwrap();
    acc + chi_squared_i(*o, *e)
  })
}

fn chi_squared_i(observed: f32, expected: f32) -> f32 {
  if expected == 0.0 && observed == 0.0 {
    return 0.0;
  }
  if expected == 0.0 && observed != 0.0 {
    return std::f32::INFINITY;
  }
  (observed - expected).powi(2) / expected
}

fn chi_squared(observed: &Vec<f32>, expected: &Vec<f32>) -> f32 {
  observed
    .iter()
    .zip(expected.iter())
    .fold(0.0, |acc, (&o, &e)| acc + chi_squared_i(o, e))
}

#[test]
fn test_chi_squared_eq() {
  let o = vec![1.0, 2.0, 3.0];
  let e = vec![1.0, 2.0, 3.0];
  assert_eq!(0.0, chi_squared(&o, &e));
}

#[test]
fn test_chi_squared_zero_expected() {
  let o = vec![1.0, 2.0];
  let e = vec![0.0, 2.0];
  let result = chi_squared(&o, &e);
  assert_eq!(std::f32::INFINITY, result);
}

#[test]
fn test_chi_squared_ne() {
  let o = vec![0.0, 1.0];
  let e = vec![1.0, 1.0];
  let result = chi_squared(&o, &e);
  assert_eq!(1.0, result);
}

#[test]
fn test_chi_squared_inverted() {
  let o = vec![1.0, 2.0];
  let e = vec![2.0, 1.0];
  let result = chi_squared(&o, &e);
  assert_eq!(1.5, result);
}

#[test]
fn test_score() {
  let a = "Now that the party is jumping\n";
  let b = "\u{1a};#t <5 t <1t$5& -t=\'t>!9$=:3^";
  assert!(score(a.as_bytes()) < score(b.as_bytes()));
}
