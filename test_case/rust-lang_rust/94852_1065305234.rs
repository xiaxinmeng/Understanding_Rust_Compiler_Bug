plain
    Finished release [optimized] target(s) in 11.12s
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 229 tests
..........i.ii....ii............F........F...........................F.............................. 100/229
.................i...................iiiiiii......i...................iii........................... 200/229
.......F...ii...........F....
Some tests failed in compiletest suite=run-make-fulldeps mode=run-make host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [run-make] run-make-fulldeps/compiler-lookup-paths stdout ----


error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
cc -ffunction-sections -fdata-sections -fPIC -m64 -c -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths/libnative.o native.c
ar crus /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths/libnative.a /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths/libnative.o
mkdir -p /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths/crate
mkdir -p /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths/native
mv /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths/libnative.a /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths/native
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths  a.rs
mv /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths/liba.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths/crate
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths  b.rs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths/crate && exit 1 || exit 0
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths  b.rs -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths/crate && exit 1 || exit 0
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths  b.rs -L crate=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths/crate
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths  b.rs -L all=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths/crate
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths  c.rs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths/crate && exit 1 || exit 0
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths  c.rs -L crate=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths/crate && exit 1 || exit 0
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths  c.rs -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths/crate
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths  c.rs -L all=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths/crate
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths  d.rs -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths/native && exit 1 || exit 0
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths  d.rs -L crate=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths/native && exit 1 || exit 0
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths  d.rs -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths/native
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths  d.rs -L all=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths/native
# Deduplication tests:
#   Same hash, no errors.
mkdir -p /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths/e1
mkdir -p /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths/e2
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths  e.rs -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths/e1/libe.rlib
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths  e.rs -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths/e2/libe.rlib
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths  f.rs -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths/e1 -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths/e2
rm /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths/libnative.o
--- stderr -------------------------------
ar: `u' modifier ignored since `D' is the default (see `U')
error[E0463]: can't find crate for `a`
 --> b.rs:2:1
---

error: aborting due to previous error

For more information about this error, try `rustc --explain E0463`.
error[E0463]: can't find crate for `a` which `b` depends on
 --> c.rs:2:1
2 | extern crate b;
  | ^^^^^^^^^^^^^^^ can't find crate

error: aborting due to previous error
error: aborting due to previous error

For more information about this error, try `rustc --explain E0463`.
error[E0463]: can't find crate for `a` which `b` depends on
 --> c.rs:2:1
2 | extern crate b;
  | ^^^^^^^^^^^^^^^ can't find crate

error: aborting due to previous error
error: aborting due to previous error

For more information about this error, try `rustc --explain E0463`.
error: could not find native static library `native`, perhaps an -L flag is missing?
error: aborting due to previous error


error: could not find native static library `native`, perhaps an -L flag is missing?
error: aborting due to previous error

warning: ignoring --out-dir flag due to -o flag


warning: 1 warning emitted

warning: ignoring --out-dir flag due to -o flag

warning: 1 warning emitted

error[E0465]: multiple rlib candidates for `e` found
 --> f.rs:2:1
2 | extern crate e;
  | ^^^^^^^^^^^^^^^
  |
  |
note: candidate #1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths/e1/libe.rlib
 --> f.rs:2:1
2 | extern crate e;
  | ^^^^^^^^^^^^^^^
  | ^^^^^^^^^^^^^^^
note: candidate #2: /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/compiler-lookup-paths/compiler-lookup-paths/e2/libe.rlib
 --> f.rs:2:1
2 | extern crate e;
  | ^^^^^^^^^^^^^^^

error: aborting due to previous error
error: aborting due to previous error

make: *** [Makefile:27: all] Error 1


---- [run-make] run-make-fulldeps/extern-flag-fun stdout ----


