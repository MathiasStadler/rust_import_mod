mod mod1;
mod mod2; // Works
mod components::header::{call_header};
//use crate::call_header;


fn main() {
    println!("main (main.rs) Hello, world!");
    mod1::mod1fn();
    mod2::mod2fn1();
    header();
}
