pub fn converter() -> i32 {
    println!("Enter a number in farenhite below.");
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).unwrap();
    let input: i32 = user_input.trim().parse().unwrap();
    let formula: i32 = (input - 32) * 5/9;
    println!("{}", formula);
    formula
}