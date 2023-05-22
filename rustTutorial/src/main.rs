#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    let arr_4 = [0,1,2,3,4,5,6,7,8,9];
    let mut loop_index = 0;
    while loop_index < arr_4.len() {
        println!("Arr: {}", arr_4[loop_index]);
        loop_index += 1;
    }

}


