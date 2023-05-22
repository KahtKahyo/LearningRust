#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
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


