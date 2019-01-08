fn main() {
    println!("Hello, world!");
}

/* IN RUST Lang variable are immutable by defualt which means at any point the value of the variables
will not change.

But just in case if you want to define a mutable variables you can do it
*/

// Variables have scope in RUST Lang.
// Immutable variable

let name = "abhishek";

// Mutable variable

let mut name = "abhishek"

// Noticed that `mut` thats the game changer it makes possible to define the Mutable variables in RUST Lang.


// Just like variable we do have constants in RUST Lang.

const NAME: u32 = 100_101;

// Rust’s naming convention for constants is to use all uppercase with underscores between words, and underscores can be inserted in numeric literals to improve readability
// Constansts in RUST are immutable in the scope they defined in.
