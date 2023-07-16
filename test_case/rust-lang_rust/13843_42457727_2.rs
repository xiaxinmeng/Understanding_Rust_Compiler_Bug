
% rust-backtrace ./regex-empty-str
Breakpoint 1 at 0x471a30
warning: Could not load shared library symbols for linux-vdso.so.1.
Do you need "set solib-search-path" or "set sysroot"?
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/usr/lib/libthread_db.so.1".
task '<main>' failed at 'index out of bounds: the len is 0 but the index is 0', /build/rust-git/src/rust/src/libregex/lib.rs:1

Breakpoint 1, 0x0000000000471a30 in rust_fail ()
#0  0x0000000000471a30 in rust_fail ()
#1  0x0000000000471a03 in rt::unwind::Unwinder::begin_unwind::h7a34b2c57cd8ebe9EV9::v0.11.pre ()
#2  0x0000000000453aad in rt::unwind::begin_unwind_inner::h21d0e74b81447276R69::v0.11.pre ()
#3  0x0000000000452d28 in rt::unwind::begin_unwind::h10355639713018170346::v0.11.pre ()
#4  0x0000000000471c17 in rt::unwind::begin_unwind_raw::hcae126cecf9ac08b039::v0.11.pre ()
#5  0x0000000000452b2b in rt::unwind::fail_::hb350ad41fcb04968E19::v0.11.pre ()
#6  0x0000000000471c5f in rt::unwind::fail_bounds_check::closure.40422 ()
#7  0x0000000000453fbe in rt::unwind::fail_bounds_check::h77cb92ae44655be1119::v0.11.pre ()
#8  0x0000000000410c08 in parse::Parser$LT$$x27a$GT$::parse::hd6035c4711160a86Kuf::v0.11.pre ()
#9  0x00000000004190b9 in re::Regex::new::h7515f9e2f6385e4exZg::v0.11.pre ()
#10 0x0000000000402afe in main::h695ce60898fd8803gaa::v0.0 ()
#11 0x0000000000452a43 in start::closure.7194 ()
#12 0x0000000000468903 in rt::task::Task::run::closure.40319 ()
#13 0x0000000000477dcc in rust_try ()
#14 0x0000000000468762 in rt::task::Task::run::h93895fb04573c54dUW7::v0.11.pre ()
#15 0x0000000000452834 in start::h1727cab19364d8a2xxd::v0.11.pre ()
#16 0x0000000000452624 in lang_start::hea05ce15f18b4932Rwd::v0.11.pre ()
#17 0x0000000000402b6f in main ()
