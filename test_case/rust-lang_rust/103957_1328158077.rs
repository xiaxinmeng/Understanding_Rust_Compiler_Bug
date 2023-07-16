plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 80a96467ec5675e9f69683b5c075a8b15950c341 and 5c180cfbff9676d1ade0db9a3e4076509713b85d
Tool subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
tests/fail/weak_memory/racing_mixed_size_read.rs ... ok
tests/fail/weak_memory/racing_mixed_size.rs ... ok
tests/fail/panic/double_panic.rs ... ok

tests/fail/stacked_borrows/drop_in_place.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--error-format=json" "--edition" "2018" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "i686-pc-windows-msvc" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/i686-pc-windows-msvc/debug/deps/libgetrandom-b554f1c6889b9fe8.rmeta" "--extern" "getrandom_2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/i686-pc-windows-msvc/debug/deps/libgetrandom-a39ba97a6f22969b.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/i686-pc-windows-msvc/debug/deps/liblibc-08e58e23ea854b0f.rmeta" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/i686-pc-windows-msvc/debug/deps/libnum_cpus-ad74e127045aa788.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/i686-pc-windows-msvc/debug/deps/libpage_size-4480a213b36b946b.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/i686-pc-windows-msvc/debug/deps/librand-1c84d5906b6f3aa4.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/i686-pc-windows-msvc/debug/deps/libtokio-d1596feda2ef395d.rmeta" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-de8d28cfa3a1467f" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-3bb8f7ac0d0402d5" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/tokio-8b7b2e3011a6b15c" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/i686-pc-windows-msvc/debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-19a0c4dd785bdb63" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/windows_i686_msvc-8f30207d97b46e66" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/winapi-64382837588e3f8c" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-eeb167ea843a342f" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-bbb475710904fa81" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-2402edfbc2151f87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/i686-pc-windows-msvc/debug/deps" "tests/fail/stacked_borrows/drop_in_place.rs"
actual output differed from expected
--- tests/fail/stacked_borrows/drop_in_place.stderr
+++ <stderr output>
... 6 lines skipped ...
... 6 lines skipped ...
    = help: this indicates a potential bug in the program: it performed an invalid operation, but the Stacked Borrows rules it violated are still experimental
    = help: see https://github.com/rust-lang/unsafe-code-guidelines/blob/master/wip/stacked-borrows.md for further information
-help: <TAG> was created by a SharedReadWrite retag at offsets [0x0..0x10]
+help: <TAG> was created by a SharedReadWrite retag at offsets [0x0..0x8]
    |
... 22 lines skipped ...



full stderr:
error: Undefined Behavior: not granting access to tag <4697> because that would remove [Unique for <4700>] which is protected because it is an argument of call 1311
  --> tests/fail/stacked_borrows/drop_in_place.rs:10:22
   |
LL |             let _x = *(self.p);
   |                      ^^^^^^^^^ not granting access to tag <4697> because that would remove [Unique for <4700>] which is protected because it is an argument of call 1311
   = help: this indicates a potential bug in the program: it performed an invalid operation, but the Stacked Borrows rules it violated are still experimental
   = help: see https://github.com/rust-lang/unsafe-code-guidelines/blob/master/wip/stacked-borrows.md for further information
   = help: see https://github.com/rust-lang/unsafe-code-guidelines/blob/master/wip/stacked-borrows.md for further information
help: <4697> was created by a SharedReadWrite retag at offsets [0x0..0x8]
  --> tests/fail/stacked_borrows/drop_in_place.rs:18:17
   |
LL |         let x = core::ptr::addr_of_mut!(x);
   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^
help: <4700> is this argument
  --> tests/fail/stacked_borrows/drop_in_place.rs:24:9
LL |         core::ptr::drop_in_place(x);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: BACKTRACE:
   = note: inside `<SelfRef as std::ops::Drop>::drop` at tests/fail/stacked_borrows/drop_in_place.rs:10:22
   = note: inside `<SelfRef as std::ops::Drop>::drop` at tests/fail/stacked_borrows/drop_in_place.rs:10:22
   = note: inside `std::ptr::drop_in_place::<SelfRef> - shim(Some(SelfRef))` at /checkout/library/core/src/ptr/mod.rs:490:1
note: inside `main` at tests/fail/stacked_borrows/drop_in_place.rs:24:9
  --> tests/fail/stacked_borrows/drop_in_place.rs:24:9
LL |         core::ptr::drop_in_place(x);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: this error originates in the macro `core::ptr::addr_of_mut` (in Nightly builds, run with -Z macro-backtrace for more info)

