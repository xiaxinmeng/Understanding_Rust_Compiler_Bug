plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
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
Diff in /checkout/src/bootstrap/lib.rs at line 917:
     /// Returns the number of parallel jobs that have been configured for this
     /// build.
     fn jobs(&self) -> u32 {
-        self.config.jobs.unwrap_or_else(|| std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get) as u32)
+        self.config.jobs.unwrap_or_else(|| {
+            std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get) as u32
     }
 
 
     fn debuginfo_map_to(&self, which: GitRepo) -> Option<String> {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/alloc/tests/fmt.rs" "/checkout/library/alloc/src/boxed.rs" "/checkout/library/alloc/tests/heap.rs" "/checkout/library/alloc/tests/string.rs" "/checkout/library/alloc/tests/str.rs" "/checkout/src/bootstrap/test.rs" "/checkout/src/bootstrap/lib.rs" "/checkout/library/alloc/src/raw_vec.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
