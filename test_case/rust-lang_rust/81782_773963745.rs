plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc/src/main.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Diff in /checkout/compiler/rustc/src/main.rs at line 17:
         static _F2: unsafe extern "C" fn(*mut *mut c_void, usize, usize) -> c_int =
             libmimalloc_sys::mi_posix_memalign;
         #[used]
-        static _F3: unsafe extern "C" fn(usize, usize) -> *mut c_void = libmimalloc_sys::mi_aligned_alloc;
+        static _F3: unsafe extern "C" fn(usize, usize) -> *mut c_void =
+            libmimalloc_sys::mi_aligned_alloc;
         #[used]
         static _F4: unsafe extern "C" fn(usize) -> *mut c_void = libmimalloc_sys::mi_malloc;
         #[used]
Diff in /checkout/compiler/rustc/src/main.rs at line 24:
-        static _F5: unsafe extern "C" fn(*mut c_void, usize) -> *mut c_void = libmimalloc_sys::mi_realloc;
+        static _F5: unsafe extern "C" fn(*mut c_void, usize) -> *mut c_void =
+            libmimalloc_sys::mi_realloc;
         #[used]
         static _F6: unsafe extern "C" fn(*mut c_void) = libmimalloc_sys::mi_free;
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
Build completed unsuccessfully in 0:00:17
