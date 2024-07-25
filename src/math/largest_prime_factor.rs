pub fn largest_prime_factor(mut n: u64) -> u64 {
    let mut last_factor: u64;
    if n % 2 == 0 {
        last_factor = 2;
        n = n / 2;
        while n % 2 == 0 {
            n = n / 2;
        }
    } else {
        last_factor = 1;
    }
    let mut factor = 3;
    let mut max_factor = (n as f64).sqrt() as u64;
    while n > 1 && factor <= max_factor {
        if n % factor == 0 {
            last_factor = factor;
            while n % factor == 0 {
                n /= factor;
            }
            max_factor = (n as f64).sqrt() as u64;
        }
        factor += 2;
    }
    if n == 1 {
        return last_factor;
    } else {
        return n;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two() {
        assert_eq!(largest_prime_factor(2), 2);
    }
    #[test]
    fn large_num() {
        assert_eq!(largest_prime_factor(600851475143), 6857);
    }
    #[test]
    fn large_prime() {
        assert_eq!(largest_prime_factor(4391436457), 4391436457);
    }
}
