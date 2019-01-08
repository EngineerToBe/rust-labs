fn main() {
    println!("Hello, world!");
    i_am_if_cond();
    multiple_if_cond();
    i_am_while_loop();
}

// If/Else condition in RUST Lang

fn i_am_if_cond (){
    let x = 5;
    if x < 5 {
    println!("The value of x is {}", x);
}
    else {
        println!("Go To Sleep");
    }

}

// All if conditional statements always be boolean. If they are not boolean RUST compiler will throw
// error.

// Multiple If/Else conditions in RUST Lang.

fn multiple_if_cond () {
    let number = 256;
    if number % 3 == 0 {
        println!("Hey there! The number is divisible by 3");
    }
    else if number % 4 == 0 {
        println!("Hey there! The number is divisible by 4");

    }
    else if number % 5 == 0 {
        println!("Hey there! The number is divisible by 5");
    }
    else {
        println!("i gave up")
    }
}

// There are three types of Loops in RUST Language.
/*  * loop
    * while
    * for
*/

fn i_am_while_loop() {
    let mut number = 45;
    while number != 0 {
        println!("{}!", number);
        number = number -1;

    }
    println!("Yes! Please SHUT")
}
