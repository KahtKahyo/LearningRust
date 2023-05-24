#![allow(unused)]

use std::fmt::Display;
use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::ops::Add; //for generic
use std::collections::HashMap; //for HashMap

//to use mod.rs at restaurant crate at main.rs
mod restaurant;

use crate::restaurant::order_food;

fn main() {
    order_food(); 
}