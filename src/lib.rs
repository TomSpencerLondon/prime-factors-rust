#[allow(dead_code)]
fn prime_factors(num: i64) -> Vec<i64>{
    if num % 2 == 0 && num > 2 {
        [vec![2], prime_factors(num / 2)].concat()
    } else {
        vec![num]
    }
}

#[cfg(test)]
mod tests {
    use crate::prime_factors;

    #[test]
    fn prime_factors_of_two() {
        assert_eq!(prime_factors(2), [2]);
    }

    #[test]
    fn prime_factors_of_three() {
        assert_eq!(prime_factors(3), [3]);
    }

    #[test]
    fn prime_factors_of_four() { assert_eq!(prime_factors(4), [2, 2]); }
}

