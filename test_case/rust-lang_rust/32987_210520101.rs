
---- binary_heap::BinaryHeap<T>::append_0 stdout ----
    <anon>:5:17: 5:33 error: failed to resolve. Use of undeclared type or module `BinaryHeap` [E0433]
<anon>:5     let mut a = BinaryHeap::from(v);
                         ^~~~~~~~~~~~~~~~
<anon>:8:17: 8:33 error: failed to resolve. Use of undeclared type or module `BinaryHeap` [E0433]
<anon>:8     let mut b = BinaryHeap::from(v);
                         ^~~~~~~~~~~~~~~~
<anon>:10:5: 10:21 error: the type of this value must be known in this context
<anon>:10     a.append(&mut b);
              ^~~~~~~~~~~~~~~~
<std macros>:5:8: 5:18 error: the type of this value must be known in this context
<std macros>:5 if ! ( * left_val == * right_val ) {
                      ^~~~~~~~~~
<anon>:12:5: 12:68 note: in this expansion of assert_eq! (defined in <std macros>)
<std macros>:5:22: 5:33 error: the type of this value must be known in this context
<std macros>:5 if ! ( * left_val == * right_val ) {
                                    ^~~~~~~~~~~
<anon>:12:5: 12:68 note: in this expansion of assert_eq! (defined in <std macros>)
<anon>:13:13: 13:25 error: the type of this value must be known in this context
<anon>:13     assert!(b.is_empty());
                      ^~~~~~~~~~~~
<anon>:13:5: 13:27 note: in this expansion of assert! (defined in <std macros>)
error: aborting due to previous error(s)
thread 'binary_heap::BinaryHeap<T>::append_0' panicked at 'Box<Any>', src/librustc/session/mod.rs:161
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    binary_heap::BinaryHeap<T>::append_0
