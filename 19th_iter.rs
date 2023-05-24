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
    let mut arr_it = [1,2,3,4];
    for val in arr_it.iter(){
        println!("{}", val);
    }
    
    let mut iter1 = arr_it.iter();
    println!("{:?}", iter1.next());
}

