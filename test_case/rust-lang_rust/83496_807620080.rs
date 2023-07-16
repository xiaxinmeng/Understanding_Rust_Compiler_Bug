plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: associated function has missing stability attribute
   --> library/core/src/char/methods.rs:703:5
    |
703 | /     pub const fn code_point(self) -> u32 {
704 | |         // Casting a `char` to a `u32` gives the underlying scalar value, and all scalar values are
705 | |         // valid code points
707 | |     }
    | |_____^

error: aborting due to previous error
