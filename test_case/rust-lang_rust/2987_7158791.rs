
a.rs:1:16: 1:19 error: bare `str` is not a type
a.rs:1 fn main(args: ~[str]) {
                       ^~~
a.rs:2:16: 2:45 error: binary operation + cannot be applied to type `str/18`
a.rs:2     io::println("hello world from '" + args[0] + "'!");
                       ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~
a.rs:2:16: 2:53 error: binary operation + cannot be applied to type `str/18`
a.rs:2     io::println("hello world from '" + args[0] + "'!");
                       ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
