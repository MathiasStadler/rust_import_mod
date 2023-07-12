mod components;
mod pages;
mod mod1;
mod mod2; // Works

use components::header::*;
use mod1::*;
use mod2::*;

fn main() {
    println!("main (main.rs) Hello, main!");
    mod1::mod1fn1();
    mod2::mod2fn1();
    components::header::header();
    use_header();
    components::header::use_header();
    mod1fn1();
    mod2fn1();
    pages::home_pages::page();
}
