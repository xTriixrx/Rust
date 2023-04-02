// hashmaps.rs

use std::collections::HashMap;

fn main()
{
    let mut fruits = HashMap::new();

    fruits.insert("Apple", 3);
    fruits.insert("Mango", 6);
    fruits.insert("Orange", 2);
    fruits.insert("Avocado", 7);

    for (key, value) in &fruits
    {
        println!("I got {} {}.", value, key);
    }

    fruits.remove("Orange");

    let old_avocado = fruits["Avocado"];
    fruits.insert("Avocado", old_avocado + 5);

    println!("I now have {} Avocados!", fruits["Avocado"]);
}