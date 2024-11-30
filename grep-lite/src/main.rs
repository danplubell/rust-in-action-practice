use regex::Regex;
use clap::{App, Arg};
fn main() {
    let args = App::new("grep-lite");
    let search_term = "picture";
    let quote = "\
Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books.
What do we seek through millions of pages?";     // <1>
    let re = Regex::new("picture").unwrap();
    for line in quote.lines() {         
        let contains_substring = re.find(line);// <2>
        if let Some(_) = contains_substring { println!("{}", line) }
    }
}