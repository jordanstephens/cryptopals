use super::error::Error;

pub fn from_hex(input: &str) -> Result<Vec<u8>, Error> {
  let len = input.len();

  if len % 2 != 0 {
    return Err(Error::DecodeHexError);
  }

  let input: &[u8] = input.as_ref();
  input
    .chunks(2)
    .map(|pair| Ok(hex_val(pair[0])? << 4 | hex_val(pair[1])?))
    .collect()
}

fn hex_val(c: u8) -> Result<u8, Error> {
  match c {
    b'A'..=b'F' => Ok(c - b'A' + 10),
    b'a'..=b'f' => Ok(c - b'a' + 10),
    b'0'..=b'9' => Ok(c - b'0'),
    _ => Err(Error::DecodeHexError),
  }
}
