// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    // unimplemented!()
    let mut magazine_hash = HashMap::new();
    for word in magazine {
        let count = magazine_hash.entry(word).or_insert(0);
        *count += 1;
    }

    for matched_word in note {
        match magazine_hash.get_mut(matched_word) {
            None => return false,
            Some(count) => {
                if *count == 0 {return false;} else {*count -= 1;}
            }
        }
    }
    true
}
