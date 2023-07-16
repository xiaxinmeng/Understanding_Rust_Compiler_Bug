plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between db0597f5619d5ed93feca28e61226d3581cc7867 and f7ff5c450ac435f3ab25fd8ed2f759f8eefd31df
src/ci/scripts/should-skip-this.sh: line 23: library/std/src/sys: Is a directory
Tool subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
tests/fail/shims/sync/libc_pthread_rwlock_write_wrong_owner.rs ... ok
tests/fail/panic/double_panic.rs ... ok

tests/fail/unaligned_pointers/reference_to_packed.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--error-format=json" "--edition" "2018" "-Astable-features" "-Aunused" "-Zui-testing" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-ac993ae34a47d010.rmeta" "--extern" "getrandom_2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-20b4cd671fbdd122.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-5c91b8ee2f9a761f.rmeta" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-f87ab7e9e692ab6c.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-c9761f548003f841.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-5e75cbc746a2ceda.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-664c0f42e4d71cb6.rmeta" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-19a0c4dd785bdb63" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/tokio-5b679bca69988a36" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-eeb167ea843a342f" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-de8d28cfa3a1467f" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-2402edfbc2151f87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-3bb8f7ac0d0402d5" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-bbb475710904fa81" "tests/fail/unaligned_pointers/reference_to_packed.rs" "-Zmiri-disable-validation" "-Zmiri-disable-stacked-borrows"
actual output differed from expected
--- tests/fail/unaligned_pointers/reference_to_packed.stderr
+++ <stderr output>
-error: Undefined Behavior: accessing memory with alignment ALIGN, but alignment ALIGN is required
-error: Undefined Behavior: accessing memory with alignment ALIGN, but alignment ALIGN is required
+warning: lint `unaligned_references` has been removed: converted into hard error, see issue #82523 <https://github.com/rust-lang/rust/issues/82523> for more information
    |
-LL |         let i = *p;
-   |                 ^^ accessing memory with alignment ALIGN, but alignment ALIGN is required
-   |                 ^^ accessing memory with alignment ALIGN, but alignment ALIGN is required
+LL | #![allow(dead_code, unused_variables, unaligned_references)]
    |
-   = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
-   = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
-   = note: BACKTRACE:
-   = note: BACKTRACE:
-   = note: inside `main` at $DIR/reference_to_packed.rs:LL:CC
+   = note: `#[warn(renamed_and_removed_lints)]` on by default
 
-note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace
+error[E0791]: reference to packed field is unaligned
+  --> $DIR/reference_to_packed.rs:LL:CC
+   |
+LL |         let p = &foo.x;
+   |
+   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
+   = help: copy the field contents to a local variable, or replace the reference with a raw pointer and use `read_unaligned`/`write_unaligned` (loads and stores via `*p` must be properly aligned even when using raw pointers)
 
 
~error: aborting due to previous error; 1 warning emitted
 
+For more information about this error, try `rustc --explain E0791`.
 


substring `alignment 4 is required` not found in stderr output

There were 1 unmatched diagnostics at tests/fail/unaligned_pointers/reference_to_packed.rs:16
    Error: reference to packed field is unaligned


full stderr:
warning: lint `unaligned_references` has been removed: converted into hard error, see issue #82523 <https://github.com/rust-lang/rust/issues/82523> for more information
   |
   |
LL | #![allow(dead_code, unused_variables, unaligned_references)]
   |
   = note: `#[warn(renamed_and_removed_lints)]` on by default

error[E0791]: reference to packed field is unaligned
error[E0791]: reference to packed field is unaligned
  --> tests/fail/unaligned_pointers/reference_to_packed.rs:16:17
   |
LL |         let p = &foo.x;
   |
   = note: fields of packed structs are not properly aligned, and creating a misaligned reference is undefined behavior (even if that reference is never dereferenced)
   = help: copy the field contents to a local variable, or replace the reference with a raw pointer and use `read_unaligned`/`write_unaligned` (loads and stores via `*p` must be properly aligned even when using raw pointers)

