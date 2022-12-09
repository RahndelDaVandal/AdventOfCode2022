#![allow(non_snake_case)]

use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn main() {
    let input = calarie_inventory();

    let mut inv: Vec<i32> = Vec::new();
    let mut t: i32 = 0;

    for line in input.lines() {
        let l = line.unwrap();

        if l == "" {
            inv.push(t);
            t = 0;
        } else {
            let n: i32 = l.parse().unwrap();
            t = t + n;
        }
    }
    inv.sort();
    let inv_len = inv.len();
    let max_inv = inv.get(inv_len - 1).unwrap();
    let max_three = inv.get(inv_len - 1).unwrap() + inv.get(inv_len - 2).unwrap() + inv.get(inv_len - 3).unwrap();

    println!("Day 01 Part 01: {max_inv}");
    println!("Day 01 Part 02: {max_three}");

}

fn calarie_input() -> std::fs::File {
    match File::open("CalarieInput") {
        Ok(f) => { f },
        Err(e) => { panic!("Error reading CalarieInput: {e}"); }
    }
}

fn calarie_inventory() -> std::io::BufReader<std::fs::File> {
    BufReader::new(calarie_input())
}
