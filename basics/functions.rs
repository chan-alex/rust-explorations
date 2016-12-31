

fn main() {

    println!("Start");

    function_1();
    function_2(200);  // if no argument suppliedd, this code will not compile.
    function_3(100, 150);

    //The difference between statement and expressions.
    // (Also the important of semicolons.
    // Statement do something but don't return a value.
    // Expression evaluate to a value.
    let y = 1;   // this is a statement. does not return any value.

    // Calling functions and macros are expressions.
    // Blocks like {} also return values

    let y = {  let i = 100;
               let h = 100;
               h + i    //  <-- notice no semi colon here. Putting one here turns this
                        //      in a statement.
    };


    // Important: Expressions have no semi colons.

    println!("The value of y is: {}", y);    


    let r = return_10();
    println!("return_10() returns {}", r);


    println!("add1(20) returns {}", add1(20));
}


// Functions can be defined before or after they are used.
// Order does not matter.
// If a function is defined but never used, the compiler will warn about dead code.
fn function_1() {

    println!("This is from function_1()");
    
}


// Function argument need to help their types defined.
// If the argument is defined but never used, compiler will warn about unused variables.
fn function_2( i:u32 ) {

    println!("From function_2(), i is {}", i);

}

// Example of function with more than 1 arguments.
fn function_3( x:u32, y: u32) {

    println!("From function_3(), x is {}, x is {}", x, y);

}



// function with return value.
fn return_10 () -> i32 {

    10   // notice: no semi colon at the end.
        
}

fn add1 (x: i32) -> i32 {

    x + 1  // not semi colon.
    //x + 1;  <-- putting a semil colon will cause a compiler error.
    // "not all control paths return a value"    

}
