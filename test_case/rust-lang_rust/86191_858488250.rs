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
Diff in /checkout/library/std/src/sys/solid/memchr.rs at line 6:
             haystack.len(),
     };
     };
-    if p.is_null() {
-    } else {
-    } else {
-        Some(p as usize - (haystack.as_ptr() as usize))
-    }
+    if p.is_null() { None } else { Some(p as usize - (haystack.as_ptr() as usize)) }
 
 
 pub fn memrchr(needle: u8, haystack: &[u8]) -> Option<usize> {
Diff in /checkout/library/std/src/sys/solid/memchr.rs at line 21:
             haystack.len(),
     };
     };
-    if p.is_null() {
-    } else {
-    } else {
-        Some(p as usize - (haystack.as_ptr() as usize))
-    }
+    if p.is_null() { None } else { Some(p as usize - (haystack.as_ptr() as usize)) }
 
 
Diff in /checkout/library/std/src/sys/solid/os.rs at line 20:
 
 
 pub fn error_string(errno: i32) -> String {
-    if let Some(name) = error::error_name(errno) {
-        name.to_owned()
-        format!("{}", errno)
-    }
-    }
+    if let Some(name) = error::error_name(errno) { name.to_owned() } else { format!("{}", errno) }
 
 
 pub fn getcwd() -> io::Result<PathBuf> {
Diff in /checkout/library/std/src/sys/itron/error.rs at line 13:
     /// error code does not represent a failure or warning.
     #[inline]
     pub fn new(er: abi::ER) -> Option<Self> {
-        if er < 0 {
-            Some(Self { er })
-            None
-        }
-        }
+        if er < 0 { Some(Self { er }) } else { None }
 
 
     /// Returns `Ok(er)` if `er` represents a success or `Err(_)` otherwise.
Diff in /checkout/library/std/src/sys/itron/error.rs at line 24:
     #[inline]
     pub fn err_if_negative(er: abi::ER) -> Result<abi::ER, Self> {
-        if let Some(error) = Self::new(er) {
-            Err(error)
-            Ok(er)
-        }
-        }
+        if let Some(error) = Self::new(er) { Err(error) } else { Ok(er) }
 
 
     /// Get the raw error code.
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/sys/sgx/condvar.rs" "/checkout/library/std/src/sys/solid/memchr.rs" "/checkout/library/std/src/sys/sgx/os.rs" "/checkout/library/std/src/sys/solid/io.rs" "/checkout/library/std/src/sys/sgx/alloc.rs" "/checkout/library/std/src/sys/solid/thread_local_key.rs" "/checkout/library/std/src/sys/sgx/rwlock.rs" "/checkout/library/std/src/sys/solid/env.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:17
