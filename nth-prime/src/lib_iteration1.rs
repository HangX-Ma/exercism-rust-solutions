const PRIME_EDGE : u32 = 200_000;

fn get_all_prime(v: &mut Vec<u32>) {
    v.push(2);
    for num in 2..=PRIME_EDGE {
        if is_prime(num) { v.push(num); }
    }
}

fn is_prime(num: u32) -> bool {
    let upper_limit = f64::sqrt(num as f64) as u32 + 1;

    for factor in 2..=upper_limit {
        if num % factor == 0 {
            return false;
        }
    }
    true
}

pub fn nth(n: u32) -> u32 {
    // unimplemented!("What is the 0-indexed {n}th prime number?")
    let ref mut prime_vec: Vec<u32> = vec![];
    get_all_prime(prime_vec);
    prime_vec[n as usize]
}
