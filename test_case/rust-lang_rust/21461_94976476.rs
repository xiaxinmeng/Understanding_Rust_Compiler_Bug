 shell
$ rustc main.rs
main.rs:7:35: 7:38 error: associated type bindings are not allowed here [E0229]
main.rs:7 impl<'a> FnMut(&mut i32, &i32) -> i32 for VecEnv<'a> {
                                            ^~~
error: aborting due to previous error
