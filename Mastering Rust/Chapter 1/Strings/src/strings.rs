// strings.rs

/**
 * Strings are allocated on the heap, while &str types are usually pointers
 * to an existing string, which could either be on the heap, stack, or a string
 * in the data segment of the compiled object code.
 */
fn main()
{
    let question = "How are you?"; // A &str type
    let person: String = "Bob".to_string();
    let namaste = String::from("नमत"); // unicode!

    println!("{}! {} {}", namaste, question, person);
}
