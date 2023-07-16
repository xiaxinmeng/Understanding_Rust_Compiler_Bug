plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0425]: cannot find function `is_8digits` in this scope
   --> library/core/src/num/dec2flt/decimal.rs:234:17
    |
234 |             if !is_8digits(v) {
    |
help: consider importing this function
    |
12  | use crate::num::dec2flt::common::is_8digits;
12  | use crate::num::dec2flt::common::is_8digits;
    |

error[E0425]: cannot find value `exponent` in this scope
  --> library/core/src/num/dec2flt/slow.rs:40:24
   |
40 |     d.decimal_point += exponent as i32;

For more information about this error, try `rustc --explain E0425`.
error: could not compile `core` due to 2 previous errors
Build completed unsuccessfully in 0:00:11
