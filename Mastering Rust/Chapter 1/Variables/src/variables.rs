// variables.rs

fn main()
{
    // By default, all variables are immutable, to make them mutable
    // you must explicitly declare the variable as mutable.
    // let target: &str = "world"; // This is a compile error
    
    let mut target: &str = "world";
    let mut greeting: &str = "Hello";

    println!("{}, {}", greeting, target);

    greeting = "How are you doing";
    target = "mate";

    println!("{}, {}", greeting, target);
}
