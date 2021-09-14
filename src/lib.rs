#[allow(dead_code)]
fn prime_factors(mut num: i64) -> Vec<i64>{
    let mut result = vec![];
    let mut i = 2;
    while num > 1 {
        while num % i == 0 {
            result.push(i);
            num /= i;
        }
        i += 1;
    }
    result
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

