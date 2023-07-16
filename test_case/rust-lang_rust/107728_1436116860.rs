plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:75573f9759179a720f4c3af6c9fb518ac0061dca)
Complete job name: PR (x86_64-gnu-tools, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
tests/fail/shims/sync/libc_pthread_rwlock_write_wrong_owner.rs ... ok
tests/fail/panic/double_panic.rs ... ok

tests/fail/branchless-select-i128-pointer.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--error-format=json" "--edition" "2018" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "x86_64-unknown-linux-gnu" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rmeta" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rmeta" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-97ba768f8b612a21.rmeta" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-d339ad05ba9c2010" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-56af15a1a9078f46" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-ec0977e83bec4bbb" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-83e38450697cf16b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-023a677652b471f9" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/tokio-627adf877df7d3fd" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-2402edfbc2151f87" "tests/fail/branchless-select-i128-pointer.rs"
actual output differed from expected
--- tests/fail/branchless-select-i128-pointer.stderr
+++ <stderr output>
+++ <stderr output>
-error: Undefined Behavior: constructing invalid value: encountered a dangling reference (address $HEX is unallocated)
+error: Undefined Behavior: constructing invalid value: encountered a dangling reference ($HEX[noalloc] has no provenance)
    |
... 3 lines skipped ...
... 3 lines skipped ...
 LL | |                     | mask & transmute::<_, TwoPtrs>("true !"),
 LL | |             )
-   | |_____________^ constructing invalid value: encountered a dangling reference (address $HEX is unallocated)
+   | |_____________^ constructing invalid value: encountered a dangling reference ($HEX[noalloc] has no provenance)
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
... 9 lines skipped ...



full stderr:
error: Undefined Behavior: constructing invalid value: encountered a dangling reference (0x283e5[noalloc] has no provenance)
   |
   |
LL | /             transmute::<_, &str>(
LL | |
LL | |                 !mask & transmute::<_, TwoPtrs>("false !")
LL | |                     | mask & transmute::<_, TwoPtrs>("true !"),
LL | |             )
   | |_____________^ constructing invalid value: encountered a dangling reference (0x283e5[noalloc] has no provenance)
   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
   = note: BACKTRACE:
   = note: inside `main` at tests/fail/branchless-select-i128-pointer.rs:14:13: 18:14
---



tests/fail/validity/dangling_ref1.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--error-format=json" "--edition" "2018" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "x86_64-unknown-linux-gnu" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rmeta" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rmeta" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-97ba768f8b612a21.rmeta" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-d339ad05ba9c2010" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-56af15a1a9078f46" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-ec0977e83bec4bbb" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-83e38450697cf16b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-023a677652b471f9" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/tokio-627adf877df7d3fd" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-2402edfbc2151f87" "tests/fail/validity/dangling_ref1.rs" "-Zmiri-disable-stacked-borrows"
actual output differed from expected
--- tests/fail/validity/dangling_ref1.stderr
+++ <stderr output>
+++ <stderr output>
-error: Undefined Behavior: constructing invalid value: encountered a dangling reference (address 0x10 is unallocated)
+error: Undefined Behavior: constructing invalid value: encountered a dangling reference (0x10[noalloc] has no provenance)
   --> $DIR/dangling_ref1.rs:LL:CC
    |
 LL |     let _x: &i32 = unsafe { mem::transmute(16usize) };
-   |                             ^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered a dangling reference (address 0x10 is unallocated)
+   |                             ^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered a dangling reference (0x10[noalloc] has no provenance)
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
... 9 lines skipped ...



substring `encountered a dangling reference (address 0x10 is unallocated)` not found in stderr output

There were 1 unmatched diagnostics at tests/fail/validity/dangling_ref1.rs:6
There were 1 unmatched diagnostics at tests/fail/validity/dangling_ref1.rs:6
    Error: Undefined Behavior: constructing invalid value: encountered a dangling reference (0x10[noalloc] has no provenance)
full stderr:
full stderr:
error: Undefined Behavior: constructing invalid value: encountered a dangling reference (0x10[noalloc] has no provenance)
   |
   |
LL |     let _x: &i32 = unsafe { mem::transmute(16usize) };
   |                             ^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered a dangling reference (0x10[noalloc] has no provenance)
   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
   = note: BACKTRACE:
   = note: inside `main` at tests/fail/validity/dangling_ref1.rs:6:29: 6:52
