#![allow(unused)]

use std::fmt::Display;
use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::ops::Add; //for generic
use std::collections::HashMap; //for HashMap

fn main() {
    let can_vote = |age: i32| {
        age >= 18
    };
    println!("{}", can_vote(8));

    let mut sampl = 5; 
    let print_var = || println!("sampl = {}", sampl);
    print_var();
    sampl = 10;
    let mut change_var = || sampl += 1;
    change_var();
    println!("sampl = {}", sampl);
    sampl = 10;
    println!("sampl = {}", sampl);
}
