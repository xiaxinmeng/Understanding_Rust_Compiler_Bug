plain
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/library/std/src/os/freebsd/fs.rs at line 79:
         #[cfg(freebsd12)]
         panic!("as_raw_stat not supported with FreeBSD 12 ABI");
         #[cfg(not(freebsd12))]
-        unsafe { &*(self.as_inner().as_inner() as *const libc::stat as *const raw::stat) }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/os/freebsd/fs.rs"` failed.
+        unsafe {
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
+            &*(self.as_inner().as_inner() as *const libc::stat as *const raw::stat)
     }
     }
     fn st_dev(&self) -> u64 {
         self.as_inner().as_inner().st_dev as u64
Diff in /checkout/library/std/src/os/freebsd/fs.rs at line 144:
     fn st_lspare(&self) -> u32 {
         #[cfg(freebsd12)]
         panic!("st_lspare not supported with FreeBSD 12 ABI");
+        #[cfg(not(freebsd12))]
         #[cfg(not(freebsd12))]
         self.as_inner().as_inner().st_lspare as u32
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test
Build completed unsuccessfully in 0:00:36
== clock drift check ==
  local time: Fri Jul 31 20:12:27 UTC 2020
  local time: Fri Jul 31 20:12:27 UTC 2020
  network time: Fri, 31 Jul 2020 20:12:27 GMT
== end clock drift check ==
##[error]Process completed with exit code 1.
Terminate orphan process: pid (14790) (node)
Terminate orphan process: pid (14818) (python)
