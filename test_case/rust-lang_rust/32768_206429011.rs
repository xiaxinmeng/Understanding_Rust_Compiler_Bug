
---- slice::IterMut<'a, T>::into_slice_0 stdout ----
    <anon>:10:27: 10:35 error: no method named `as_slice` found for type `std::slice::IterMut<'_, _>` in the current scope
<anon>:10     println!("{:?}", iter.as_slice());
                                    ^~~~~~~~
<std macros>:2:25: 2:56 note: in this expansion of format_args!
<std macros>:3:1: 3:54 note: in this expansion of print! (defined in <std macros>)
<anon>:10:5: 10:39 note: in this expansion of println! (defined in <std macros>)
<anon>:14:27: 14:35 error: no method named `as_slice` found for type `std::slice::IterMut<'_, _>` in the current scope
<anon>:14     println!("{:?}", iter.as_slice());
                                    ^~~~~~~~
<std macros>:2:25: 2:56 note: in this expansion of format_args!
<std macros>:3:1: 3:54 note: in this expansion of print! (defined in <std macros>)
<anon>:14:5: 14:39 note: in this expansion of println! (defined in <std macros>)
error: aborting due to previous error(s)
thread 'slice::IterMut<'a, T>::into_slice_0' panicked at 'Box<Any>', src/librustc/session/mod.rs:161
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    slice::IterMut<'a, T>::into_slice_0

