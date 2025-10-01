// Integer Types 

fn main() {
    let num:u8 = 10; //immutable variable
    println!("num is {}", num);

    // Swapping two variables program
    let mut a:u8 = 2;
    let mut b:u8 = 3;
    let c:u8;

    c = a;
    a = b;
    b = c;
    println!("The value of a is {} and b is {}", a, b);

    // learning string literals
    // Important Note:
    // &str is a fixed size string literal
    // String is a dynamic size string

    // Creating a string_literal
    let my_string_literal = "Vishal is a software engineer";
    println!("{}", my_string_literal)
}


