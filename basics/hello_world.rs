
use std::io;   

fn main() {
    // this is how comments are written.

    // print!() is a macro.
    println!("Hello world!");

    // Why is such a simple thing a macro in Rust?
    // One reason is to safely allow variadic arguments which Rust functions
    // currently don't allow.
    // Other reasons see: https://users.rust-lang.org/t/newbie-why-macros-vs-functions/1012
   

    println!("What is your name?");

    let mut input = String::new();

    // "let" create a new variable blinding for variable input
    // "mut" specifies that this variable is mutable.
    // Without "mut", the variable cannot be modified.
    // String::new() create a growable UTF-8 string. new() is something like a static method.

    io::stdin().read_line(&mut input)
        .expect("Failed to read input!");

    // std::io::stdin returns a handle to the standard input of the terminal.
    // .read_line() calls the read_line function on this handle.
    // This function has the signature "&mut String".
    // &mut means mutable reference.

    // std::io::stdin.read_line returns an value of type io::Result
    // io::Result has a function .expect() that panics with the given message on error.

    println!("Hello there, {}", input);
}
