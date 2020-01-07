use std::collections::BTreeMap;

pub struct Distribution {
  pub table: BTreeMap<char, f32>,
}

impl Distribution {
  fn new() -> Self {
    Distribution {
      table: BTreeMap::new(),
    }
  }

  pub fn from(input: &str) -> Self {
    let mut distribution = Distribution::empty();
    let len = input.len() as f32;
    for car in input.chars() {
      let key = car.to_ascii_lowercase();
      if distribution.table.contains_key(&key) {
        distribution.table.insert(
          key,
          distribution.table.get(&key).unwrap_or(&0.0) + (1.0 / len),
        );
      }
    }
    distribution
  }

  pub fn empty() -> Self {
    let keys = "abcdefghijklmnopqrstuvwxyz".chars();
    let mut distribution = Distribution::new();
    for key in keys {
      distribution.table.insert(key, 0.0);
    }
    distribution
  }

  pub fn english() -> Self {
    let keys = "abcdefghijklmnopqrstuvwxyz".chars();
    let frequencies: Vec<f32> = vec![
      0.08167, 0.01492, 0.02202, 0.04253, 0.12702, 0.02228, 0.02015, 0.06094, 0.06966, 0.00153,
      0.01292, 0.04025, 0.02406, 0.06749, 0.07507, 0.01929, 0.00095, 0.05987, 0.06327, 0.09356,
      0.02758, 0.00978, 0.02560, 0.00150, 0.01994, 0.00077,
    ];
    let mut distribution = Distribution::new();
    for (key, freq) in keys.zip(frequencies.into_iter()) {
      distribution.table.insert(key, freq);
    }
    distribution
  }
}
