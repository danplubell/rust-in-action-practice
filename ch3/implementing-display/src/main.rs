#![allow(dead_code)]

use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug,PartialEq)]
enum FileState {
    Open,
    Closed,
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl Display for FileState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            FileState::Open => write!(f,"OPEN"),
            FileState::Closed => write!(f,"CLOSED")
        }
    }
}
impl Display for File {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "<{} ({})", self.name, self.state)
    }
}
impl File {
    fn new(name: &str)-> Self {
        Self {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }
}
fn main() {
    let f6 = File::new("f6.txt");
    println!("{:?}", f6);
    println!("{}", f6);
    
}
