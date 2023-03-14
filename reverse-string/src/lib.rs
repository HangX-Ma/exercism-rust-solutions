use unicode_reverse::reverse_grapheme_clusters_in_place;

pub fn reverse(input: &str) -> String {
    // unimplemented!("Write a function to reverse {input}");
    
    /* method normal */
    // input.chars().rev().collect()
    
    /* method using unicode_reverse(https://crates.io/crates/unicode-reverse) */
    let mut x = input.to_string();
    reverse_grapheme_clusters_in_place(&mut x);
    x.to_string()
}
