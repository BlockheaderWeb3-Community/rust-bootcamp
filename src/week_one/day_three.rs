/**
    Rust Bootcamp Day Two
    - Primitive Data Types
    - String Slice
    - Booleans
    - Integers
    Hands-on: simple exercises to reinforce concepts
*/

// Write a program to calculate the factorial of a number.
pub fn factorial(n: i32) -> i32 {
    // 1. Base case: Factorial of 0 and 1 is 1
    if n <= 1 {
        return n;
    }

    // 2. Recursive case: Factorial of n is n * factorial(n-1)
    n * factorial(n - 1)
}

// Create a program to check if a given number is prime.
pub fn is_prime(n: i32) -> bool {
    // 1. Handle base cases: numbers less than or equal to 3 are prime
    if n <= 3 {
        return true;
    }

    // 2. Check divisibility by numbers from 2 to the square root of n
    // for i in 2..((n as f64).sqrt() as i32) + 1 {
    for i in 2..n {
        if n % i == 0 {
            // 3. If n is divisible by any number in the range, it's not prime
            return false;
        }
    }

    // 4. If no divisors are found, the number is prime
    true
}

// Implement a simple guessing game.
pub fn guessing_game() -> bool {
    // 1. Declare a constant `magic_number`
    let magic_number = 17;

    // 2. Create a mutable string to store user input
    let mut guess_input = String::new();

    println!("Enter your guess below.");
    // 3. Read a line of input from the user and store it in `guess_input`
    std::io::stdin().read_line(&mut guess_input).unwrap();

    // 4. Convert the input string to an integer and store it in `guess`
    let guess: i32 = guess_input.trim().parse::<i32>().unwrap();

    // 5. Compare the `guess` with the `magic_number` using a match expression
    match guess.cmp(&magic_number) {
        // 6. If the guess is equal to the magic number, return true
        std::cmp::Ordering::Equal => true,
        // 7. Otherwise, return false
        _ => false,
    }
}
