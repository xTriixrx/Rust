// function_mut.rs

// Val is only mutable within the context increase function; pass by copy mutable
fn increase(mut val: u32, incr: u32)
{
    val += incr;
    println!("You made {} points.", val);
}

fn main()
{
    let score = 2048;
    increase(score, 52);

    // Score doesn't actually change, only allows change within function scope
    println!("Score: {}", score);
}