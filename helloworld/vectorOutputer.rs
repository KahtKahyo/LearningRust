#![allow(unused)]
#![allow(non_snake_case)]

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
    let v = vec![1,2,3,4,5];
    for i in 0..v.len(){
        print!("{} ", v[i].to_string());
        if i == v.len() - 1 {
            print!("-> ");
        }
    }

    let greeting = String::from("Hello");

    let y = &greeting;

    println!("{}!", y);
}
