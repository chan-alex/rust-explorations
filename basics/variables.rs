fn main () {

    // Variables are declared with let keyword.
    let x1 = 5;
    println!("Value of x1 is {}", x1);

    // In Rust, variables are immutable by default.
    // This won't compile:
    // x1 = 9;

    // Use mut to make mutable variables.
    let mut x2 = 10;
    println!("Value of x2 is {}", x2);    

    // However if the mutable variable is never modified, the compiler issue warnings (by default).
    // E.g. without the next 2 lines, a "variable does not need to be mutable" warning is
    // issued.
    x2 = 15;
    println!("Value of x2 is {}", x2);

    // You can chhange the value of a mutable variable but you can't change it's type
    // x2 = "this is a string";
    // The above line won't compile because x2 is expected to be of type integer.

    // Although variables are immutable by default. you can always redfine it with let with
    // a new value. This is called shadowing.
    let x1 = 9;
    println!("Value of x1 is now {}", x1);

    
    // Shadowing allows you to change the type of the variable.
    let x1 = "This is a string";
    println!("x1 is now a string: {}", x1);


    
    // Constant are defined using the "const" keyword.
    // Constants must be annotaed with the value. In constrast, you don't have to
    // specify the type for variables as Rust is able to infer the type.
    const C1: u32 = 100;   // u32 is unsigned 32 bit number.
    println!("Value of C1 is {}", C1);

    // Once a constant is defined, the name of constant cannot be used for another variable
    // or constant.
    
    // let C1 = 101;    Won't compile.
    // const C1: u64 = 1000; Won't compile either.

    
}
