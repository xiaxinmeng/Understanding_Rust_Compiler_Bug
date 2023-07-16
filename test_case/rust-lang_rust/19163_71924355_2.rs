
$ rustc --version
rustc 1.0.0-dev (4e4e8cff1 2015-01-24 22:14:14 +0000)

$ rustc -L . mywrite.rs 

$ time rustc -L . main.rs 
<mywrite macros>:3:17: 5:16 error: cannot borrow immutable borrowed content as mutable
<mywrite macros>:3 let dst = & mut * $ dst ; ( | & mut : args | { dst . write_fmt ( args ) } ) (
<mywrite macros>:4 format_args ! ( $ ( $ arg ) * ) ) } )
<mywrite macros>:1:1: 4:38 note: in expansion of mywrite!
main.rs:5:5: 5:33 note: expansion site
error: aborting due to previous error

real    0m0.066s
user    0m0.056s
sys     0m0.008s
