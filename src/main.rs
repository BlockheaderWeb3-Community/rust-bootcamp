/**
    Rust Bootcamp
*/
// Modules are a fundamental building block for organizing and structuring Rust code.
// Modules help break down complex code into smaller, more manageable units.
mod week_one;
mod week_two;

// To avoid typing long module paths, you can use the use keyword to bring items into scope.
use crate::week_one::{
    day_one,
    day_three::{factorial, guessing_game, is_prime},
    day_two,
};

use crate::week_two::{
    day_one::{christmas_song, fibonacci, temp_converter},
    day_two::calculator::{addition, division, multiplication, subtraction},
};

// The main function in Rust serves as the entry point for your program's execution.
fn main() {
    day_one::main();
    day_two::main();
    println!("{}", factorial(12));
    println!("{}", is_prime(17));
    println!("{}", guessing_game());
    println!("{}", fibonacci(30));
    println!("{}", temp_converter());
    christmas_song();
    println!("{}", addition(10, 20));
    println!("{}", subtraction(40, 20));
    println!("{}", multiplication(10, 20));
    println!("{}", division(100, 5));
}
