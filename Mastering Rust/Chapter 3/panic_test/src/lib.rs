#[cfg(test)]
mod tests
{
    #[test]
    #[should_panic]
    fn this_panics() {
        assert_eq!(1, 2);
    }
}
