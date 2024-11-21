pub fn main() {
    dynamic_converter();
}
pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * (5.0 / 9.0 )
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * (9.0 / 5.0) + 32.0
}

fn dynamic_converter(){
    println!("Enter the temperature to convert: usinng C for Celsius and F for Fahrenheit");
    let mut temp_type = String::new();
    std::io::stdin().read_line(&mut temp_type).unwrap();
    let temp_type: String = temp_type.trim().to_uppercase().parse().unwrap();

    println!("Enter the temperature value: ");
    let mut temp= String::new();
    std::io::stdin().read_line(&mut temp).unwrap();
    let temp: f64 = temp.trim().parse().unwrap();
    if temp_type == "C" {
        println!("{} celsius is {} fahrenheit", temp, celsius_to_fahrenheit(temp));
    } else if temp_type == "F" {
        println!("{} fahrenheit is {} celsius", temp, fahrenheit_to_celsius(temp));
    } else {
        println!("Invalid temperature type");
    }
}

