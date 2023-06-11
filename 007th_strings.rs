#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;


// string
fn main() {
    let mut st11 = String::new();
    st11.push('A');
    st11.push_str(" word");
    for word in st11.split_whitespace(){
        println!("{}", word);
    }
    let st22 = st11.replace("A", "Another");
    println!("{}", st22);

    let st3 = String::from("x y w e q w e t i");
    let mut v1: Vec<char> = st3.chars().collect();
    v1.sort();
    v1.dedup();
    for char in v1 {
        println!("{}", char);
    }
    let st4: &str = "Random string";
    let mut st5: String = st4.to_string();
    println!("{}", st5);
    let byte_arr1 = st5.as_bytes();
    let st6 = &st5[0..6];
    print!("String length {}", st6.len());
    st5.clear();

    let st6 = String::from("Just some");
    let st7 = String::from(" words");
    let st8 = st6 + &st7;
    for char in st8.bytes() {
        println!("{}", char);
    }
}
