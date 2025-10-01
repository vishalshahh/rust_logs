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
    println!("{}", my_string_literal);

    let _new_string: String = String::from("Hello World!"); //immutable string

    let mut s = String::from("Hello");
    s.push_str(", world!"); // push_str appends a literal to a String
    println!("{}", s); // this will print "Hello, world!"

    //Tuples

    let employee_info: (&str, i32) = ("Salary", 50_000);
    println!("{} is equal to {}", employee_info.0, employee_info.1);
    println!("The complete tuple at once {:?}", employee_info);

    let (salary, salary_values) = employee_info; // Destructuring
    println!("{} of the employee is {}", salary, salary_values);

    let salary = employee_info.0;
    let salary_values = employee_info.1;
    println!("{} of the employee is {}", salary, salary_values);

}


