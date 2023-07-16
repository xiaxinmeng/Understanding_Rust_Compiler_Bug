
error: constant expression depends on a generic parameter
   --> library/core/src/array/mod.rs:505:59
    |
505 |     pub fn concat<const M: usize>(self, other: [T; M]) -> [T; N + M] {
    |                                                           ^^^^^^^^^^
    |
    = note: this may fail depending on what value the parameter takes
