
$ cat src/main.rs
fn main() {}

$ RUSTFLAGS="-Z sanitizer=address" xargo run
   Compiling oob v0.1.0 (file:///home/japaric/tmp/oob)
    Finished debug [unoptimized + debuginfo] target(s) in 0.53 secs
     Running `target/debug/oob`
=================================================================
==5051==ERROR: AddressSanitizer: odr-violation (0x5579099b23a0):
  [1] size=0 'ref.1O' core.cgu-0.rs
  [2] size=0 'ref.19' std.cgu-0.rs
These globals were registered at these points:
  [1]:
    #0 0x5579098c66e0 in __asan_register_globals.part.10 /shared/rust/checkouts/lsan/src/compiler-rt/lib/asan/asan_globals.cc:309
    #1 0x5579099b0dfb in asan.module_ctor (/home/japaric/tmp/oob/target/debug/oob+0x188dfb)

  [2]:
    #0 0x5579098c66e0 in __asan_register_globals.part.10 /shared/rust/checkouts/lsan/src/compiler-rt/lib/asan/asan_globals.cc:309
    #1 0x5579098a8b5b in asan.module_ctor (/home/japaric/tmp/oob/target/debug/oob+0x80b5b)

==5051==HINT: if you don't care about these errors you may set ASAN_OPTIONS=detect_odr_violation=0
SUMMARY: AddressSanitizer: odr-violation: global 'ref.1O' at core.cgu-0.rs
==5051==ABORTING
