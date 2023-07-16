
/home/manishearth/Mozilla/rust/src/libstd/panicking.rs:36:61: 36:65 warning: the `uint` type is deprecated; use `usize` or a fixed-sized integer
/home/manishearth/Mozilla/rust/src/libstd/panicking.rs:36 pub fn on_panic(obj: &(Any+Send), file: &'static str, line: uint) {
                                                                                                                      ^~~~
/home/manishearth/Mozilla/rust/src/libstd/panicking.rs:36:65: 36:65 help: add #![feature(int_uint)] to the crate attributes to silence this warning
/home/manishearth/Mozilla/rust/src/libgetopts/lib.rs:966:50: 966:66 error: failed to resolve. Could not find `uint` in `std`
/home/manishearth/Mozilla/rust/src/libgetopts/lib.rs:966     t("\nMary had a little lamb\nLittle lamb\n", ::std::uint::MAX,
                                                                                                          ^~~~~~~~~~~~~~~~
/home/manishearth/Mozilla/rust/src/libgetopts/lib.rs:966:50: 966:66 error: unresolved name `std::uint::MAX`
/home/manishearth/Mozilla/rust/src/libgetopts/lib.rs:966     t("\nMary had a little lamb\nLittle lamb\n", ::std::uint::MAX,
                                                                                                          ^~~~~~~~~~~~~~~~
error: aborting due to 2 previous errors
make: *** [x86_64-unknown-linux-gnu/stage1/test/getoptstest-x86_64-unknown-linux-gnu] Error 101
make: *** Waiting for unfinished jobs....
/home/manishearth/Mozilla/rust/src/libstd/old_io/mod.rs:1618:10: 1618:11 warning: the type parameter `T` is not constrained by the impl trait, self type, or predicates
/home/manishearth/Mozilla/rust/src/libstd/old_io/mod.rs:1618 impl<'a, T, A: ?Sized + Acceptor<T>> Iterator for IncomingConnections<'a, A> {
                                                                      ^
/home/manishearth/Mozilla/rust/src/libstd/thread.rs:818:21: 818:31 error: mismatched types:
 expected `u32`,
    found `usize`
(expected u32,
    found usize) [E0308]
/home/manishearth/Mozilla/rust/src/libstd/thread.rs:818             tx.send(x_in_child).unwrap();
                                                                            ^~~~~~~~~~
<core macros>:5:24: 5:35 error: mismatched types:
 expected `usize`,
    found `u32`
(expected usize,
    found u32) [E0308]
<core macros>:5 if ! ( ( * left_val == * right_val ) && ( * right_val == * left_val ) ) {
                                       ^~~~~~~~~~~
<core macros>:1:1: 9:39 note: in expansion of assert_eq!
/home/manishearth/Mozilla/rust/src/libstd/thread.rs:822:9: 822:45 note: expansion site
<core macros>:5:58: 5:68 error: mismatched types:
 expected `u32`,
    found `usize`
(expected u32,
    found usize) [E0308]
<core macros>:5 if ! ( ( * left_val == * right_val ) && ( * right_val == * left_val ) ) {
                                                                         ^~~~~~~~~~
<core macros>:1:1: 9:39 note: in expansion of assert_eq!
/home/manishearth/Mozilla/rust/src/libstd/thread.rs:822:9: 822:45 note: expansion site
/home/manishearth/Mozilla/rust/src/libstd/thread.rs:859:24: 859:35 error: mismatched types:
 expected `usize`,
    found `u32`
(expected usize,
    found u32) [E0308]
/home/manishearth/Mozilla/rust/src/libstd/thread.rs:859                 if x < GENERATIONS {
                                                                               ^~~~~~~~~~~
error: aborting due to 4 previous errors
make: *** [x86_64-unknown-linux-gnu/stage1/test/stdtest-x86_64-unknown-linux-gnu] Error 101
