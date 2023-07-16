plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: associated function has missing stability attribute
  --> library/core/src/slice/ascii.rs:32:5
   |
32 | /     pub fn is_ascii_digit(&self) -> bool {
33 | |         self.iter().all(|i| {
34 | |             let i = *i;
35 | |             (i >= b'0') && (i <= b'9')
37 | |     }
   | |_____^

error: could not compile `core` due to previous error
