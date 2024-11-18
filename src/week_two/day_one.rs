/**
    Rust Bootcamp Week Two: Day One
    - Conditionals
    - Pattern matching
    - Loops
*/

/**
    Conditionals in Rust allow you to execute different code blocks based on specific conditions.
    The primary conditional construct in Rust is the `if` expression.
*/

/**
    Loops in Rust are used to execute a block of code repeatedly until a certain condition is met.
    Rust provides three primary types of loops: `loop`, `while`, and `for`.

    1. `loop` Loop:
        - Creates an infinite loop that continues indefinitely unless explicitly broken.
        - Use the break keyword to exit the loop.
        - Often used for continuous processing or waiting for specific events.

    2. `while` Loop:
        - Executes a block of code as long as a given condition is true.
        - The condition is checked before each iteration.

    3. `for` Loop:
        - Iterates over a sequence of values, often from a range or a collection.
        - It's commonly used to iterate over arrays, vectors, and other iterable data structures.
*/

/**
    Rust's match statement is a powerful control flow construct that allows you to
    match a value against a series of patterns and execute code based on the first
    matching pattern. It's often used for pattern matching, error handling, and more
    complex control flow scenarios.
*/
use std::io::stdin;

// Convert temperatures between Fahrenheit and Celsius.
pub fn temp_converter() -> f64 {
    let temp = process_temp_input();
    let unit = process_unit_input();
    if unit == 'C' {
        far_to_cel(temp)
    } else {
        cel_to_far(temp)
    }
}

fn process_temp_input() -> f64 {
    println!("Enter the temperature value you want to convert");
    loop {
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(e) => {
                println!("Error occured reading your input {e}");
                continue;
            }
        };
        match input.trim().parse::<f64>() {
            Ok(n) => return n,
            Err(e) => {
                println!(
                    "Error {e} occured parsing your input,\
                    \nPlease provide a valid integer character"
                );
                continue;
            }
        }
    }
}

fn process_unit_input() -> char {
    println!(
        "Enter the unit you want to convert to\
        C for farenheit to celcius\
        F for celcius to farenheit"
    );
    loop {
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(e) => {
                println!("Error occured reading your input {e}");
                continue;
            }
        };
        match input.trim().parse::<char>() {
            Ok(n) if n == 'C' || n == 'F' => return n,
            Ok(_) => {
                println!(
                    "Please input a valid temperature unit.\
            Valid units are\
            \nC for Celcius\
            \nF for Farenheit"
                );
                continue;
            }
            Err(e) => {
                println!("An error {e} occured parsing your input, please try again");
                continue;
            }
        }
    }
}

fn cel_to_far(celcius: f64) -> f64 {
    ((celcius * 9.0) / 5.0) + 32.0
}

fn far_to_cel(farenheit: f64) -> f64 {
    ((farenheit - 32.0) * 5.0) / 9.0
}

// Generate the nth Fibonacci number.
pub fn fibonacci(n: u128) -> u128 {
    // Using an if statement, we check if the sequence to be generated is either of the base case,
    // if it is, we return the sequence back to the caller.
    if n <= 1 {
        return n;
    }

    // Fn = Fn-1 + Fn-2
    // prev = Fn-2
    // next = Fn-1
    let mut prev = 0;
    let mut next = 1;

    // Using a for loop and a range, we iterate until we get to the sequence.
    // We perform add and swap operations;
    for _ in 2..=n {
        // Temp is a variable that acts as a temporary store before the value is swapped.
        let temp = next;
        // We perform an `AddAssign` operation here
        // next = next + prev
        next += prev;
        // We update prev to point to the value stored in temp
        prev = temp;
    }

    // At the end of the loop, we return next; the number in the fibonacci sequence.
    next
}

// Print the lyrics to the Christmas carol “The Twelve Days of Christmas,”
// taking advantage of the repetition in the song.
pub fn christmas_song() {
    // Program initializes with a message
    println!("Enter the day of christmas");

    // A variable to store user input from standard input.
    let mut day = String::new();

    // Match arm to check the possible variants of reading from stdin
    // Ok => if the operation was successful.
    // Err => if the operation failed.
    // Result<T, E>
    // https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html?highlight=result#recoverable-errors-with-result
    match stdin().read_line(&mut day) {
        Ok(_) => (),
        Err(e) => {
            println!("Error reading from stdin: {e}");
            panic!();
        }
    };

    // User input is stored as a `String` in the program, to convert it to usize
    // we use the parse method, and the turbofish syntax specifying the type we want to parse to.
    let day = match day.trim().parse::<usize>() {
        // We use match guard here, to further improve our program's ergonomics.
        Ok(n) if n <= 12 => n,
        Ok(_) => {
            println!("Error {day} must be less than or equal to 12");
            panic!();
        }
        Err(e) => {
            println!("Error converting {day} to integer: {e}");
            panic!();
        }
    };

    // An array of string literals storing the gifts
    let gifts = [
        "And a partridge in a pear tree!",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    // An array of string literals storing the days
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eight", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    println!(
        "On the {} day of Christmas,\nmy true love gave to me",
        days[day]
    );

    // A for loop picking every element in the range and using it to index the gifts array.
    // The iterator is reversed to print the gifts in descending order.
    for i in (0..day).rev() {
        println!("{}", gifts[i]);
    }
}
