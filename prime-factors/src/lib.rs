fn is_prime(num: u64) -> bool {
    !(2..((num as f64).sqrt() as u64 + 1)).any(|factor| num % factor == 0)
}

pub fn factors(n: u64) -> Vec<u64> {
    // unimplemented!("This should calculate the prime factors of {n}")
    let mut v: Vec<u64> = vec![];
    let mut val = n.clone();
    while val != 1 {
        let factor_iter = (2..=((n as f64).sqrt() as u64 * 3))
                                    .filter(|&x| is_prime(x))
                                    .find(|&divisor| val % divisor == 0)
                                    .unwrap();
        val /= factor_iter;
        v.push(factor_iter);
    }
    v
}
