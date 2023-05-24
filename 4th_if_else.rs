#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
//comparisson 
// if else
 let age: i32 = 8;
 if (age >= 1) && (age <= 18){
    println!("Birthday coming soon!");
 } else if (age == 21) || (age == 50){
    println!("Important Birthday");
 } else if age >= 65 {
    println!("Important Birthday");
 } else {
    println!("Not an Important Birthday");
 }

// tenary
 let mut my_age = 19;
 let can_vote = if my_age >= 18 {
     true
 } else {
     false
 };
 println!("Can Vote : {}", can_vote);

 // match
 let age2 = 8;
 match age2 {
    1..=18 => println!("Important Birthday"),
    22 | 50 => println!("Important Birthday"),
    // check every integer more than 65 up to the max interger of i32
    65..=i32::MAX => println!("Important Birthday"),
    // any value is going to return: "Not important Birthday"
    _ => println!("Not Important Birthday"), 
 }; 
// another match
match my_age.cmp(&voting_age){
    Ordering::Less => println!("Can't Vote"), 
    Ordering::Greater => println!("Can Vote"),
    Ordering::Equal => println!("You gained the right tp vote"),
    
}

}
