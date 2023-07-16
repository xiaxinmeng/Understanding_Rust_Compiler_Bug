
macro_rules! foo(() => ([3]))
foo().to_owned()

Untitled.rs:3:10: 3:11 error: unexpected token: `.`
Untitled.rs:3     foo!().to_owned()
                        ^
