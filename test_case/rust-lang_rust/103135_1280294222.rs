plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between a501e6699ed979ef0540949211fc28256336a3f3 and 06ba4bc1531cd4eee7ef342ef3dbdf352ff95ec6
Tool subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
tests/pass-dep/concurrency/libc_pthread_cond.rs ... ok
tests/pass-dep/concurrency/linux-futex.rs ... ok

tests/pass-dep/shims/fs.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--error-format=json" "--edition" "2018" "-Astable-features" "-Aunused" "-Zui-testing" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-ac993ae34a47d010.rmeta" "--extern" "getrandom_2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-20b4cd671fbdd122.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-5c91b8ee2f9a761f.rmeta" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-f87ab7e9e692ab6c.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-c9761f548003f841.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-5e75cbc746a2ceda.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-664c0f42e4d71cb6.rmeta" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-3bb8f7ac0d0402d5" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-19a0c4dd785bdb63" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-de8d28cfa3a1467f" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/tokio-5b679bca69988a36" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-eeb167ea843a342f" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-2402edfbc2151f87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-bbb475710904fa81" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "tests/pass-dep/shims/fs.rs" "-Zmiri-disable-isolation"
Pass got exit status: 1

actual output differed from expected
--- tests/pass-dep/shims/fs.stderr
--- tests/pass-dep/shims/fs.stderr
+++ <stderr output>
-hello dup fd
+error: Undefined Behavior: dereferencing pointer failed: ALLOC has size 31, so pointer to 280 bytes starting at offset 0 is out-of-bounds
+  --> RUSTLIB/std/src/sys/PLATFORM/fs.rs:LL:CC
+   |
+LL |                 let name = CStr::from_ptr(ptr::addr_of!((*entry_ptr).d_name) as *const c_char);
+   |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ dereferencing pointer failed: ALLOC has size 31, so pointer to 280 bytes starting at offset 0 is out-of-bounds
+   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
+   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
+   = note: BACKTRACE:
+   = note: inside `<std::sys::PLATFORM::fs::ReadDir as std::iter::Iterator>::next` at RUSTLIB/core/src/ptr/mod.rs:LL:CC
---
-hello dup fd


There were 1 unmatched diagnostics that occurred outside the testfile and had no pattern
    Error: Undefined Behavior: dereferencing pointer failed: alloc59726 has size 31, so pointer to 280 bytes starting at offset 0 is out-of-bounds
full stderr:
full stderr:
error: Undefined Behavior: dereferencing pointer failed: alloc59726 has size 31, so pointer to 280 bytes starting at offset 0 is out-of-bounds
   |
   |
LL |                 let name = CStr::from_ptr(ptr::addr_of!((*entry_ptr).d_name) as *const c_char);
   |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ dereferencing pointer failed: alloc59726 has size 31, so pointer to 280 bytes starting at offset 0 is out-of-bounds
   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
   = note: BACKTRACE:
   = note: inside `<std::sys::unix::fs::ReadDir as std::iter::Iterator>::next` at /checkout/library/core/src/ptr/mod.rs:2005:5
