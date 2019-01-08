// Single Line Comment:- This is how you do a single line comment in Rust Lang.

// In this code file we will go through how to about the Functions in Rust Lang.

/* Multi Line Comment:- This is how your multi line comment looks like.
   Below Function func_one_parameter accepts 1 parameter x and its of type integer and 32 bit.
*/

fn one_parameter (x: i32 ) {
    println!("This function accepts 1 parameters x");
    println!("The value of x is {} and its 32 bit integer:", x);
}

fn two_parameter (x: i32, y: i32) {
    println!("This function accepts 2 parameters x and y");
    println!("The value of x is {} and its 32 bit integer:", x);
    println!("The value of y is {} and its 64 bit integer:", y);
}

// Rust is Expression based language, so lets look into what is Statement and Expressions?

/*  Statements:- are sets of instructions in Rust Lang when they execute they perform some actions.
                 But Statements never returns any value.
    Expressions:- are also sets of instructions in Rust Lang when they executes they performs some actions
                  and also returns values.
*/

// below is the example of Statement

fn i_am_statement () -> i32 {
    let x = 1;
    x + 100
    }

fn five() -> i32 {
        5
}
// This is main function and its a entry point for any Rust Lang program.

fn main () {
    one_parameter(100);
    two_parameter(110, 120);
    i_am_statement();
    five();
}
