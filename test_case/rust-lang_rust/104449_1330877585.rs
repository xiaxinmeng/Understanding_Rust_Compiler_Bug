plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between c372b1470109547e37dcc29ffd4723ac9f66f15d and 6bf996479359d2dcf3e3c7f18de53791b62a8dac
Tool subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
tests/fail/shims/sync/libc_pthread_rwlock_write_wrong_owner.rs ... ok
tests/fail/panic/double_panic.rs ... ok

tests/fail/stacked_borrows/box_noalias_violation.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--error-format=json" "--edition" "2018" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "x86_64-unknown-linux-gnu" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-581ef5fab20fc228.rmeta" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-565e790f1ffc9f2f.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-146dc4a50e00d092.rmeta" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-f652634992780220.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-caab810b4b176c87.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-05ebe94257021903.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-7772657900252095.rmeta" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-e0f73d7a3f2987a4" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/tokio-4651019dff74fe87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-10dd9dcfa669cfb7" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-2402edfbc2151f87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-bbb475710904fa81" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-f6345840f8fd0648" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-4b90074282ccb75b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "tests/fail/stacked_borrows/box_noalias_violation.rs"
actual output differed from expected
--- tests/fail/stacked_borrows/box_noalias_violation.stderr
+++ <stderr output>
... 18 lines skipped ...
---
... 8 lines skipped ...


full stderr:
error: Undefined Behavior: not granting access to tag <2972> because that would remove [Unique for <2990>] which is weakly protected because it is an argument of call 840
   |
LL |     *y
LL |     *y
   |     ^^ not granting access to tag <2972> because that would remove [Unique for <2990>] which is weakly protected because it is an argument of call 840
   = help: this indicates a potential bug in the program: it performed an invalid operation, but the Stacked Borrows rules it violated are still experimental
   = help: see https://github.com/rust-lang/unsafe-code-guidelines/blob/master/wip/stacked-borrows.md for further information
   = help: see https://github.com/rust-lang/unsafe-code-guidelines/blob/master/wip/stacked-borrows.md for further information
help: <2972> was created by a SharedReadWrite retag at offsets [0x0..0x4]
   |
LL |         let ptr = &mut v as *mut i32;
   |                   ^^^^^^
   |                   ^^^^^^
help: <2990> is this argument
   |
   |
