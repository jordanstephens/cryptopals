pub fn to_hex(bytes: &[u8]) -> String {
  let charset = Charset::hex();
  let out_bytes = bytes.iter().map(|byte| {
    let high = ((byte & 0xf0) >> 4) as usize;
    let low = (byte & 0x0f) as usize;
    vec![charset.table[high], charset.table[low]]
  }).flatten().collect();
  String::from_utf8(out_bytes).expect("invalid utf8")
}

pub fn to_base64(bytes: &[u8]) -> String {
  let charset = Charset::base64();
  let chunk_size = 3;
  let chunks = bytes.chunks_exact(chunk_size);
  let remainder = chunks.remainder();
  let mut output: Vec<u8> = chunks
    .flat_map(|chunk| convert_chunk(chunk, &charset))
    .collect();

  let remainder_len = remainder.len();
  let pad_len = if remainder_len > 0 {
    chunk_size - remainder_len
  } else {
    0
  };

  if pad_len > 0 {
    let zeros = [0, 0];
    let padding_string = "=".repeat(pad_len);
    let padding_bytes = padding_string.as_bytes().to_vec();
    let last_chunk = [remainder, &zeros[0..pad_len]].concat();
    let start = chunk_size - pad_len + 1;
    let splice_range = start..(start + pad_len);
    let mut last_result = convert_chunk(last_chunk.as_slice(), &charset);
    last_result.splice(splice_range, padding_bytes);
    output.append(&mut last_result);
  }

  String::from_utf8(output).expect("invalid utf8")
}

fn convert_chunk(chunk: &[u8], charset: &Charset) -> Vec<u8> {
  let group: u32 = (chunk[0] as u32) << 16 | (chunk[1] as u32) << 8 | (chunk[2] as u32);

  let a = ((group >> 18) & 0x3f) as usize;
  let b = ((group >> 12) & 0x3f) as usize;
  let c = ((group >> 6) & 0x3f) as usize;
  let d = (group & 0x3f) as usize;

  vec![
    charset.table[a],
    charset.table[b],
    charset.table[c],
    charset.table[d],
  ]
}

struct Charset {
  table: Vec<u8>,
}

impl Charset {
  fn new(chars: &str) -> Self {
    Charset {
      table: String::from(chars)
        .as_bytes()
        .to_vec(),
    }
  }

  fn base64() -> Self {
    Charset::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/")
  }

  fn hex() -> Self {
    Charset::new("0123456789abcdef")
  }
}
