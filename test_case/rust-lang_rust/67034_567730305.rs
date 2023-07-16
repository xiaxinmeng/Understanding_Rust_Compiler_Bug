
panic: OMG EVERYTHING IS ON FIRE!!!
  --> tests/custom-panic/src/main.rs:17:3
   |
17 |   panic!("OMG EVERYTHING IS ON FIRE!!!");
   |   ^~~
  ::: tests/custom-panic/src/main.rs:13:1
   |
13 |   foo();
   |   ^~~ 0: 0x103509ce0 - custom_panic_test::main::h20ecc527e431a0b3
  ::: /rustc/19bd93467617a447c22ec32cc1cf14d40cb84ccf/src/libstd/rt.rs:67:1
   | 0: 0x103509c12 - std::rt::lang_start::{{closure}}::hb6bdba643f51f66f
  ::: src/libstd/rt.rs:52:1
   | 0: 0x103563548 - std::rt::lang_start_internal::{{closure}}::hc58c6d65eddab43f
   ::: src/libstd/panicking.rs:292:1
    | 1: 0x103563548 - std::panicking::try::do_call::h1693ed59f66b9306
  ::: src/libpanic_unwind/lib.rs:78:1
   | 0: 0x10356504f - __rust_maybe_catch_panic
   ::: src/libstd/panicking.rs:270:1
    | 0: 0x103563ede - std::panicking::try::h16546adfad4ff0ff
   ::: src/libstd/panic.rs:394:1
    | 1: 0x103563ede - std::panic::catch_unwind::h9f8111749c7f253d
  ::: src/libstd/rt.rs:51:1
   | 2: 0x103563ede - std::rt::lang_start_internal::h3806eae227db69b6
  ::: /rustc/19bd93467617a447c22ec32cc1cf14d40cb84ccf/src/libstd/rt.rs:67:1
   | 0: 0x103509bf2 - std::rt::lang_start::hb9532b1815095118
