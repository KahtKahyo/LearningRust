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
    // struct generic, trait
    struct Rectangle<T, U> {
        length: T,
        height: U,
    }

    let rec = Rectangle {
        length: 10,
        height: 10.5
    };

}