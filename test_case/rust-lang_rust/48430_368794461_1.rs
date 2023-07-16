
error[E0407]: method `is_ascii_hexdigit` is not a member of trait `AsciiExt`
   --> libstd/ascii.rs:261:5
    |
261 | /     fn is_ascii_hexdigit(&self) -> bool {
262 | |         self.iter().all(|b| b.is_ascii_hexdigit())
263 | |     }
    | |_____^ not a member of trait `AsciiExt`
