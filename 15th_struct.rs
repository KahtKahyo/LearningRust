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
    // struct
    struct Customer{
        name: String,
        address: String,
        balance: f32,
    }

    let mut bob = Customer{
        name: String::from("Bob Junior"),
        address: String::from("555 main st"),
        balance: 23456.12
    };
    bob.address = String::from("505 Main st");
    // struct with generic
        struct Rectangle<T, U> {
            length: T,
            height: U,
        }
    
        let rec = Rectangle {
            length: 10,
            height: 10.5
        };
}


