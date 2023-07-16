plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:24cb9080177205b6e8c946b17badbe402adc938f)
Download action repository 'rust-lang/simpleinfra@master' (SHA:3fb2b44a4eaebb9ed8086446bde46c27199ef5ed)
Complete job name: PR (x86_64-gnu-tools, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
tests/fail/shims/sync/libc_pthread_rwlock_write_wrong_owner.rs ... ok
tests/fail/panic/double_panic.rs ... ok

tests/fail/stacked_borrows/drop_in_place_retag.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--error-format=json" "--edition" "2018" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "x86_64-unknown-linux-gnu" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rmeta" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rmeta" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-0f7911dcb05eb597.rmeta" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-2402edfbc2151f87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-83e38450697cf16b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-d339ad05ba9c2010" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-56af15a1a9078f46" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-ec0977e83bec4bbb" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-023a677652b471f9" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/tokio-bfd338245ca0e2e7" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "tests/fail/stacked_borrows/drop_in_place_retag.rs"
actual output differed from expected
--- tests/fail/stacked_borrows/drop_in_place_retag.stderr
+++ <stderr output>
    |
    |
   --> RUSTLIB/core/src/ptr/mod.rs:LL:CC
    |
-LL | pub unsafe fn drop_in_place<T: ?Sized>(to_drop: *mut T) {
-   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+LL | pub const unsafe fn drop_in_place<T: ?Sized + ~const Destruct>(to_drop: *mut T) {
    | |
    | |
    | trying to retag from <TAG> for Unique permission at ALLOC[0x0], but that tag only grants SharedReadOnly permission for this location


full stderr:
full stderr:
error: Undefined Behavior: trying to retag from <2791> for Unique permission at alloc1467[0x0], but that tag only grants SharedReadOnly permission for this location
   |
   |
LL | pub const unsafe fn drop_in_place<T: ?Sized + ~const Destruct>(to_drop: *mut T) {
   | |
   | |
   | trying to retag from <2791> for Unique permission at alloc1467[0x0], but that tag only grants SharedReadOnly permission for this location
   | this error occurs as part of retag at alloc1467[0x0..0x1]
   = help: this indicates a potential bug in the program: it performed an invalid operation, but the Stacked Borrows rules it violated are still experimental
   = help: see https://github.com/rust-lang/unsafe-code-guidelines/blob/master/wip/stacked-borrows.md for further information
   = help: see https://github.com/rust-lang/unsafe-code-guidelines/blob/master/wip/stacked-borrows.md for further information
help: <2791> was created by a SharedReadOnly retag at offsets [0x0..0x1]
   |
   |
LL |         let x = core::ptr::addr_of!(x);
   = note: BACKTRACE (of the first span):
   = note: BACKTRACE (of the first span):
   = note: inside `std::ptr::drop_in_place::<u8> - shim(None)` at /checkout/library/core/src/ptr/mod.rs:492:1: 492:80
  --> tests/fail/stacked_borrows/drop_in_place_retag.rs:10:9
   |
   |
LL |         core::ptr::drop_in_place(x.cast_mut());
   = note: this error originates in the macro `core::ptr::addr_of` (in Nightly builds, run with -Z macro-backtrace for more info)

note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace


error: aborting due to previous error




tests/fail/unaligned_pointers/drop_in_place.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--error-format=json" "--edition" "2018" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "x86_64-unknown-linux-gnu" "--extern" "getrandom=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-14b1aaee86e20c9a.rmeta" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-522d1fe1f3fb1413.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-8df47f1ee3f80022.rmeta" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-b37fde289af25dca.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-be16f3668653c591.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-2709be2ca9b220ed.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-0f7911dcb05eb597.rmeta" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-2402edfbc2151f87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-83e38450697cf16b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-d339ad05ba9c2010" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-56af15a1a9078f46" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-ec0977e83bec4bbb" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-023a677652b471f9" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/tokio-bfd338245ca0e2e7" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "tests/fail/unaligned_pointers/drop_in_place.rs"
actual output differed from expected
--- tests/fail/unaligned_pointers/drop_in_place.stderr
+++ <stderr output>
    |
    |
   --> RUSTLIB/core/src/ptr/mod.rs:LL:CC
    |
-LL | pub unsafe fn drop_in_place<T: ?Sized>(to_drop: *mut T) {
-   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ accessing memory with alignment ALIGN, but alignment ALIGN is required
+LL | pub const unsafe fn drop_in_place<T: ?Sized + ~const Destruct>(to_drop: *mut T) {
+   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ accessing memory with alignment ALIGN, but alignment ALIGN is required
    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
... 14 lines skipped ...



full stderr:
error: Undefined Behavior: accessing memory with alignment 1, but alignment 2 is required
  --> /checkout/library/core/src/ptr/mod.rs:492:1
   |
LL | pub const unsafe fn drop_in_place<T: ?Sized + ~const Destruct>(to_drop: *mut T) {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ accessing memory with alignment 1, but alignment 2 is required
   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
   = note: BACKTRACE:
   = note: BACKTRACE:
   = note: inside `std::ptr::drop_in_place::<PartialDrop> - shim(Some(PartialDrop))` at /checkout/library/core/src/ptr/mod.rs:492:1: 492:80
  --> tests/fail/unaligned_pointers/drop_in_place.rs:23:9
   |
LL |         core::ptr::drop_in_place(p);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^
