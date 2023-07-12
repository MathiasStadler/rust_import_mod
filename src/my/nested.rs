pub fn function() {
    println!("nested.rs :: called `my::nested::function()`");
}

#[allow(dead_code)]
fn private_function() {
    println!("nested.rs :: called `my::nested::private_function()`");
}
