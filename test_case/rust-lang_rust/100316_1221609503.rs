plain
   Compiling xz2 v0.1.6
   Compiling toml v0.5.9
    Finished dev [unoptimized] target(s) in 58.49s
Checking stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Updating git repository `https://github.com/r-efi/r-efi`
---
     |
note: `std::path::Prefix` defined here
    --> /checkout/library/std/src/path.rs:143:1
     |
143  | pub enum Prefix<'a> {
     = note: the matched value is of type `std::path::Prefix`
     = note: the matched value is of type `std::path::Prefix`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
     |
1892 ~             Prefix::Disk(drive) => Utf8Prefix::Disk(drive),
1893 ~             _ => todo!(),

For more information about this error, try `rustc --explain E0004`.
error: could not compile `camino` due to previous error
warning: build failed, waiting for other jobs to finish...
