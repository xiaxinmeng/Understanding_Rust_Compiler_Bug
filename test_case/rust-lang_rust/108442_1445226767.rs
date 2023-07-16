plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:697bea7ddceb6696743da8f159f268aef8bfb3c6)
Complete job name: PR (x86_64-gnu-tools, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
tests/fail/shims/sync/libc_pthread_rwlock_write_write_deadlock.rs ... ok
tests/fail/panic/double_panic.rs ... ok

tests/fail/never_transmute_humans.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--error-format=json" "--edition" "2018" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "x86_64-unknown-linux-gnu" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rmeta" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rmeta" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-97ba768f8b612a21.rmeta" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-023a677652b471f9" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-56af15a1a9078f46" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-d339ad05ba9c2010" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/tokio-627adf877df7d3fd" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-ec0977e83bec4bbb" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-83e38450697cf16b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-2402edfbc2151f87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "tests/fail/never_transmute_humans.rs" "-Zmiri-disable-validation"
actual output differed from expected
--- tests/fail/never_transmute_humans.stderr
+++ <stderr output>
-error: Undefined Behavior: transmuting to uninhabited type
-error: Undefined Behavior: transmuting to uninhabited type
+error: Undefined Behavior: entering unreachable code
   --> $DIR/never_transmute_humans.rs:LL:CC
    |
 LL |         std::mem::transmute::<Human, !>(Human)
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ transmuting to uninhabited type
+   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ entering unreachable code
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
... 9 lines skipped ...


---
full stderr:
error: Undefined Behavior: entering unreachable code
  --> tests/fail/never_transmute_humans.rs:10:9
   |
LL |         std::mem::transmute::<Human, !>(Human)
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ entering unreachable code
   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
   = note: BACKTRACE:
   = note: inside `main` at tests/fail/never_transmute_humans.rs:10:9: 10:47
