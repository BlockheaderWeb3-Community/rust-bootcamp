fn fibonacci(n: i32) ->  i32{
    match n {
        0 => 0, 
        1 => 1,
        _ => fibonacci(n-1) + fibonacci(n-2),
    }
}

pub fn main() -> i32 {
    let mut user_number_input = String::new();
    println!("Please enter a number: ");
    std::io::stdin().read_line(&mut user_number_input).unwrap();
    let number_input: i32 = user_number_input.trim().parse().unwrap();

    println!("the fibonacci of {}  is {}", number_input, fibonacci(number_input));
    fibonacci(number_input)
}