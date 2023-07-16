console
$ cargo run -q --target x86_64-pc-windows-gnu
[src/main.rs:4] s = "\u{9b}:]  = \n\0\0\0\0\0\0\01\u{10}L[...]thread 'main' panicked at 'failed printing to stderr: Windows stdio in console mode does not support writing non-UTF-8 byte sequences', library\std\src\io\stdio.rs:1008:9
