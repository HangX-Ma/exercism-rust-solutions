pub fn square(s: u32) -> u64 {
    // unimplemented!("grains of rice on square {s}");
    match s {
        1..=64 => 2u64.pow(s - 1),
        _ => panic!("Square must be between 1 and 64"),
    }
}

pub fn total() -> u64 {
    (1..=64).into_iter().map(|num| square(num)).sum()
}
