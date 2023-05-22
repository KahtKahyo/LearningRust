#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    let arr_1 = [1, 2, 3, 4];
    println!("1st: {}", arr_1[0]);
    println!("2nd: {}", arr_1[1]);
    println!("3rd: {}", arr_1[2]);
    println!("4th: {}", arr_1[3]);

    let arr_2 = [1,2,3,4,5,6,7,8,9,10,11,12];
    println!("Length: {}", arr_2.len());

    let arr_3 = [1,2,3,4,5,6,7,8,9,10,11,12];
    let mut loop_idx = 0;
    loop {
        if arr_3[loop_idx] % 2 == 0 {
            loop_idx +=1;
            continue;
        }
        if arr_3[loop_idx] == 11{
            break;
        }
        println!("Val: {}", arr_3[loop_idx]);
        loop_idx +=1; 

    }
    
}
