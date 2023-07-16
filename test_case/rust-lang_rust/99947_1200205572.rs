plain
   Compiling miniz_oxide v0.4.0
   Compiling hashbrown v0.12.3
   Compiling std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
   Compiling addr2line v0.16.0
error[E0201]: duplicate definitions with name `is_documentation`:
     |
     |
1512 | /     pub const fn is_documentation(&self) -> bool {
1513 | |         (self.segments()[0] == 0x2001) && (self.segments()[1] == 0xdb8)
1514 | |     }
     | |_____- previous definition of `is_documentation` here
...
1630 | /     pub const fn is_documentation(&self) -> bool {
1631 | |         (self.segments()[0] == 0x2001) && (self.segments()[1] == 0xdb8)
     | |_____^ duplicate definition

For more information about this error, try `rustc --explain E0201`.
error: could not compile `std` due to previous error
