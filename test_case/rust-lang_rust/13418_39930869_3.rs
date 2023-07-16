 rust
test.rs:3:13: 3:16 error: cannot move out of dereference of `&`-pointer
test.rs:3 fn arg_item(&_x: &~str) {}
                      ^~~
test.rs:7:11: 7:14 error: cannot move out of dereference of `&`-pointer
test.rs:7     with(|&_x| ())
                    ^~~
test.rs:12:9: 12:12 error: cannot move out of dereference of `&`-pointer
test.rs:12     let &_x = &~"hi";
                   ^~~
test.rs:20:14: 20:29 error: cannot move out of dereference of `&`-pointer
test.rs:20     let _x = *Rc::new(~"hi");
                        ^~~~~~~~~~~~~~~
test.rs:33:9: 33:17 error: cannot move out of type `S`, which defines the `Drop` trait
test.rs:33         S {f:_s} => {}
                   ^~~~~~~~
test.rs:39:9: 39:17 error: cannot move out of type `S`, which defines the `Drop` trait
test.rs:39     let S {f:_s} = S {f:~"foo"};
                   ^~~~~~~~
test.rs:43:19: 43:27 error: cannot move out of type `S`, which defines the `Drop` trait
test.rs:43 fn move_in_fn_arg(S {f:_s}: S) {
                             ^~~~~~~~
test.rs:57:11: 57:14 error: cannot move out of dereference of `&`-pointer
test.rs:57     match a.a {
                     ^~~
test.rs:68:33: 68:44 error: cannot move out of captured outer variable
test.rs:68         let _h: proc() -> int = proc() *bar; //~ ERROR cannot move out of captured outer variable
                                           ^~~~~~~~~~~
