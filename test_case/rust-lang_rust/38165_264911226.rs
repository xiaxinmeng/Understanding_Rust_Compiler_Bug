
thread 'main' panicked at 'explicit panic', t.rs:2
stack backtrace:
{ skipping 5 frames }
   6:    t::test
             at ./t.rs:2
   7:    t::main::{{closure}}
             at ./t.rs:6
   8:    <core::result::Result<T, E>>::map
             at builtin file libcore/result.rs:465
   9:    t::main
             at ./t.rs:6
{ skipping 6 frames }
