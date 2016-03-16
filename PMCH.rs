use std::io::{BufReader,BufRead};
use std::fs::File;
use std::collections::HashMap;
use std::env;


fn factorial(n: &u64) -> u64 {
    (1..n+1).fold(1, |p, n| p*n)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut nts = HashMap::new();
    let file = File::open(&args[1]).unwrap();
    for line in BufReader::new(file).lines().skip(1) {
        for c in line.unwrap().chars() {
          let counter = nts.entry(c).or_insert(0);
          *counter += 1;
        }
    }
    println!("{}", (factorial(nts.get(&'A').unwrap())) * factorial(nts.get(&'G').unwrap()));
}
