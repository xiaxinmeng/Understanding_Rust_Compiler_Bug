plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 5605ed85363345f3def5da6b1ead2ac0b803bfe7 and 93b1d1644ba28f5d9bdd05b221f7471b662fff9d
Tool subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
tests/fail/shims/sync/libc_pthread_rwlock_write_wrong_owner.rs ... ok
tests/fail/panic/double_panic.rs ... ok

tests/fail/shims/fs/isolated_file.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "--error-format=json" "--edition" "2018" "-Astable-features" "-Aunused" "-Zui-testing" "--extern" "getrandom_1=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-ac993ae34a47d010.rmeta" "--extern" "getrandom_2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libgetrandom-20b4cd671fbdd122.rmeta" "--extern" "libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/liblibc-5c91b8ee2f9a761f.rmeta" "--extern" "num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libnum_cpus-f87ab7e9e692ab6c.rmeta" "--extern" "page_size=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libpage_size-c9761f548003f841.rmeta" "--extern" "rand=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/librand-5e75cbc746a2ceda.rmeta" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps/libtokio-664c0f42e4d71cb6.rmeta" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/syn-3bb8f7ac0d0402d5" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/memchr-3aa21f28168e754b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/lock_api-2402edfbc2151f87" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/tokio-5b679bca69988a36" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/parking_lot_core-de8d28cfa3a1467f" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/libc-19a0c4dd785bdb63" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/x86_64-unknown-linux-gnu/debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/getrandom-dd8529f5bb54559b" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/quote-bbb475710904fa81" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/log-5da2e714312fda90" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/miri/debug/build/proc-macro2-eeb167ea843a342f" "tests/fail/shims/fs/isolated_file.rs"
actual output differed from expected
--- tests/fail/shims/fs/isolated_file.stderr
+++ <stderr output>
... 15 lines skipped ...
... 15 lines skipped ...
    = note: inside `std::sys::PLATFORM::fs::File::open` at RUSTLIB/std/src/sys/PLATFORM/fs.rs:LL:CC
    = note: inside `std::fs::OpenOptions::_open` at RUSTLIB/std/src/fs.rs:LL:CC
-   = note: inside `std::fs::OpenOptions::open::<&std::path::Path>` at RUSTLIB/std/src/fs.rs:LL:CC
+   = note: inside closure at RUSTLIB/std/src/fs.rs:LL:CC
+   = note: inside `<&str as std::path::PathLike>::with_path::<std::result::Result<std::fs::File, std::io::Error>, [closure@std::fs::OpenOptions::open<&str>::{closure#0}]>` at RUSTLIB/std/src/path.rs:LL:CC
    = note: inside `std::fs::File::open::<&str>` at RUSTLIB/std/src/fs.rs:LL:CC
 note: inside `main` at $DIR/isolated_file.rs:LL:CC
... 10 lines skipped ...

---
   = note: inside closure at /checkout/library/std/src/sys/unix/fs.rs:922:36
   = note: inside `std::sys::unix::cvt_r::<i32, [closure@std::sys::unix::fs::File::open_c::{closure#0}]>` at /checkout/library/std/src/sys/unix/mod.rs:269:19
   = note: inside `std::sys::unix::fs::File::open_c` at /checkout/library/std/src/sys/unix/fs.rs:922:18
   = note: inside closure at /checkout/library/std/src/sys/unix/fs.rs:910:41
   = note: inside `std::sys::common::small_c_string::run_with_cstr::<std::sys::unix::fs::File, [closure@std::sys::unix::fs::File::open::{closure#0}]>` at /checkout/library/std/src/sys/common/small_c_string.rs:43:18
   = note: inside `std::sys::common::small_c_string::run_path_with_cstr::<std::sys::unix::fs::File, [closure@std::sys::unix::fs::File::open::{closure#0}]>` at /checkout/library/std/src/sys/common/small_c_string.rs:22:5
   = note: inside `std::fs::OpenOptions::_open` at /checkout/library/std/src/fs.rs:1060:9
   = note: inside closure at /checkout/library/std/src/fs.rs:1056:31
   = note: inside closure at /checkout/library/std/src/fs.rs:1056:31
   = note: inside `<&str as std::path::PathLike>::with_path::<std::result::Result<std::fs::File, std::io::Error>, [closure@std::fs::OpenOptions::open<&str>::{closure#0}]>` at /checkout/library/std/src/path.rs:1137:9
   = note: inside `std::fs::File::open::<&str>` at /checkout/library/std/src/fs.rs:352:9
note: inside `main` at tests/fail/shims/fs/isolated_file.rs:5:17
  --> tests/fail/shims/fs/isolated_file.rs:5:17
   |
