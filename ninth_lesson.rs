#![allow(unused)]

use std::fmt::Display;
use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    //enumerated types
    enum Days {
        Sunday,
        Monday, 
        Tuesday, 
        Wednesday, 
        Thursday, 
        Friday,
        Saturday
    }

    impl Days{
        fn is_weekend(&self) -> bool {
            match self{
                Days::Saturday | Days::Sunday => true,
                _ => false
            }
        }
    }
    let today:Days = Days::Monday;
    match today {
        Days::Monday => println!("Everyone hates monday"),
        Days::Tuesday => println!("Donut day"),
        Days::Wednesday => println!("Hump day"),
        Days::Thursday => println!("Pay day"),
        Days::Friday => println!("Almost Weekend"),
        Days::Saturday => println!("Weekend!"),
        Days::Sunday => println!("Weekend!"),
    }

    println!("Is today the weekend {}", today.is_weekend());
}