error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-flag-fun/extern-flag-fun:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-flag-fun/extern-flag-fun -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-flag-fun/extern-flag-fun  bar.rs --crate-type=rlib
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-flag-fun/extern-flag-fun:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-flag-fun/extern-flag-fun -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-flag-fun/extern-flag-fun  bar.rs --crate-type=rlib -C extra-filename=-a
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-flag-fun/extern-flag-fun:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-flag-fun/extern-flag-fun -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-flag-fun/extern-flag-fun  bar-alt.rs --crate-type=rlib
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-flag-fun/extern-flag-fun:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-flag-fun/extern-flag-fun -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-flag-fun/extern-flag-fun  foo.rs --extern bar=no-exist && exit 1 || exit 0
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-flag-fun/extern-flag-fun:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-flag-fun/extern-flag-fun -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-flag-fun/extern-flag-fun  foo.rs --extern bar=foo.rs && exit 1 || exit 0
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-flag-fun/extern-flag-fun:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-flag-fun/extern-flag-fun -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-flag-fun/extern-flag-fun  foo.rs \
 --extern bar=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-flag-fun/extern-flag-fun/libbar.rlib \
 --extern bar=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-flag-fun/extern-flag-fun/libbar-alt.rlib \
 && exit 1 || exit 0
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-flag-fun/extern-flag-fun:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-flag-fun/extern-flag-fun -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-flag-fun/extern-flag-fun  foo.rs \
 --extern bar=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-flag-fun/extern-flag-fun/libbar.rlib \
 --extern bar=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-flag-fun/extern-flag-fun/libbar-a.rlib
--- stderr -------------------------------
--- stderr -------------------------------
error: extern location for bar does not exist: no-exist
  |
1 | extern crate bar;
  | ^^^^^^^^^^^^^^^^^

---
  |
1 | extern crate bar;
  | ^^^^^^^^^^^^^^^^^ can't find crate
  |
  = note: extern location for bar is of an unknown type: foo.rs
  = help: file name should be lib*.rlib or lib*..so
error: aborting due to previous error

For more information about this error, try `rustc --explain E0463`.
For more information about this error, try `rustc --explain E0463`.
error: extern location for bar does not exist: /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-flag-fun/extern-flag-fun/libbar-alt.rlib
  |
1 | extern crate bar;
  | ^^^^^^^^^^^^^^^^^


error: aborting due to previous error

error[E0465]: multiple rlib candidates for `bar` found
  |
1 | extern crate bar;
  | ^^^^^^^^^^^^^^^^^
  |
  |
note: candidate #1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-flag-fun/extern-flag-fun/libbar.rlib
  |
1 | extern crate bar;
  | ^^^^^^^^^^^^^^^^^
  | ^^^^^^^^^^^^^^^^^
note: candidate #2: /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-flag-fun/extern-flag-fun/libbar-a.rlib
  |
1 | extern crate bar;
  | ^^^^^^^^^^^^^^^^^


error: aborting due to previous error

make: *** [Makefile:10: all] Error 1



---- [run-make] run-make-fulldeps/issue-11908 stdout ----
error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
--- stdout -------------------------------
mkdir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-11908/issue-11908/other
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-11908/issue-11908:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-11908/issue-11908 -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-11908/issue-11908  foo.rs --crate-type=dylib -C prefer-dynamic
mv /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-11908/issue-11908/libfoo.so /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-11908/issue-11908/other
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-11908/issue-11908:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-11908/issue-11908 -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-11908/issue-11908  foo.rs --crate-type=dylib -C prefer-dynamic
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-11908/issue-11908:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-11908/issue-11908 -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-11908/issue-11908  bar.rs -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-11908/issue-11908/other
--- stderr -------------------------------
--- stderr -------------------------------
error[E0465]: multiple dylib candidates for `foo` found
 --> bar.rs:1:1
1 | extern crate foo;
  | ^^^^^^^^^^^^^^^^^
  |
  |
note: candidate #1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-11908/issue-11908/other/libfoo.so
 --> bar.rs:1:1
1 | extern crate foo;
  | ^^^^^^^^^^^^^^^^^
  | ^^^^^^^^^^^^^^^^^
note: candidate #2: /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-11908/issue-11908/libfoo.so
 --> bar.rs:1:1
1 | extern crate foo;
  | ^^^^^^^^^^^^^^^^^

