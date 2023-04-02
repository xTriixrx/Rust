// match_expressions.rs

fn request_status() -> u32
{
    200
}
/**
 * Within a match brace, we write expressions; these are known as match arms.
 * These arms represent the possible values that the variable being matched can
 * take. Each match arm is written by writing the possible value of the variable,
 * followed by a =>, and then the expression on the right. To the right, you can
 * either have a single line expression or a multi line expression within {} braces.
 * When writing a single line expression, they need to be delimited with a comma. Also,
 * every match arm must return the same type.
 * 
 * Another nice feature or guarantee of match expressions is that we have to match
 * exhaustively against all possible cases of the value we are matching against.
 * Rust allows us to either ignore the rest of the possibilities by using a catch all
 * variable (here, this is other) or an underscore (_) if we want to ignore the value.
 * 
 * Like if/else expressions, the return value of a match expression can also be assigned
 * to a variable in a let statement when its delimited with a semicolon, with all match
 * arms returning the same type.
 */
fn main()
{
    let status = request_status();

    match status
    {
        200 => println!("Success!"),
        404 => println!("Not Found!"),
        other => {
            println!("Request failed with code: {}.", other);
            // get response from cache
        }
    }
}