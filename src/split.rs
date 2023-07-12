// This declaration will look for a file named `my.rs` and will
// insert its contents inside a module named `my` under this scope
mod my;

fn function() {
    println!("split.rs :: called `function()`");
}

fn main() {
    function();

    my::function();

    my::indirect_access();

    my::nested::function();
}