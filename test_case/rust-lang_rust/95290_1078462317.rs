plain
    Checking miniz_oxide v0.4.0
    Checking object v0.26.2
    Checking hashbrown v0.12.0
    Checking addr2line v0.16.0
error[E0545]: `issue` must be a non-zero numeric string or "none"
    |
    |
192 |     #[unstable(feature = "osstr_bytes", issue = "0")]
    |                                                 |
    |                                                 |
    |                                                 `issue` must not be "0", use "none" instead

error[E0545]: `issue` must be a non-zero numeric string or "none"
    |
    |
712 |     #[unstable(feature = "osstr_bytes", issue = "0")]
    |                                                 |
    |                                                 |
    |                                                 `issue` must not be "0", use "none" instead

error[E0624]: associated function `as_u8_slice` is private
    |
715 |         self.inner.as_u8_slice()
    |                    ^^^^^^^^^^^ private associated function
    |
    |
   ::: library/std/src/sys/unix/os_str.rs:199:5
    |
199 |     fn as_u8_slice(&self) -> &[u8] {

Some errors have detailed explanations: E0545, E0624.
For more information about an error, try `rustc --explain E0545`.
error: could not compile `std` due to 3 previous errors
