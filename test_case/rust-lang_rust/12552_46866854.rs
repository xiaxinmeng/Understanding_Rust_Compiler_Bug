
$ rustc bugs.rs 
bugs.rs:4:5: 4:12 error: mismatched types: expected `core::result::Result<<generic #9>,<generic integer #0>>` but found `core::option::Option<<generic #12>>` (expected enum core::result::Result but found enum core::option::Option)
bugs.rs:4     Some(k) => match k {
              ^~~~~~~
bugs.rs:7:5: 7:9 error: mismatched types: expected `core::result::Result<<generic #9>,<generic integer #0>>` but found `core::option::Option<<generic #13>>` (expected enum core::result::Result but found enum core::option::Option)
bugs.rs:7     None => ()
              ^~~~
bugs.rs:5:27: 5:28 error: failed to find an implementation of trait core::fmt::Show for <generic #0>
bugs.rs:5       a => println!("{}", a)
                                    ^
note: in expansion of format_args!
<std macros>:2:23: 2:77 note: expansion site
<std macros>:1:1: 3:2 note: in expansion of println!
bugs.rs:5:12: 6:6 note: expansion site

