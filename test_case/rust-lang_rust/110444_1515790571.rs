plain
   Compiling compiletest v0.0.0 (/checkout/src/tools/compiletest)
error[E0308]: mismatched types
    --> src/tools/compiletest/src/runtest.rs:3842:22
     |
3842 |                 ("", actual)
     |                      ^^^^^^ expected `&str`, found `HashSet<&str>`
     = note: expected reference `&str`
     = note: expected reference `&str`
                   found struct `HashSet<&str>`
For more information about this error, try `rustc --explain E0308`.
error: could not compile `compiletest` due to previous error
Build completed unsuccessfully in 0:04:52
