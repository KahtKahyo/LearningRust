
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
    fn use_func<T>(a: i32, b: i32, func: T) -> i32 
    where T: Fn(i32, i32) -> i32 {
        func(a,b)
    }
    let sum = |a, b| a+b;
    let prod = |a, b| a*b;
    println!("5 + 4 = {}", use_func(5,4, sum));
    println!("5 * 4 = {}", use_func(5,4, prod));
}
