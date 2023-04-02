// mod_within.rs

use food::Cake;

mod food
{
    pub struct Cake;
    struct Smoothie;
    struct Pizza;
}

fn main()
{
    let _ = Cake;
}
