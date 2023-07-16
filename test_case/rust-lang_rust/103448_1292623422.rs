plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 0a6b941df354c59b546ec4c0d27f2b9b0cb1162c and 3332f086add67442e68681ca485b54326d9effc3
Tool subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
tests/fail/shims/sync/libc_pthread_rwlock_write_wrong_owner.rs ... ok
tests/fail/panic/double_panic.rs ... ok

tests/fail/stacked_borrows/newtype_pair_retagging.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--error-format=json" "--edition" "2018" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "x86_64-unknown-linux-gnu" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-ac993ae34a47d010.rmeta" "--extern" "getrandom_2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-20b4cd671fbdd122.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-5c91b8ee2f9a761f.rmeta" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-f87ab7e9e692ab6c.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-c9761f548003f841.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-5e75cbc746a2ceda.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-664c0f42e4d71cb6.rmeta" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-19a0c4dd785bdb63" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-bbb475710904fa81" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-2402edfbc2151f87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/tokio-5b679bca69988a36" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-3bb8f7ac0d0402d5" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-de8d28cfa3a1467f" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-eeb167ea843a342f" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "tests/fail/stacked_borrows/newtype_pair_retagging.rs" "-Zmiri-retag-fields=scalar"
actual output differed from expected
--- tests/fail/stacked_borrows/newtype_pair_retagging.stderr
+++ <stderr output>
... 24 lines skipped ...
... 24 lines skipped ...
 LL |             || drop(Box::from_raw(ptr)),
    |                     ^^^^^^^^^^^^^^^^^^
~note: inside `dealloc_while_running::<[closure@$DIR/newtype_pair_retagging.rs:LL:CC], '_>` at $DIR/newtype_pair_retagging.rs:LL:CC
    |
... 16 lines skipped ...



full stderr:
error: Undefined Behavior: not granting access to tag <3169> because that would remove [Unique for <3188>] which is protected because it is an argument of call 890
   |
LL |         Box(unsafe { Unique::new_unchecked(raw) }, alloc)
LL |         Box(unsafe { Unique::new_unchecked(raw) }, alloc)
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not granting access to tag <3169> because that would remove [Unique for <3188>] which is protected because it is an argument of call 890
   = help: this indicates a potential bug in the program: it performed an invalid operation, but the Stacked Borrows rules it violated are still experimental
   = help: see https://github.com/rust-lang/unsafe-code-guidelines/blob/master/wip/stacked-borrows.md for further information
   = help: see https://github.com/rust-lang/unsafe-code-guidelines/blob/master/wip/stacked-borrows.md for further information
help: <3169> was created by a SharedReadWrite retag at offsets [0x0..0x4]
   |
   |
LL |     let ptr = Box::into_raw(Box::new(0i32));
   |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: <3188> is this argument
   |
   |
LL | fn dealloc_while_running(_n: Newtype<'_>, dealloc: impl FnOnce()) {
   = note: BACKTRACE:
   = note: inside `std::boxed::Box::<i32>::from_raw_in` at /checkout/library/alloc/src/boxed.rs:1013:9
   = note: inside `std::boxed::Box::<i32>::from_raw` at /checkout/library/alloc/src/boxed.rs:957:18
note: inside closure at tests/fail/stacked_borrows/newtype_pair_retagging.rs:16:21
note: inside closure at tests/fail/stacked_borrows/newtype_pair_retagging.rs:16:21
  --> tests/fail/stacked_borrows/newtype_pair_retagging.rs:16:21
   |
LL |             || drop(Box::from_raw(ptr)),
   |                     ^^^^^^^^^^^^^^^^^^
note: inside `dealloc_while_running::<[closure@tests/fail/stacked_borrows/newtype_pair_retagging.rs:16:13: 16:15], '_>` at tests/fail/stacked_borrows/newtype_pair_retagging.rs:6:5
   |
LL |     dealloc();
Build completed unsuccessfully in 0:04:12
   |     ^^^^^^^^^
   |     ^^^^^^^^^
note: inside `main` at tests/fail/stacked_borrows/newtype_pair_retagging.rs:14:9
  --> tests/fail/stacked_borrows/newtype_pair_retagging.rs:14:9
   |
LL | /         dealloc_while_running(
LL | |             Newtype(&mut *ptr, 0),
LL | |             || drop(Box::from_raw(ptr)),
LL | |         )

note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace

error: aborting due to previous error
