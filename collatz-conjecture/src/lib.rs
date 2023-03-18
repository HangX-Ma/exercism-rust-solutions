pub fn collatz(n: u64) -> Option<u64> {
    // unimplemented!(
    //     "return Some(x) where x is the number of steps required to reach 1 starting with {n}"
    // )
    let mut count: u64 = 0;
    let mut collatz_num = n.clone();
    while collatz_num != 1 {
        match collatz_num {
            0 => return None,
            n if n % 2 == 0 => collatz_num /= 2,
            _ => {
                collatz_num = u64::checked_mul(collatz_num, 3)?;
                collatz_num = u64::checked_add(collatz_num, 1)?;
            }
        }
        count += 1;
    }
    Some(count)
}
