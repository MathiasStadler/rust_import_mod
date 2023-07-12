// Similarly `mod inaccessible` and `mod nested` will locate the `nested.rs`
// and `inaccessible.rs` files and insert them here under their respective
// modules
mod inaccessible;
pub mod nested;

pub fn function() {
    println!("my.rs :: called `my::function()`");
}

fn private_function() {
    println!("my,rs :: called `my::private_function()`");
}

pub fn indirect_access() {
    print!("my.rs  :: called `my::indirect_access()`, that\n> ");

    private_function();
}