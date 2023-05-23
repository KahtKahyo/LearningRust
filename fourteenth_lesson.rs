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
    // HashMap
    let mut heroes = HashMap::new();
    heroes.insert("Superman", "Clark Kent");
    heroes.insert("Batman", "Bruce Wayne");
    heroes.insert("Flash", "Barry Allen");

    for (k, v) in heroes.iter() {
        println!("Hero: {}, Name: {}", k, v);
    }

    // println!("Length: ", heroes.len());
    if heroes.contains_key(&"Batman") {
        let the_batman = heroes.get(&"Batman");
        match the_batman{
            Some(x) => println!("Batman is a hero!")
            None => println!("Batman is not a hero!"),
        }
    }
}


