use std::f64;
use std::io::{self, Write};
use std::io::prelude::*;

pub fn parse(args: &[String]) -> Vec<f64> {
  if args.len() > 1 {
    let inputs: Vec<f64> = args
      .iter()
      .skip(1)
      .filter_map(|s| s.parse::<f64>().ok())
      .collect();
    return inputs;
  } else {
    let handle = io::stdin();
    let inputs: Vec<f64> = handle
      .lock()
      .lines()
      .filter_map(|buff| {
      match buff {
        Ok(line) => {
          let mut raw_inputs: Vec<String> = Vec::new();
          let mut chars = line.chars();
          let mut current_num: String = String::new();
          while let Some(current_char) = chars.next() {
            match current_char {
              '-' => {
                current_num.push(current_char);
              },
              '.' => {
                current_num.push(current_char);
              },
              '0' ..= '9' => {
                current_num.push(current_char);
              },
              _ => {
                raw_inputs.push(current_num.clone());
                current_num = String::new();
              }
            }
          }
          if current_num.len() > 0 {
            raw_inputs.push(current_num.clone());
          }

          let line_inputs: Vec<f64> = raw_inputs
            .iter()
            .filter_map(|s| s.parse::<f64>().ok())
            .collect();
          return Some(line_inputs);
        },
        Err(_) => None
      }
    }).flatten().collect();
    return inputs;
  }
}

pub fn build(inputs: Vec<f64>) -> Vec<Vec<u8>> {
  let max = inputs.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
  let min = inputs.iter().fold(f64::INFINITY, |a, &b| a.min(b));
  let difference: f64 = max - min;
  let levels: f64 = 8.0;
  let bytes = b"\xE2\x96\x81";
  let result: Vec<Vec<u8>> = inputs.iter().map(|input| {
    let height = (*input - min) / difference * (levels - 1.0);
    let rounded = height.round() as u8;
    return vec![bytes[0], bytes[1], bytes[2] + rounded];
  }).collect();
  result
}

pub fn write(symbols: Vec<Vec<u8>>) -> () {
  let flattened: Vec<u8> = symbols.into_iter().flatten().collect();
  io::stdout().write_all(&flattened.as_slice()).unwrap();
}