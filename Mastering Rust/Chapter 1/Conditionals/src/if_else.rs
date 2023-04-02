// if_else.rs

/**
 * The if construct is not a statement but rather an expression. Meaning the
 * if else construct always returns a value in Rust. The value may be an empty
 * () unit type, or it may be an actual value. Whatever remains in the last line
 * inside the braces becomes the return value of the if else expression. It is
 * important to note that both if and else branches should have the same return value.
 */
fn main()
{
    let rust_is_awesome = true;

    if rust_is_awesome
    {
        println!("Indeed.");
    }
    else
    {
        println!("Well, you should try Rust!");    
    }
}
