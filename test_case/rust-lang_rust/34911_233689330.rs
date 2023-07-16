
failures:
---- vec::Vec<T>::set_len_0 stdout ----
    <anon>:10:5: 10:6 error: unresolved name `v` [E0425]
<anon>:10     v.set_len(3);
              ^
error: aborting due to previous error(s) 
thread 'vec::Vec<T>::set_len_0' panicked at 'Box<Any>', src/librustc/session/mod.rs:171
note: Run with `RUST_BACKTRACE=1` for a backtrace.
---- vec::Vec<T>::set_len_2 stdout ----
    <anon>:4:19: 4:27 error: unable to infer enough type information about `_`; type annotations or generic parameter binding required [E0282]
<anon>:4     let mut vec = Vec::new();
                           ^~~~~~~~
error: aborting due to previous error(s) 
thread 'vec::Vec<T>::set_len_2' panicked at 'Box<Any>', src/librustc/session/mod.rs:171
failures:
    vec::Vec<T>::set_len_0
    vec::Vec<T>::set_len_2
