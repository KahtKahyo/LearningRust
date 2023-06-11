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
    pub struct Bank {
        balance: f32

    } fn withdraw(the_bank: &mut Bank, amt: f32){
        the_bank.balance -= amt;
    }
    let mut bank = Bank{balance: 100.0};
    withdraw(&mut bank, 5.00);
    println!("Balance: {}", bank.balance);
    fn customer(the_bank: &mut Bank){
        withdraw(the_bank, 5.00);
    }
    thread::spawn(move || {
        customer(&mut bank)
    }).join().unwrap();
}

