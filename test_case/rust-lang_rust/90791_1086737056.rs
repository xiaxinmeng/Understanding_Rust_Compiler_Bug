plain
9  | / /// Calls implementation provided memcmp.
10 | | ///
11 | | /// Interprets the data as u8.
12 | | ///
13 | | /// Returns 0 for equal, < 0 for less than and > 0 for greater
14 | | /// than.
15 | / extern "C" {
15 | / extern "C" {
16 | |     #[cfg(any(
17 | |         target_arch = "avr", 
18 | |         target_arch = "msp430",
...  |
25 | |     fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32;
26 | | }
   | |_- rustdoc does not generate documentation for extern block
   |
   = note: `-D unused-doc-comments` implied by `-D warnings`
   = help: use `//` for a plain comment
error: could not compile `core` due to previous error
Build completed unsuccessfully in 0:04:15
