pub fn build_proverb(list: &[&str]) -> String {
    // unimplemented!("build a proverb from this list of items: {list:?}")
    match list.len() {
        0 => String::new(),
        _ => list.windows(2)
                    .map(|sub_list| format!("For want of a {} the {} was lost.\n", sub_list[0], sub_list[1]))
                    .chain(
                        list.into_iter()
                            .take(1)
                            .map(|&elem| format!("And all for the want of a {}.", elem))
                    )
                    .collect()
    }
}
