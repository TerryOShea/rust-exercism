fn is_prime(n: u32) -> bool {
    if n == 2 {
        return true;
    } else if n % 2 == 0 {
        return false;
    } else {
        let mut square_root = f64::from(n).sqrt() as u32;
        square_root += 1;
        for i in (3..square_root).step_by(2) {
            if n % i == 0 {
                return false;
            }
        }
        return true;
    }
}

pub fn nth(n: u32) -> u32 {
    let mut count = 0;
    let mut prime = 2;

    while count < n {
        prime += 1;
        while !is_prime(prime) {
            prime += 1;
        }
        count += 1;
    }

    prime
}
