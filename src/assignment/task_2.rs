pub fn main () {
    fibanacci();
}

pub fn fibanacci(){
    let mut n = String::new();
    println!("Enter the number to find the fibanacci value of: ");
    std::io::stdin().read_line(&mut n).unwrap();
    let n: u32 = n.trim().parse().unwrap();
    if n <= 1 {
        let value =   (n - 1) + (n - 2);
        println!("the  fibanacci value of {} is {}", n, value)
    }
    let value =   (n - 1) + (n - 2);
    println!("the  fibanacci value of {} is {}", n, value);
}