
---- range::RangeArgument::end_0 stdout ----
    <anon>:9:5: 9:38 error: unresolved import `collections::range::RangeArgument`. Did you mean `self::collections::range`? [E0432]
<anon>:9 use collections::range::RangeArgument;
             ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
<anon>:11:18: 11:21 error: no method named `end` found for type `std::ops::RangeFrom<_>` in the current scope 
<anon>:11 assert_eq!((3..).end(), None);
                           ^~~
<anon>:11:1: 11:31 note: in this expansion of assert_eq! (defined in <std macros>)
<anon>:11:18: 11:21 help: items from traits can only be used if the trait is in scope; the following trait is implemented but not in scope, perhaps add a `use` for it: 
<anon>:11:18: 11:21 help: candidate #1: `use main::collections::range::RangeArgument` 
<anon>:12:20: 12:23 error: no method named `end` found for type `std::ops::Range<_>` in the current scope 
<anon>:12 assert_eq!((3..10).end(), Some(&10));
                             ^~~
<anon>:12:1: 12:38 note: in this expansion of assert_eq! (defined in <std macros>)
<anon>:12:20: 12:23 note: did you mean to write `(3..10).end`? 
<anon>:12 assert_eq!((3..10).end(), Some(&10));
                             ^~~
<anon>:12:1: 12:38 note: in this expansion of assert_eq! (defined in <std macros>)
<anon>:12:20: 12:23 help: items from traits can only be used if the trait is in scope; the following trait is implemented but not in scope, perhaps add a `use` for it: 
<anon>:12:20: 12:23 help: candidate #1: `use main::collections::range::RangeArgument` 
error: aborting due to previous error(s) 
thread 'range::RangeArgument::end_0' panicked at 'Box<Any>', src/librustc/session/mod.rs:165
note: Run with `RUST_BACKTRACE=1` for a backtrace.

---- range::RangeArgument::start_0 stdout ----
    <anon>:9:5: 9:38 error: unresolved import `collections::range::RangeArgument`. Did you mean `self::collections::range`? [E0432]
<anon>:9 use collections::range::RangeArgument;
             ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
<anon>:11:19: 11:24 error: no method named `start` found for type `std::ops::RangeTo<_>` in the current scope 
<anon>:11 assert_eq!((..10).start(), None);
                            ^~~~~
<anon>:11:1: 11:34 note: in this expansion of assert_eq! (defined in <std macros>)
<anon>:11:19: 11:24 help: items from traits can only be used if the trait is in scope; the following trait is implemented but not in scope, perhaps add a `use` for it: 
<anon>:11:19: 11:24 help: candidate #1: `use main::collections::range::RangeArgument` 
<anon>:12:20: 12:25 error: no method named `start` found for type `std::ops::Range<_>` in the current scope 
<anon>:12 assert_eq!((3..10).start(), Some(&3));
                             ^~~~~
<anon>:12:1: 12:39 note: in this expansion of assert_eq! (defined in <std macros>)
<anon>:12:20: 12:25 note: did you mean to write `(3..10).start`? 
<anon>:12 assert_eq!((3..10).start(), Some(&3));
                             ^~~~~
<anon>:12:1: 12:39 note: in this expansion of assert_eq! (defined in <std macros>)
<anon>:12:20: 12:25 help: items from traits can only be used if the trait is in scope; the following trait is implemented but not in scope, perhaps add a `use` for it: 
<anon>:12:20: 12:25 help: candidate #1: `use main::collections::range::RangeArgument` 
error: aborting due to previous error(s) 
thread 'range::RangeArgument::start_0' panicked at 'Box<Any>', src/librustc/session/mod.rs:165
