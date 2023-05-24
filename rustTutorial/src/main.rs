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

fn main() {
    // Common problems with parallel progamming involve :
    // 1. Thread are accessing data in th wrong order
    // 2. Threads are blocked from executing because of confusion
    // over requirements to proceed with execution 
    
    let thread1 = thread::spawn(|| {
        for i in 1..25 {
            println!("Spawned thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    
    for i in 1..20 {
        println!("Main thread : {}", i);
        thread::sleep(Duration::from_millis(1));   
    }

    thread1.join().unwrap();
}



