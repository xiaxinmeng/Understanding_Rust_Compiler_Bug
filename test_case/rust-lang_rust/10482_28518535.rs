
use std::num::strconv;

fn main() {
   strconv::float_to_str_common(
        1.1_f64, 10u, true, strconv::SignNeg, strconv::DigAll);
}
