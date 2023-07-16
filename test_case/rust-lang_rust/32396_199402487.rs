
---- ops::RangeInclusive_0 stdout ----
        <anon>:5:16: 5:19 error: unresolved name `arr` [E0425]
<anon>:5     assert_eq!(arr[ ...2], [0,1,2  ]);
                        ^~~
<anon>:5:5: 5:39 note: in this expansion of assert_eq! (defined in <std macros>)
<std macros>:5:8: 5:18 error: the type of this value must be known in this context
<std macros>:5 if ! ( * left_val == * right_val ) {
                      ^~~~~~~~~~
<anon>:5:5: 5:39 note: in this expansion of assert_eq! (defined in <std macros>)
<std macros>:5:22: 5:33 error: the type of this value must be known in this context
<std macros>:5 if ! ( * left_val == * right_val ) {
                                    ^~~~~~~~~~~
<anon>:5:5: 5:39 note: in this expansion of assert_eq! (defined in <std macros>)
error: aborting due to previous error(s)
thread 'ops::RangeInclusive_0' panicked at 'Box<Any>', src/librustc/session/mod.rs:153
note: Run with `RUST_BACKTRACE=1` for a backtrace.

---- ops::RangeToInclusive_0 stdout ----
        <anon>:4:5: 4:13 error: unresolved name `Examples` [E0425]
<anon>:4     Examples
             ^~~~~~~~
error: aborting due to previous error(s)
thread 'ops::RangeToInclusive_0' panicked at 'Box<Any>', src/librustc/session/mod.rs:153

---- ops::RangeToInclusive_1 stdout ----
        <anon>:5:21: 5:25 error: inclusive range syntax is experimental (see issue #28237)
<anon>:5     assert_eq!(arr[ ...2], [0,1,2  ]);  // RangeToInclusive
                             ^~~~
<anon>:5:5: 5:39 note: in this expansion of assert_eq! (defined in <std macros>)
<anon>:5:21: 5:25 help: add #![feature(inclusive_range_syntax)] to the crate attributes to enable
<anon>:6:20: 6:25 error: inclusive range syntax is experimental (see issue #28237)
<anon>:6     assert_eq!(arr[1...2], [  1,2  ]);
                            ^~~~~
<anon>:6:5: 6:39 note: in this expansion of assert_eq! (defined in <std macros>)
<anon>:6:20: 6:25 help: add #![feature(inclusive_range_syntax)] to the crate attributes to enable
error: aborting due to previous error(s)
thread 'ops::RangeToInclusive_1' panicked at 'Box<Any>', src/librustc/session/mod.rs:153


failures:
    ops::RangeInclusive_0
    ops::RangeToInclusive_0
    ops::RangeToInclusive_1
