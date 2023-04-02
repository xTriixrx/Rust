pub fn silly_loop()
{
    for _ in 1..1_000_000_000 {};
}

#[cfg(test)]
mod tests
{
    use crate::silly_loop;

    #[test]
    #[ignore]
    fn test_silly_loop()
    {
        silly_loop();
    }
}
