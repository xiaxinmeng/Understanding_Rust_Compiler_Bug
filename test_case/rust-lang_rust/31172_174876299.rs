
failures:

---- iter::RangeFrom<A>::step_by_0 stdout ----
    <anon>:4:22: 4:29 error: use of unstable library feature 'step_by': recent addition (see issue #27741)
<anon>:4     for i in (0u8..).step_by(2).take(10) {
                              ^~~~~~~
<anon>:4:22: 4:29 help: add #![feature(step_by)] to the crate attributes to enable
error: aborting due to previous error
thread 'iter::RangeFrom<A>::step_by_0' panicked at 'Box<Any>', ../src/libsyntax/errors/mod.rs:527
