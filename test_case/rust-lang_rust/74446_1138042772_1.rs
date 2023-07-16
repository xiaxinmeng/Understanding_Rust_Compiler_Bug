rust
#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Month { Jan, Feb, Mar, Apr, May, Jun, Jul, Aug, Sep, Oct, Nov, Dec }
use self::Month::*;

fn quarter(val: Month) -> u8 {
    match val {
        Jan..=Mar => 1,
        Apr..=Jun => 2,
        Jul..=Aug => 3,
        Sep..=Dec => 4,
    }
}
