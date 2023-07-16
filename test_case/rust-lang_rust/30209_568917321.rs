
$ rustc +nightly foo.rs --crate-type lib  
$  rustc +nightly bar.rs --extern foo=./libfoo.rlib  
