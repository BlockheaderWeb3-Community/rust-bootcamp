pub fn main() {
    name();
}

fn name (){
    println!("Enter your name: ");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).unwrap();
    println!("Hello, {}", name);
}