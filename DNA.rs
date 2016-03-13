use std::io::{BufReader,BufRead};
use std::fs::File;
use std::collections::HashMap;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut nts = HashMap::new();
    let file = File::open(&args[1]).unwrap();
    for line in BufReader::new(file).lines() {
        for c in line.unwrap().chars() {
          let counter = nts.entry(c).or_insert(0);
          *counter += 1;
        }
    }  
    for nt in &['A', 'C', 'G', 'T'] {
      match nts.get(nt) {
        Some(count) => print!("{} ", count),
        None        => print!("")
      }
    }
}
