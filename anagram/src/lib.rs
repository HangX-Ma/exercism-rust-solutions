use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let to_lowercase_closure = |ch: char| {
        if ch.is_uppercase() {ch.to_lowercase().to_string()} else {ch.to_string()}
    };

    let lw: Vec<_> = word.chars().map(to_lowercase_closure).collect();
    let mut sorted_w: Vec<_> = word.chars().map(to_lowercase_closure).collect();
    sorted_w.sort_unstable();
    
    let res = possible_anagrams.iter().filter(
        |w| {
            let mut anagram: Vec<_> = w.chars().map(to_lowercase_closure).collect();
            let is_same = lw == anagram;
            anagram.sort_unstable();
            anagram == sorted_w && !is_same
        }
    ).map(|&x| x);

    HashSet::from_iter(res)
}
