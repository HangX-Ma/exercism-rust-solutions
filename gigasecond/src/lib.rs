use time::{PrimitiveDateTime as DateTime, ext::NumericalDuration};

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    // unimplemented!("What time is a gigasecond later than {start}");
    start + (1e9).seconds()
}