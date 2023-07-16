
Program received signal SIGABRT, Aborted.
[Switching to process 55754 thread 0x1203]
0x00007fff85b97212 in __pthread_kill ()
(gdb) bt
#0  0x00007fff85b97212 in __pthread_kill ()
#1  0x00007fff8fdfab24 in pthread_kill ()
#2  0x00007fff8fe3ef61 in abort ()
#3  0x0000000100007835 in rt::util::abort::h78af1956eb5e49das::v0.9$x2dpre ()
#4  0x00000001006472b3 in rt::task::begin_unwind::hd550344a74825a7av::v0.9$x2dpre ()
#5  0x0000000100683915 in uvio::HomingIO::fire_homing_missile::h275a15cd89566c45a7::v0.9$x2dpre ()
#6  0x0000000100671181 in file::Drop$FileWatcher::drop::h243d5422d728745Mgap::v0.9$x2dpre ()
#7  0x00000001006710cd in file..FileWatcher::glue_drop::hb83953d6ca359e89ao ()
#8  0x000000010067106d in _$UP$file..FileWatcher::glue_free::hb4f5f58975dc6e74ak ()
#9  0x0000000100039ffb in logging::log::hd2877e3741c67e5aw::v0.9$x2dpre ()
#10 0x0000000100001d03 in main::anon::expr_fn::ap ()
#11 0x00000001000018c7 in main::hada226789343cd84ah::v0.0 ()
#12 0x00000001000d7755 in rt::task::__extensions__::build_start_wrapper::anon::anon::expr_fn::aw ()
#13 0x00000001000d6333 in rt::task::__extensions__::run::anon::expr_fn::a1 ()
#14 0x00000001007402e3 in rust_try (f=<value temporarily unavailable, due to optimizations>, fptr=<value temporarily unavailable, due to optimizations>, env=<value temporarily unavailable, due to optimizations>) at /Users/davidrenshaw/Code/rust/src/rt/rust_cxx_glue.cpp:20
#15 0x00000001000d6243 in rt::task::Unwinder::try::hdc872d3e23884703Sa0::v0.9$x2dpre ()
#16 0x00000001000d4e05 in rt::task::Task::run::he4b13dcb792462759TaY::v0.9$x2dpre ()
#17 0x00000001000d6eda in rt::task::__extensions__::build_start_wrapper::anon::expr_fn::ab ()
