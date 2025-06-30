fn main() {
    println!("# CHAPTER 3, SECTION 3: Functions\n");
    println!("Hello, world!");
    another_function(5);
}

fn another_function(x: i32) {
    println!("This is from within a function named `another_function()`.");
    println!("The value of `x` is {x}.");
}