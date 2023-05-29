#![allow(unused)]

use std::fmt::Display;
use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::ops::Add; //for generic
use std::collections::HashMap; //for HashMap
use std::thread; //for threads
use std::time::Duration; 
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};  
use std::fmt::Debug; 

fn main() {
    let v = vec![1,2,3,4,5,6,7,8,9,10];
    for p in 0..v.len(){
        println!("{}", p);
    }
}



