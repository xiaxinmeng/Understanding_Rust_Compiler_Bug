
% multirust run nightly cargo build
   Compiling regex v0.1.30
   Compiling libc v0.1.8
   Compiling log v0.3.1
   Compiling rand v0.3.8
   Compiling getopts v0.2.11
   Compiling tempfile v0.3.0
   Compiling env_logger v0.3.1
   Compiling rusti v0.0.1 (file:///l/src/rust/rusti)

% ./target/debug/rusti
dyld: Library not loaded: x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/librustc_driver-11582ce5.dylib
  Referenced from: /l/src/rust/rusti/./target/debug/rusti
  Reason: image not found
Trace/BPT trap: 5 (core dumped)

% multirust run nightly bash
% ./target/debug/rusti
rusti=>
