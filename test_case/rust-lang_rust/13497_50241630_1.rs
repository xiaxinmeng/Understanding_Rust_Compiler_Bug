
$ rustc r13497.rs 
r13497.rs:1:33: 1:37 error: missing lifetime specifier [E0106]
r13497.rs:1 fn read_lines_borrowed() -> Vec<&str> {
