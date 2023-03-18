pub fn raindrops(n: u32) -> String {
    // unimplemented!("what sound does Raindrop #{n} make?")
    let s: String = (1..=3).into_iter().map(|x| match x * 2 + 1 {
        3 if n % 3 == 0 => String::from("Pling"),
        5 if n % 5 == 0 => String::from("Plang"),
        7 if n % 7 == 0 => String::from("Plong"),
        _ => String::new(),
    })
    .collect();
    if s.is_empty() { n.to_string() } else { s }
}
