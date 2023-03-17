pub fn verse(n: u32) -> String {
    let mut lyric = String::new();
    match n {
        0 => lyric += &format!("No more bottles of beer on the wall, no more bottles of beer.\n").to_string(),
        1 => lyric += &format!("1 bottle of beer on the wall, 1 bottle of beer.\n").to_string(),
        val @ _ => lyric += &format!("{} bottles of beer on the wall, {} bottles of beer.\n", val, val).to_string(),
    };
    match n {
        0 => lyric += &format!("Go to the store and buy some more, 99 bottles of beer on the wall.\n"),
        1 => lyric += &format!("Take it down and pass it around, no more bottles of beer on the wall.\n").to_string(),
        2 => lyric += &format!("Take one down and pass it around, 1 bottle of beer on the wall.\n").to_string(),
        val @ _ => lyric += &format!("Take one down and pass it around, {} bottles of beer on the wall.\n", val - 1).to_string(),
    };
    lyric
}

pub fn sing(start: u32, end: u32) -> String {
    // unimplemented!("sing verses {start} to {end}, inclusive")
    let mut s = String::new();
    for n in (end..=start).rev() {
       s = s + &verse(n) + &"\n";
    }
    // delete the final '\n'
    let new_s = &s[0..s.len() - 1];
    new_s.to_string()
}
