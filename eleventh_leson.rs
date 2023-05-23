#![allow(unused)]

use std::fmt::Display;
use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn get_sum(x: i32, y: i32){
    println!("{} + {} = {}", x, y, x+y);
}

fn say_hello(){
    println!("Hello");
}

fn get_sum_2(x: i32, y: i32) -> i32 {
    x + y
}

fn get_sum_3(x: i32, y: i32) -> i32 {
    return x + y;
}

fn get_2(x: i32) -> (i32, i32){
    return (x+1, x+2); 
}

fn sum_list(list: &[i32]) -> i32 {
    let mut sum = 0;
    for &val in list.iter(){
        sum += &val;
    }
    sum
}

fn main() {
   get_sum(5, 4); 
   println!("{}", get_sum_2(3, 2));
   println!("{}", get_sum_3(5,6)); 
   let(val_1, val_2) = get_2(42);
   println!("{}, {}", val_1, val_2);
   let num_list = vec![1,2,3,4,5]; 
   println!("Sum of lits {}", sum_list(&num_list));
}

