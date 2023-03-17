use std::{collections::HashSet, cmp::Ordering};
use itertools::Itertools;


pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    // unimplemented!(
    //     "For the '{word}' word find anagrams among the following words: {possible_anagrams:?}"
    // );
    let mut output_set: HashSet<&'a str> = HashSet::new();
    let w = word.to_lowercase().chars().sorted().collect::<String>();

    for anagram in possible_anagrams {
        if word.to_lowercase().cmp(&anagram.to_lowercase().to_string()) == Ordering::Equal {
            continue;
        }
        let cmpw = anagram.to_lowercase().chars().sorted().collect::<String>();
        if w.cmp(&cmpw) == Ordering::Equal {
            output_set.insert(*anagram);
        }
    }
    output_set
}
