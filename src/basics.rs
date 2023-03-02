//Basics of Rust
//Rust is a statically typed language, which means that it must know the types of all variables at compile time,
//however, the compiler can usually infer what type we want to use based on the value and how we use it.

// Variables in Rust
// You can declare variables using the let, const or static keywords:

let my_variable = 5; // immutable
const MY_CONSTANT: i32 = 5; // constant
static MY_STATIC: i32 = 5; // static

// By default, all variables are immutable, meaning that their value cannot be changed once they are bound to a name.
// You can make a variable mutable by using the mut keyword:

let mut my_variable = 5; // mutable

// Rust convention relies on the following casing conventions:

// Variables: snake_case
// Functions: snake_case
// Files: snake_case
// Constants: SCREAMING_SNAKE_CASE
// Statics: SCREAMING_SNAKE_CASE
// Types: PascalCase
// Traits: PascalCase
// Enums: PascalCase

// Since Rust is statically typed, you'll need to explicitly type variables –
// unless the variable is declared with let and the type can be inferred.

// Funtions in Rust
// You declare functions using the fn keyword:

fn main() {
    // This is a function
}

// Functions return using the return keyword, and you need to explicitly specify the return type of a function, unless the
// return type is an empty tuple ():

fn main() -> () { // Unnecessary return type
    my_function();
}

fn my_function() -> u8 {
    return 0;
}

// Functions also return an expression missing the semi-colon:

fn my_func() -> u8 {
    5
}

// Function parameters are typed using the : syntax:

fn main() {
    let _unused_variable = my_func(10);
}

fn my_func(x: u8) -> i32 {
    x as i32
}

// The underscore (_) prefix is used to indicate that a variable will not be used. 
// This is useful when you want to bind a value to a name, but don't want to use it.
// The "as" keyword asserts the type of the expression provided the type conversion is valid.