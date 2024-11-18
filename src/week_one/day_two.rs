/**
    Rust Bootcamp Day Two
    - Introduction to Memory
    - Deep dive into Heap and Stack
    - Introduction to Rust syntax and fundamental concepts
    - Variables, data types, and basic operations.
*/

// Static variables are allocated memory at compile time
// and their memory remains fixed throughout the entire program's execution.
// `Data Segment`
// PS: Underscore before a variable name typically indicates that the variable is intentionally unused.
static _FIRST: &str = "Rust";

// When a function is called, a stack frame is created on the call stack.
// This frame holds:
// - Local Variables
// - Function arguments
// - Return address
pub fn main() {
    // Variables are binded to a value in rust,
    // This makes them immutable by default.
    let _value: i32 = 12;
    // You can change what a variable is binded to by shadowing the variable
    let _value: u8 = 255;
    // You can make a variable mutable by specifying the `mut` keyword
    let mut _mutable: i32 = 10;
    _mutable = 20;

    // For Integers, the default type is i32
    // Unsigned: u8, u16, u32, u64, u128
    // Signed: i8, i16, i32, i64, i128
    // `&` symbol refers to Referencing, taking a pointer reference
    // `*` symbol refers to Dereferencing, accesing the value behind the pointer.
    let x: i64 = 5;
    let y = &x;
    let z = &y;
    let sum = **z + 10;
    println!("{}", sum);

    // For Float, the default type is f64
    // There's also f32
    let _fl = 2.6;

    // Boolean Data types
    // True or False
    let _is_bool: bool = false;

    // Tuples
    // Unit, tuple, triple, quadruple, ...
    let tup: (i32, &str) = (1, "Papi Chulo");
    let (index, name) = tup;
    println!("{}, is seated at index: {} by my right", name, index);
    println!("{}, is seated at index: {} by my right", tup.1, tup.0);

    // Static Arrays
    // Arrays which their size cannot be mutated at runtime
    let _class: [i32; 11] = [1; 11];

    // Heap allocation is used for dynamic memory allocation,
    // but it's generally less efficient due to the overhead of memory management.
    // Heap allocated data return
    // - A Pointer to where the value is stored on the heap
    // - Capacity: The total amount of memory allocated for a data structure.
    // - Length:  The number of elements currently stored in a data structure.
    //
    // Aside: When the length of a data structure exceeds its capacity, a reallocation occurs.
    // This involves allocating a larger block of memory and copying the existing elements to the new block.
    let mut name = String::from("Colab");
    println!(
        "{name}, capacity: {}, length: {}",
        name.capacity(),
        name.len()
    );
    name.push_str(" Innovation Campus");
    println!(
        "{name}, capacity: {}, length: {}",
        name.capacity(),
        name.len()
    );
    let mut vect: Vec<i32> = Vec::new();
    println!(
        "Vector: {:#?}, Capacity: {}, Length: {}",
        vect,
        vect.capacity(),
        vect.len()
    );
    vect.push(1);
    vect.push(1);
    vect.push(1);
    println!(
        "Vector: {:#?}, Capacity: {}, Length: {}",
        vect,
        vect.capacity(),
        vect.len()
    );
}

fn _five() -> u8 {
    5
}

fn _sum(a: i32, b: i32) -> i32 {
    let c: u8 = 12;
    i32::from(c) + a + b
}

// String -> String::new(),
// &str -> "Rust" string literals.
// String::from()
