
rprichard@ryan:~$ fixld ~/work/rust-staging2/build/install/bin/rustc test.rs 
<std macros>:2:20: 2:66 error: type `&mut std::fs::File` does not implement any method in scope named `write_fmt`
<std macros>:2 ( & mut * $ dst ) . write_fmt ( format_args ! ( $ ( $ arg ) * ) ) )
                                  ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
<std macros>:1:1: 2:68 note: in expansion of write!
<std macros>:4:1: 4:62 note: expansion site
<std macros>:1:1: 4:66 note: in expansion of writeln!
test.rs:9:13: 9:45 note: expansion site
<std macros>:2:66: 2:66 help: methods from traits can only be called if the trait is in scope; the following trait is implemented but not in scope, perhaps add a `use` for it:
<std macros>:1:1: 2:68 note: in expansion of write!
<std macros>:4:1: 4:62 note: expansion site
<std macros>:1:1: 4:66 note: in expansion of writeln!
test.rs:9:13: 9:45 note: expansion site
<std macros>:2:66: 2:66 help: candidate #1: use `std::io::Write`
<std macros>:1:1: 2:68 note: in expansion of write!
<std macros>:4:1: 4:62 note: expansion site
<std macros>:1:1: 4:66 note: in expansion of writeln!
test.rs:9:13: 9:45 note: expansion site
error: aborting due to previous error
