
fn main() {
    println!("Start");

    
    let i = 3;
    
    //if
    if i < 5 {  // this "if" won't compile if variable is defined.
        println!("i is less than 5.");
    } else {
        println!("i is greater than 5.");
    }

    // One thing to note if that condition test must be a bool.
    // The below won't compile.
    // if i {
    //     println!("i is defined.")
   //  }

    
}
