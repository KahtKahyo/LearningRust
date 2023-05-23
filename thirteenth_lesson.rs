#![allow(unused)]

use std::fmt::Display;
use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::ops::Add; //for generic

// Stack : Stores values in a last in first out format
// Data on the stack must have a defined fixed size 
// Heap :  When putting data on the heap you request 
// certain amount of space. The OS finds space available 
// and returns an address for that space called a pointer

// RULES:
// 1. Each value has a variable that's called its owner
// 2. There is only one owner at a time
// 3. When the owner goes out of the scope the value disappear

fn print_str(x: String){
    println!("A string {}", x);

}

fn print_return_str(x: String) -> String{
    println!("A string {}", x);
    x
}

fn change_string(name: &mut String){
    name.push_str(" is happy");
    println!("Message: {}", name);
}

fn main() {
    // first test, if you try str2 = str 1
    // then print out str 1, this will cause an error
    // since the str1 will not hold any value anymore

    let mut str1 = String::from("My Name");
    // let str2 = str1.clone();
    // let str3 = print_return_str(str1);

    change_string(&mut str1);

    // println!("str3 = {}", str3);
}


