plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/library/std/src/sys/sgx/abi/usercalls/tests.rs at line 1:
-use super::alloc::{copy_from_userspace, copy_to_userspace};
 use super::alloc::User;
+use super::alloc::{copy_from_userspace, copy_to_userspace};
 #[test]
 fn test_copy_to_userspace_function() {
 fn test_copy_to_userspace_function() {
Diff in /checkout/library/std/src/sys/sgx/abi/usercalls/alloc.rs at line 314:
 //   | small1 | Chunk smaller than 8 bytes
 //   +--------+
 fn region_as_aligned_chunks(ptr: *const u8, len: usize) -> (u8, usize, u8) {
-    let small0_size = if ptr as usize % 8 == 0 {
-    } else {
-    } else {
-        (8 - ptr as usize % 8) as u8
-    };
+    let small0_size = if ptr as usize % 8 == 0 { 0 } else { (8 - ptr as usize % 8) as u8 };
     let small1_size = ((len - small0_size as usize) % 8) as u8;
     let big_size = len - small0_size as usize - small1_size as usize;
 
Diff in /checkout/library/std/src/sys/sgx/abi/usercalls/alloc.rs at line 475:
             copy_quadwords(aligned_src, tmp_buff.as_mut_ptr(), aligned_len);
             // Copy the correct parts of the temporary buffer to the destination
-            ptr::copy(
-            ptr::copy(
-                (tmp_buff.as_ptr() as usize + pad0_size) as _,
-                len as _
-            );
-            );
+            ptr::copy((tmp_buff.as_ptr() as usize + pad0_size) as _, dst, len as _);
     }
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/sys/sgx/abi/mod.rs" "/checkout/library/std/src/sys/unix/thread.rs" "/checkout/library/std/src/sys/sgx/abi/tls/mod.rs" "/checkout/library/std/src/sys/sgx/abi/tls/sync_bitset.rs" "/checkout/library/std/src/sys/sgx/abi/tls/sync_bitset/tests.rs" "/checkout/library/std/src/sys/sgx/abi/usercalls/alloc.rs" "/checkout/library/std/src/sys/sgx/abi/reloc.rs" "/checkout/library/std/src/sys/sgx/abi/mem.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
thread '<unnamed>' panicked at 'tx.send(entry.into_path()) failed with sending on a closed channel', format.rs:166:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread '<unnamed>' panicked at 'tx.send(entry.into_path()) failed with sending on a closed channel', format.rs:166:17
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any { .. }', /cargo/registry/src/github.com-1ecc6299db9ec823/ignore-0.4.18/src/walk.rs:1302:31
