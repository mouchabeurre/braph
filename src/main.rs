use std::env;
use braph;

fn main() {
  let args: Vec<String> = env::args().collect();
  let inputs = braph::parse(&args);
  let output = braph::build(inputs);
  braph::write(output);
}