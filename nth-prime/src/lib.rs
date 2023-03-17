
fn is_prime(num: u32) -> bool {
    !(2..((num as f64).sqrt() as u32 + 1)).any(|factor| num % factor == 0)
}

pub fn nth(n: u32) -> u32 {
    (2..).filter(|&num| is_prime(num)).nth(n as usize).unwrap()
}