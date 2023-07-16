plain
    Checking std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
    Checking hashbrown v0.11.0
    Checking miniz_oxide v0.4.0
    Checking addr2line v0.14.0
error[E0599]: no method named `into_raw_parts` found for struct `BufWriter` in the current scope
    |
    |
70  | pub struct BufWriter<W: Write> {
    | ------------------------------ method `into_raw_parts` not found for this
...
310 |             Ok(()) => Ok(self.into_raw_parts().0),
    |                               ^^^^^^^^^^^^^^ help: there is an associated function with a similar name: `into_parts`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
error: could not compile `std`
