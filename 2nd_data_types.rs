#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    const ONE_MIL: u32 = 1_000_000;
    const PIE: f32 = 3.141592;
    let age = "19";
    let mut age: u32 = age.trim().parse()
    .expect("Age wasn't assigned a number");
    age = age + 1;
    println!(" I'm {} and i want ${}", age, ONE_MIL);
}
