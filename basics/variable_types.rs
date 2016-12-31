fn main () {

    println!("Start");


    //Integer types

    //8 bit 
    let x: i8 = 100;    // signed
    let x: u8 = 100;    // unsigned

    //16 bit 
    let x: i16 = 100;    // signed
    let x: u16 = 100;    // unsigned

    //32 bit 
    let x: i32 = 100;    // signed
    let x: u32 = 100;    // unsigned

    //64 bit 
    let x: i64 = 100;    // signed
    let x: u64 = 100;    // unsigned

    //architecture size
    let x: isize = 100;  // signed
    let x: usize = 100;  // unsigned

    // In general, the i32 (also the default) is the fastest. Even for 64 bit platforms.


    //floating types.
    // Rust has two floating types. f32 and f64. Default is f64 because it is almost
    // as fast as f32 while providing more precision.

    // Note: Here no need to find type as Rust will infer it is a floating number and
    // use f64 for x:
    let f = 3.14;

    //
    let f: f32 = 3.14;

    // Unlike C. implicit type conversions are not allowed.
    // let r = f * x;  // This won't compile.


    // Boolean type.
    // A boolean type can have either "true" or "false"

    let b = true;   // inferred/
    let b: bool = false;    


    // Char type
    // Note a char type represents a Unicode value. not just ASCII
    let c = 'x';
    let c: char = 'X';


    // Compound type: tuple
    let t = (10, 100, 'Z');  // inferred. should be (i32, i32, char)
    let t: (i32, i64, char) = (10, 100, 'Z');  // using type annotation to be specific.

    // Accessing tuple by destructing
    let ( x, y, z ) = t;
    println!("x = {}, y = {}, z = {}", x, y, z);

    // Accessing tuple with the period "."
    // Index starts from 0.
    println!("t.1 = {}, t.2 = {}, t.3 = {}", t.0, t.1, t.2);    


    
    // Compound type: arrays
    // Unlike tuple, every element must be of same type.
    // Also the size is fixed.
    let a = [ 1, 2 ,3 ,4 ,5 ];

    // Accessing arrays.
    let a2 = a[2];
    let a5 = a[4];


    // Arrays are bounds checked.
    let a10 = a[10];   // this will compile but throw a warning.
    // The compiler warns "this expression will panic at run-time"
    // When the executable is run, it exits with a panic.
    // "thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 10'"
    
}
