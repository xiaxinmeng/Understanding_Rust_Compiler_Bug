
$ cat /tmp/foo.rs
#![feature(plugin, custom_attribute)]
#![plugin(lint_for_crate)]

fn main() { }

$ rustc -C prefer-dynamic --crate-type dylib lint_for_crate.rs

$ rustc -D crate-not-okay -L . /tmp/foo.rs  
/tmp/foo.rs:1:1: 4:13 error: crate is not marked with #![crate_okay] [-D crate-not-okay]  
/tmp/foo.rs:1 #![feature(plugin, custom_attribute)]  
/tmp/foo.rs:2 #![plugin(lint_for_crate)]  
/tmp/foo.rs:3  
/tmp/foo.rs:4 fn main() { }  
error: aborting due to previous error  

$ sed -i '3i#![crate_okay]' /tmp/foo.rs
$ rustc -D crate-not-okay -L . /tmp/foo.rs  
$ echo $?
0
