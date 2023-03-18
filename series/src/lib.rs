pub fn series(digits: &str, len: usize) -> Vec<String> {
    // unimplemented!("What are the series of length {len} in string {digits:?}")
    let digits_len = digits.len();
    match digits_len >= len {
            false => vec![],
            true => (len..=digits_len).into_iter()
                                     .map(|val| digits[(val - len)..val].to_string())
                                     .collect()
    }
}
