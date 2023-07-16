
num-integer = { version = "0.1", default-features = false }
use num_integer::{div_floor, Integer};

let next_nr_x: u16 = nr_x.div_floor(&10);
