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
Diff in /checkout/library/std/src/path.rs at line 2489:
     #[unstable(feature = "path_try_exists", issue = "none")]
     #[inline]
     pub fn try_exists(&self) -> io::Result<bool> {
-        fs::metadata(self)
-            .map(|_| true)
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/path.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
-            .or_else(|error| if error.kind() == io::ErrorKind::NotFound { Ok(false) } else { Err(error) } )
+        fs::metadata(self).map(|_| true).or_else(|error| {
+            if error.kind() == io::ErrorKind::NotFound { Ok(false) } else { Err(error) }
     }
 
 
     /// Returns `true` if the path exists on disk and is pointing at a regular file.
Build completed unsuccessfully in 0:00:21
