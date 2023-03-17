use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn invert_relationship(c: Comparison) -> Comparison {
    match c { 
        Comparison::Sublist => Comparison::Superlist,
        Comparison::Superlist => Comparison::Sublist, 
        x => x
    }
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    // unimplemented!("Determine if the first list is equal to, sublist of, superlist of or unequal to the second list.");
    let f_len = _first_list.len(); 
    let s_len = _second_list.len();
    match f_len.cmp(&s_len) {
        Ordering::Equal => if _first_list == _second_list {Comparison::Equal} else {Comparison::Unequal},
        Ordering::Greater => invert_relationship(sublist(_second_list, _first_list)),
        _ => { 
            for shift in f_len..=s_len {
                if &_second_list[shift - f_len..shift] == _first_list {
                    return Comparison::Sublist;
                }
            }
            Comparison::Unequal
        } 
    }
}