LL | unsafe fn test(mut x: Box<i32>, y: *const i32) -> i32 {
   = note: BACKTRACE:
   = note: inside `test` at tests/fail/stacked_borrows/box_noalias_violation.rs:5:5
note: inside `main`
  --> tests/fail/stacked_borrows/box_noalias_violation.rs:12:9
---



tests/fail/stacked_borrows/illegal_dealloc1.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--error-format=json" "--edition" "2018" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "x86_64-unknown-linux-gnu" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-581ef5fab20fc228.rmeta" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-565e790f1ffc9f2f.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-146dc4a50e00d092.rmeta" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-f652634992780220.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-caab810b4b176c87.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-05ebe94257021903.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-7772657900252095.rmeta" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-e0f73d7a3f2987a4" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/tokio-4651019dff74fe87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-10dd9dcfa669cfb7" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-2402edfbc2151f87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-bbb475710904fa81" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-f6345840f8fd0648" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-4b90074282ccb75b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "tests/fail/stacked_borrows/illegal_dealloc1.rs"
actual output differed from expected
--- tests/fail/stacked_borrows/illegal_dealloc1.stderr
+++ <stderr output>
... 18 lines skipped ...
---
... 8 lines skipped ...


full stderr:
error: Undefined Behavior: attempting deallocation using <2993> at alloc1592, but that tag does not exist in the borrow stack for this location
   |
LL |     unsafe { __rust_dealloc(ptr, layout.size(), layout.align()) }
LL |     unsafe { __rust_dealloc(ptr, layout.size(), layout.align()) }
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ attempting deallocation using <2993> at alloc1592, but that tag does not exist in the borrow stack for this location
   = help: this indicates a potential bug in the program: it performed an invalid operation, but the Stacked Borrows rules it violated are still experimental
   = help: see https://github.com/rust-lang/unsafe-code-guidelines/blob/master/wip/stacked-borrows.md for further information
   = help: see https://github.com/rust-lang/unsafe-code-guidelines/blob/master/wip/stacked-borrows.md for further information
help: <2993> was created by a SharedReadWrite retag at offsets [0x0..0x1]
   |
   |
LL |         let ptr2 = (&mut *ptr1) as *mut u8;
   |                    ^^^^^^^^^^^^
help: <2993> was later invalidated at offsets [0x0..0x1] by a write access
   |
LL |         ptr1.write(0);
   |         ^^^^^^^^^^^^^
   = note: BACKTRACE:
---



tests/fail/stacked_borrows/retag_data_race_write.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--error-format=json" "--edition" "2018" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "x86_64-unknown-linux-gnu" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-581ef5fab20fc228.rmeta" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-565e790f1ffc9f2f.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-146dc4a50e00d092.rmeta" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-f652634992780220.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-caab810b4b176c87.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-05ebe94257021903.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-7772657900252095.rmeta" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-e0f73d7a3f2987a4" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/tokio-4651019dff74fe87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-10dd9dcfa669cfb7" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-2402edfbc2151f87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-bbb475710904fa81" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-f6345840f8fd0648" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-4b90074282ccb75b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "tests/fail/stacked_borrows/retag_data_race_write.rs" "-Zmiri-preemption-rate=0"
actual output differed from expected
--- tests/fail/stacked_borrows/retag_data_race_write.stderr
+++ <stderr output>
... 8 lines skipped ...
---
... 8 lines skipped ...


full stderr:
error: Undefined Behavior: Data race detected between Write on thread `<unnamed>` and Write on thread `<unnamed>` at alloc1584
   |
LL |         *p = 5;
LL |         *p = 5;
   |         ^^^^^^ Data race detected between Write on thread `<unnamed>` and Write on thread `<unnamed>` at alloc1584
   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
   = note: BACKTRACE:
   = note: inside `thread_2` at tests/fail/stacked_borrows/retag_data_race_write.rs:18:9
   = note: inside `thread_2` at tests/fail/stacked_borrows/retag_data_race_write.rs:18:9
note: inside closure
  --> tests/fail/stacked_borrows/retag_data_race_write.rs:28:41
   |
LL |     let t2 = std::thread::spawn(move || thread_2(p));

note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace

error: aborting due to previous error
error: aborting due to previous error




tests/fail/stacked_borrows/retag_data_race_read.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--error-format=json" "--edition" "2018" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "x86_64-unknown-linux-gnu" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-581ef5fab20fc228.rmeta" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-565e790f1ffc9f2f.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-146dc4a50e00d092.rmeta" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-f652634992780220.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-caab810b4b176c87.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-05ebe94257021903.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-7772657900252095.rmeta" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-e0f73d7a3f2987a4" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/tokio-4651019dff74fe87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-10dd9dcfa669cfb7" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-2402edfbc2151f87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-bbb475710904fa81" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-f6345840f8fd0648" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-4b90074282ccb75b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "tests/fail/stacked_borrows/retag_data_race_read.rs" "-Zmiri-preemption-rate=0"
actual output differed from expected
--- tests/fail/stacked_borrows/retag_data_race_read.stderr
+++ <stderr output>
... 8 lines skipped ...
---
... 8 lines skipped ...


full stderr:
error: Undefined Behavior: Data race detected between Write on thread `<unnamed>` and Read on thread `<unnamed>` at alloc1584
   |
LL |         *p = 5;
LL |         *p = 5;
   |         ^^^^^^ Data race detected between Write on thread `<unnamed>` and Read on thread `<unnamed>` at alloc1584
   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
   = note: BACKTRACE:
   = note: inside `thread_2` at tests/fail/stacked_borrows/retag_data_race_read.rs:18:9
   = note: inside `thread_2` at tests/fail/stacked_borrows/retag_data_race_read.rs:18:9
note: inside closure
  --> tests/fail/stacked_borrows/retag_data_race_read.rs:28:41
   |
LL |     let t2 = std::thread::spawn(move || thread_2(p));

note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace

error: aborting due to previous error
