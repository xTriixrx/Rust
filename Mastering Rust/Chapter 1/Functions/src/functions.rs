// functions.rs

fn add(a: u64, b: u64) -> u64
{
    // Automatically returns last statement, can also use return w/ semicolon
    // return a + b;
    a + b
}

fn main()
{
    let a: u64 = 17;
    let b = 3;
    let result = add(a, b);
    println!("Result: {}", result);
}
