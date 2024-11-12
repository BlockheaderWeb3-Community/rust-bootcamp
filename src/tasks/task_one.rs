    fn main() {
        dynamic_converter();
    }
    
    pub fn converter_to_fnh(user_input: i32) -> i32 {
        let formula: i32 = (user_input - 32) * 5/9;
        formula
    }
    
    pub fn converter_to_cls(user_input: i32) -> i32 {
        let formula: i32 =  (user_input * 9/5) + 32;
        formula
    }
    
    // dynamic converter
    pub fn dynamic_converter() {
        println!("Enter a number below.");
        let mut user_input_number = String::new();
        std::io::stdin().read_line(&mut user_input_number).unwrap();
        let input_number: i32 = user_input_number.trim().parse().unwrap();
    
    
        println!("Enter C for celcius or F for farenhite (this is the unit)");
        let mut user_input_temp = String::new();
        std::io::stdin().read_line(&mut user_input_temp).unwrap();
        let input_temp: String = user_input_temp.trim().to_uppercase().parse().unwrap();
    
        if input_temp == "C" {
            let result: i32 = converter_to_cls(input_number);
            println!("Hey, {} degrees celcius in degrees farenhite is: {} ", input_number, result);
        } else if input_temp == "F" {
            let result: i32 = converter_to_fnh(input_number);
            println!("Hey, {} degrees farenhite in degrees celcius is: {}", input_number, result);
        } else {
            println!("NOT RECOGNIZED, PLS TRY AGAIN");
            dynamic_converter();
        }
    }