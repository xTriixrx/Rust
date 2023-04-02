// for_loops.rs

fn main()
{
    // not inclusive
    print!("Normal range: ");

    for i in 0 .. 10
    {
        print!("{},", i);
    }
    println!();

    print!("Inclusive range: ");
    for i in 0 ..= 10
    {
        print!("{},", i);
    }
}