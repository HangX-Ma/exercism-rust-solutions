pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    // unimplemented!("Sum the multiples of all of {factors:?} which are less than {limit}")
    (1..limit).filter(|elem| {
        factors.into_iter()
               .filter(|&&fac| fac > 0)
               .any(|fac| elem % fac == 0)
    })
    .sum()
}
