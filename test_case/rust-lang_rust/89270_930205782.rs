plain
    Checking rand v0.7.3
    Checking std v0.0.0 (/checkout/library/std)
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking core v0.0.0 (/checkout/library/core)
error: unknown character escape: `D`
    --> library/std/src/path/tests.rs:1270:29
     |
1270 |         tp!(r"\\?\C:", "\\?\D:/foo//.//", r"\\?\D:\foo");
     |                             ^ unknown character escape
     |
     = help: for more information, visit <https://static.rust-lang.org/doc/master/reference.html#literals>

error: unknown character escape: `f`
    --> library/std/src/path/tests.rs:1272:30
     |
1272 |         tp!(r"\\?\A:\x\y", "\foo\.", r"\\?\A:\foo");
     |                              ^ unknown character escape
     |
     = help: for more information, visit <https://static.rust-lang.org/doc/master/reference.html#literals>

error: unknown character escape: `.`
    --> library/std/src/path/tests.rs:1272:34
     |
1272 |         tp!(r"\\?\A:\x\y", "\foo\.", r"\\?\A:\foo");
     |                                  ^ unknown character escape
     |
     = help: for more information, visit <https://static.rust-lang.org/doc/master/reference.html#literals>

error: unknown character escape: `f`
    --> library/std/src/path/tests.rs:1273:30
     |
1273 |         tp!(r"\\?\A:\x\y", "\foo\.", r"\\?\A:\foo");
     |                              ^ unknown character escape
     |
     = help: for more information, visit <https://static.rust-lang.org/doc/master/reference.html#literals>

error: unknown character escape: `.`
    --> library/std/src/path/tests.rs:1273:34
     |
1273 |         tp!(r"\\?\A:\x\y", "\foo\.", r"\\?\A:\foo");
     |                                  ^ unknown character escape
     |
     = help: for more information, visit <https://static.rust-lang.org/doc/master/reference.html#literals>

error: cannot find macro `ty` in this scope
    --> library/std/src/path/tests.rs:1274:9
8    | / macro_rules! t (
8    | / macro_rules! t (
9    | |     ($path:expr, iter: $iter:expr) => (
10   | |         {
11   | |             let path = Path::new($path);
106  | |     );
107  | | );
107  | | );
     | |__- similarly named macro `t` defined here
...
1274 |           ty!(r"\\?\D:\foo\bar", r"\\?\x:\y\z", r"\\?\x:\y\z");
     |           ^^ help: a macro with a similar name exists: `t`
error: could not compile `std` due to 6 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:01:32
