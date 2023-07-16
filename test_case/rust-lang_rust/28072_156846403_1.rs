
$ cargo test
   Compiling macro-scoping-bug v0.1.0 (file:///home/n/sandbox/rust/macro-scoping-bug)
src/main.rs:14:6: 14:7 error: unresolved name `C`
src/main.rs:14 mac!(C);
                    ^
note: in expansion of format_args!
<std macros>:2:25: 2:56 note: expansion site
<std macros>:1:1: 2:62 note: in expansion of print!
<std macros>:3:1: 3:54 note: expansion site
<std macros>:1:1: 3:58 note: in expansion of println!
src/main.rs:8:17: 8:58 note: expansion site
src/main.rs:3:1: 12:2 note: in expansion of mac!
src/main.rs:14:1: 14:9 note: expansion site
error: aborting due to previous error
Could not compile `macro-scoping-bug`.

To learn more, run the command again with --verbose.