error: aborting due to previous error
error: aborting due to previous error

make: *** [Makefile:15: all] Error 1


---- [run-make] run-make-fulldeps/reproducible-build stdout ----


error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
rm -rf /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build && mkdir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build  linker.rs -O
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build  reproducible-build-aux.rs
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build  reproducible-build.rs -C linker=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build/linker
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build  reproducible-build.rs -C linker=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build/linker
diff -u "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build/linker-arguments1" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build/linker-arguments2"
rm -rf /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build && mkdir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build  linker.rs -O
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build  reproducible-build-aux.rs -g
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build  reproducible-build.rs -C linker=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build/linker -g
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build  reproducible-build.rs -C linker=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build/linker -g
diff -u "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build/linker-arguments1" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build/linker-arguments2"
rm -rf /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build && mkdir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build  linker.rs -O
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build  reproducible-build-aux.rs -O
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build  reproducible-build.rs -C linker=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build/linker -O
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build  reproducible-build.rs -C linker=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build/linker -O
diff -u "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build/linker-arguments1" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build/linker-arguments2"
rm -rf /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build && mkdir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build  reproducible-build-aux.rs
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build  reproducible-build.rs --crate-type rlib -L /b
cp /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build/libreproducible_build.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build/libfoo.rlib
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build  reproducible-build.rs --crate-type rlib -L /a
cmp "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build/libreproducible_build.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build/libfoo.rlib" || exit 1
/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build/libreproducible_build.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build/reproducible-build/libfoo.rlib differ: char 14575, line 48
--- stderr -------------------------------
--- stderr -------------------------------
make: *** [Makefile:55: link_paths] Error 1



---- [run-make] run-make-fulldeps/reproducible-build-2 stdout ----
error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
--- stdout -------------------------------
rm -rf /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build-2/reproducible-build-2 && mkdir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build-2/reproducible-build-2
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build-2/reproducible-build-2:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build-2/reproducible-build-2 -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build-2/reproducible-build-2  reproducible-build-aux.rs
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build-2/reproducible-build-2:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build-2/reproducible-build-2 -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build-2/reproducible-build-2  reproducible-build.rs -C lto=fat
cp /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build-2/reproducible-build-2/reproducible-build /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build-2/reproducible-build-2/reproducible-build-a
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build-2/reproducible-build-2:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build-2/reproducible-build-2 -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build-2/reproducible-build-2  reproducible-build.rs -C lto=fat
cmp "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build-2/reproducible-build-2/reproducible-build-a" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build-2/reproducible-build-2/reproducible-build" || exit 1
rm -rf /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build-2/reproducible-build-2 && mkdir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build-2/reproducible-build-2
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build-2/reproducible-build-2:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build-2/reproducible-build-2 -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build-2/reproducible-build-2  reproducible-build-aux.rs
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build-2/reproducible-build-2:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build-2/reproducible-build-2 -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build-2/reproducible-build-2  reproducible-build.rs --crate-type rlib --sysroot /checkout/obj/build/x86_64-unknown-linux-gnu/stage2 --remap-path-prefix=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2=/sysroot
cp -R /checkout/obj/build/x86_64-unknown-linux-gnu/stage2 /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build-2/reproducible-build-2/sysroot
cp /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build-2/reproducible-build-2/libreproducible_build.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build-2/reproducible-build-2/libfoo.rlib
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build-2/reproducible-build-2:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build-2/reproducible-build-2 -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build-2/reproducible-build-2  reproducible-build.rs --crate-type rlib --sysroot /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build-2/reproducible-build-2/sysroot --remap-path-prefix=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build-2/reproducible-build-2/sysroot=/sysroot
cmp "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build-2/reproducible-build-2/libreproducible_build.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build-2/reproducible-build-2/libfoo.rlib" || exit 1
/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build-2/reproducible-build-2/libreproducible_build.rlib /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/reproducible-build-2/reproducible-build-2/libfoo.rlib differ: char 14577, line 48
--- stderr -------------------------------
--- stderr -------------------------------
make: *** [Makefile:26: sysroot] Error 1



failures:
