pub fn add(left: usize, right: usize) -> usize
{
    left + right
}

pub fn pow(base: i64, exponent: usize) -> i64
{
    let mut res = 1;
    
    if exponent == 0
    {
        return 1;
    }

    for _ in 0 .. exponent
    {
        res *= base as i64;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }


    #[test]
    fn negative_two_raised_three_is_negative_eight()
    {
        assert_eq!(pow(-2, 3), -8);
    }
}
