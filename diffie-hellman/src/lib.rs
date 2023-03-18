use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    // unimplemented!("Pick a private key greater than 1 and less than {p}")
    rand::thread_rng().gen_range(2..p) as u64
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    // unimplemented!("Calculate public key using prime numbers {p} and {g}, and private key {a}")
    modular_pow(g as u128, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    // unimplemented!(
    //     "Calculate secret key using prime number {p}, public key {b_pub}, and private key {a}"
    // )
    modular_pow(b_pub as u128, a, p)
}

fn modular_pow(base: u128, exponent: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }

    let mut mut_result = 1;
    let mut mut_base = base.clone();
    let mut mut_exponent = exponent.clone();

    while mut_exponent > 0 {
        if mut_exponent % 2 == 1 {
            mut_result = (mut_result * mut_base) % modulus as u128;
        }
        mut_exponent = mut_exponent >> 1;
        mut_base = (mut_base * mut_base) % modulus as u128;
    }
    mut_result as u64
}