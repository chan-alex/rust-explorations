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


    // Although variables are immutable by default. you can always redfine it with let with
    // a new value. E.g. for x1 above:
    let x1 = 9;
    println!("Value of x1 is now {}", x1);


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
