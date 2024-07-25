fn is_prime(n: u64) -> bool {
    if n == 1 {
        return false;
    } else if n < 4 {
        return true; // 2 and 3 are prime
    } else if n % 2 == 0 {
        return false;
    } else if n < 9 {
        return true; //we have already excluded 4,6 and 8
    } else if n % 3 == 0 {
        return false;
    } else {
        let r = (n as f64).sqrt().floor() as u64;
        let mut f = 5;
        while f <= r {
            if n % f == 0 {
                return false;
            }
            if n % (f + 2) == 0 {
                return false;
            }
            f += 6;
        }
        return true;
    }
}

pub fn nth_prime(n: u64) -> u64 {
    if n == 1 {
        return 2;
    }
    let mut count = 2;
    let mut candidate = 3;
    while count != n {
        candidate += 2;
        if is_prime(candidate) {
            count += 1;
        }
    }
    candidate
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        assert_eq!(nth_prime(1), 2);
    }
    #[test]
    fn second() {
        assert_eq!(nth_prime(2), 3);
    }
    #[test]
    fn prime_true() {
        assert_eq!(is_prime(4391436457), true);
    }
    #[test]
    fn prime_false() {
        assert_eq!(is_prime(4391436454), false);
    }
    #[test]
    fn _10001st() {
        assert_eq!(nth_prime(10001), 104743);
    }
}
