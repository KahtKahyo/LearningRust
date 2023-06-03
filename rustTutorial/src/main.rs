#![allow(unused)]
#![allow(non_snake_case)]

use std::fmt::Display;
use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::cmp::PartialOrd; 
use std::ops::Add; //for generic
use std::collections::HashMap; //for HashMap
use std::thread; //for threads
use std::time::Duration; 
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};  
use std::fmt::Debug; 

#[derive(Debug)]

pub struct LinkedList<T>(Option<(T, Box<LinkedList<T>>)>);

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList(None)
    }

    pub fn push_front(&mut self, data: T) {
        let t = self.0.take();
        self.0 = Some((data,Box::new(LinkedList(t))));
    }

    pub fn push_back(&mut self, data: T){
        match  self.0 {
            Some((_, ref mut child)) => child.push_back(data),
            None => self.push_front(data),
        }
    }
}

fn main() {
    let mut ll = LinkedList::new();

    ll.push_front(1);
    ll.push_back(12);
    ll.push_front(15);

    println!("ll = {:?}", ll);
}



