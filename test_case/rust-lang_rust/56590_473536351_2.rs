
   Compiling repro v0.1.0 (/private/tmp/r)
error[E0465]: multiple rlib candidates for `debug_unreachable` found
  |
note: candidate #1: /private/tmp/r/target/debug/deps/libdebug_unreachable-9846113be7cd0a2b.rlib
note: candidate #2: /private/tmp/r/target/debug/deps/libdebug_unreachable-843675cb387b3fdc.rlib

error[E0412]: cannot find type `Derp` in this scope
 --> src/main.rs:3:15
  |
3 | struct Wat(Rc<Derp>);
  |               ^^^^ not found in this scope
