 rust
failures:

---- iter::traits::ExactSizeIterator_1 stdout ----
    <anon>:35:21: 35:28 error: cannot borrow immutable local variable `counter` as mutable
<anon>:35 assert_eq!(Some(1), counter.next());
                              ^~~~~~~
<anon>:35:21: 35:28 note: in this expansion of assert_eq! (defined in <std macros>)
<anon>:32:5: 32:12 help: to make the local variable mutable, use `mut` as shown:
<anon>:   let mut counter = Counter::new();
<anon>:37:21: 37:28 error: cannot borrow immutable local variable `counter` as mutable
<anon>:37 assert_eq!(Some(2), counter.next());
                              ^~~~~~~
<anon>:37:21: 37:28 note: in this expansion of assert_eq! (defined in <std macros>)
<anon>:32:5: 32:12 help: to make the local variable mutable, use `mut` as shown:
<anon>:   let mut counter = Counter::new();
<anon>:39:21: 39:28 error: cannot borrow immutable local variable `counter` as mutable
<anon>:39 assert_eq!(Some(3), counter.next());
                              ^~~~~~~
<anon>:39:21: 39:28 note: in this expansion of assert_eq! (defined in <std macros>)
<anon>:32:5: 32:12 help: to make the local variable mutable, use `mut` as shown:
<anon>:   let mut counter = Counter::new();
<anon>:41:21: 41:28 error: cannot borrow immutable local variable `counter` as mutable
<anon>:41 assert_eq!(Some(4), counter.next());
                              ^~~~~~~
<anon>:41:21: 41:28 note: in this expansion of assert_eq! (defined in <std macros>)
<anon>:32:5: 32:12 help: to make the local variable mutable, use `mut` as shown:
<anon>:   let mut counter = Counter::new();
<anon>:43:21: 43:28 error: cannot borrow immutable local variable `counter` as mutable
<anon>:43 assert_eq!(Some(5), counter.next());
                              ^~~~~~~
<anon>:43:21: 43:28 note: in this expansion of assert_eq! (defined in <std macros>)
<anon>:32:5: 32:12 help: to make the local variable mutable, use `mut` as shown:
<anon>:   let mut counter = Counter::new();
<anon>:45:21: 45:28 error: cannot borrow immutable local variable `counter` as mutable
<anon>:45 assert_eq!(Some(6), counter.next());
                              ^~~~~~~
<anon>:45:21: 45:28 note: in this expansion of assert_eq! (defined in <std macros>)
<anon>:32:5: 32:12 help: to make the local variable mutable, use `mut` as shown:
<anon>:   let mut counter = Counter::new();
<anon>:47:18: 47:25 error: cannot borrow immutable local variable `counter` as mutable
<anon>:47 assert_eq!(None, counter.next());
                           ^~~~~~~
<anon>:47:18: 47:25 note: in this expansion of assert_eq! (defined in <std macros>)
<anon>:32:5: 32:12 help: to make the local variable mutable, use `mut` as shown:
<anon>:   let mut counter = Counter::new();
error: aborting due to previous error(s)
thread 'iter::traits::ExactSizeIterator_1' panicked at 'Box<Any>', src/librustc/session/mod.rs:167
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    iter::traits::ExactSizeIterator_1


