use super::decoder;
use super::encoder;
use super::error::Error;

struct Buffer {
  bytes: Vec<u8>,
}

impl Buffer {
  fn new(bytes: Vec<u8>) -> Self {
    Buffer { bytes }
  }

  fn from_hex(hex_str: &str) -> Result<Self, Error> {
    let bytes = decoder::from_hex(hex_str)?;
    Ok(Buffer { bytes })
  }

  fn to_base64(&self) -> String {
    encoder::to_base64(&self.bytes)
  }
}

#[test]
fn test_empty() {
  let buffer = Buffer::new(vec![]);
  let result = buffer.to_base64();
  assert_eq!(result, "")
}

#[test]
fn test_padding1() {
  let string = String::from("AB");
  let bytes = string.as_bytes().to_vec();
  let buffer = Buffer::new(bytes);
  let result = buffer.to_base64();
  assert_eq!(result, "QUI=")
}

#[test]
fn test_padding2() {
  let string = String::from("A");
  let bytes = string.as_bytes().to_vec();
  let buffer = Buffer::new(bytes);
  let result = buffer.to_base64();
  assert_eq!(result, "QQ==")
}

#[test]
fn test_simple() {
  let buffer = Buffer::from_hex("BADA55");
  let result = buffer.unwrap().to_base64();
  assert_eq!(result, "utpV")
}

#[test]
fn test_hex_to_b64() {
  let buffer = Buffer::from_hex("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
  let b64 = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
  let result = buffer.unwrap().to_base64();
  assert_eq!(result, b64)
}
