// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    // unimplemented!("implement `fn divmod`");
    (dividend / divisor, dividend % divisor)
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    // unimplemented!("implement `fn evens`");
    // TODO: remove this; it's only necessary to allow this function to compile
    // before the student has done any work.
    iter.enumerate().filter(|tuple| tuple.0 % 2 == 0).map(|tuple| tuple.1)
}

pub struct Position(pub i16, pub i16);
impl Position {
    pub fn manhattan(&self) -> i16 {
        // unimplemented!("implement `fn manhattan`")
        self.0.abs() + self.1.abs()
    }
}
