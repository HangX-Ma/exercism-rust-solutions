pub fn square_of_sum(n: u32) -> u32 {
    // unimplemented!("square of sum of 1...{n}")
    (1..=n).into_iter().sum::<u32>().pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    // unimplemented!("sum of squares of 1...{n}")
    // (1..=n).into_iter().map(|x| x.pow(2)).sum::<u32>()
    (1..=n).into_iter().fold(0, |acc, elem| acc + elem.pow(2))

}

pub fn difference(n: u32) -> u32 {
    // unimplemented!("difference between square of sum of 1...{n} and sum of squares of 1...{n}")
    square_of_sum(n) - sum_of_squares(n)
}
