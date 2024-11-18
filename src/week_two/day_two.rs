/**
    Rust Bootcamp Week two: Day Two
    - Writing Functions
    - Organizing code into modules
*/

/**
    Functions are reusable blocks of code that perform a specific task.
    They help in organizing your code, making it more modular and easier to understand and maintain.
*/

/**
    - Modules group together related code, and let you control what is a private implementation
      detail and what should be public interface.
    - Crates are a tree of modules that produce a library or an executable.
    - Paths are used to reference a function, struct, enum, etc... from some other module or crate.
    - A package contains a library crate, one or more binary crates, or both.
    - Workspaces let you group together a set of related packages, similar to a monorepo.
*/

// fn sum(a: i32, b: i32) -> i32
// A function is declared with the `fn` keyword
// sum is the function name.
// The parentheses after the sum holds the parameters
// The return type is specifed after the parentheses with the `->` symbol.
fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn sub(a: i32, b: i32) -> i32 {
    a - b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn divide(a: i32, b: i32) -> i32 {
    match b == 0 {
        true => 0,
        false => a / b,
    }
}

pub mod calculator {
    // The `path` operator `::` is used to access functions, struct in the module they're defined.
    use super::{divide, multiply, sub, sum};
    pub fn addition(a: i32, b: i32) -> i32 {
        sum(a, b)
    }

    pub fn subtraction(a: i32, b: i32) -> i32 {
        sub(a, b)
    }

    pub fn multiplication(a: i32, b: i32) -> i32 {
        multiply(a, b)
    }

    pub fn division(a: i32, b: i32) -> i32 {
        divide(a, b)
    }
}
