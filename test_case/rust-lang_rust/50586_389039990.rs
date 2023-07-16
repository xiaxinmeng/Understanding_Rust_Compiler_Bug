
==75691== Process terminating with default action of signal 11 (SIGSEGV)
==75691==  General Protection Fault
==75691==    at 0x100151B0B: thread_local::thread_id::get (in target/release/oida)
==75691==    by 0x1000FC3E5: regex::re_unicode::Regex::captures (in target/release/oida)
==75691==    by 0x100032CEB: oida::index::EventIndex::feed (in target/release/oida)
==75691==    by 0x10003C742: oida::main (in target/release/oida)
==75691==    by 0x10002ECE5: std::rt::lang_start::{{closure}} (in target/release/oida)
==75691==    by 0x100172477: _ZN3std9panicking3try7do_call17hc7b9e6190a1d9f3eE.llvm.7628423903506348692 (rt.rs:59)
==75691==    by 0x10018655E: __rust_maybe_catch_panic (lib.rs:102)
==75691==    by 0x100170E01: std::rt::lang_start_internal (panicking.rs:285)
==75691==    by 0x10003E16B: main (in target/release/oida)
