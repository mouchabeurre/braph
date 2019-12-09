use std::io::{self, Write};
use std::io::prelude::*;

type MaxLen = u32;

pub fn parse(args: &[String]) -> Vec<MaxLen> {
  if args.len() > 1 {
    let inputs: Vec<MaxLen> = args
      .iter()
      .skip(1)
      .filter_map(|s| s.parse::<MaxLen>().ok())
      .collect();
    return inputs;
  } else {
    let handle = io::stdin();
    let inputs: Vec<MaxLen> = handle.lock().lines().filter_map(|buff| {
      match buff {
        Ok(line) => {
          let line_inputs: Vec<MaxLen> = line
            .split(|s: char| !s.is_ascii_digit())
            .filter_map(|s| s.parse::<MaxLen>().ok())
            .collect();
          return Some(line_inputs);
        },
        Err(_) => None
      }
    }).flatten().collect();
    return inputs;
  }
}

pub fn build(inputs: Vec<MaxLen>) -> Vec<Vec<u8>> {
  let max = match inputs.iter().max() {
    Some(m) => m.clone(),
    None => 0
  };
  let min = match inputs.iter().min() {
    Some(m) => m.clone(),
    None => 0
  };
  let difference: MaxLen = max - min;
  let levels: MaxLen = 8;
  let bytes = b"\xE2\x96\x81";
  let result: Vec<Vec<u8>> = inputs.iter().map(|input| {
    let height = ((*input as f32 - min as f32) / difference as f32 * levels as f32) as f32;
    let rounded = height.round() as u8;
    return vec![bytes[0], bytes[1], bytes[2] + rounded];
  }).collect();
  result
}

pub fn write(symbols: Vec<Vec<u8>>) -> () {
  let flattened: Vec<u8> = symbols.into_iter().flatten().collect();
  io::stdout().write_all(&flattened.as_slice()).unwrap();
}