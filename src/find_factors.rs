//  O(sqrt(num))
pub fn find_factors(mut num: i32) -> Vec<i32> {
    let mut factors: Vec<i32> = Vec::new();
    while num % 2 == 0 {
        factors.push(2);
        num /= 2
    }
    let mut i = 3;
    // если num = i * x, то i || x <= sqrt(num)
    let mut max_factor = f32::sqrt(num as f32);
    while i <= max_factor as i32 {
        while num % i == 0 {
            factors.push(i);
            num /= i;
            max_factor = f32::sqrt(num as f32);
        }
        i += 2;
    }
    if num > 1 {
        factors.push(num);
    }
    factors
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_simple_number() {
        assert_eq!(find_factors(127), [127]);
    }

    #[test]
    fn test_complex_number() {
        assert_eq!(find_factors(126), [2,3,3,7]);
    }
}