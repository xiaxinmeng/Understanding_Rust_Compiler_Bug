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
Diff in /checkout/library/std/src/sys/wasi/fs.rs at line 558:
     let (original, original_file) = open_parent(original)?;
     let (link, link_file) = open_parent(link)?;
     // Pass 0 as the flags argument, meaning don't follow symlinks.
-    original.link(
-        0,
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/sys/wasi/fs.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
-        osstr2str(original_file.as_ref())?,
-        &link,
-        osstr2str(link_file.as_ref())?,
-    )
+    original.link(0, osstr2str(original_file.as_ref())?, &link, osstr2str(link_file.as_ref())?)
 
 
 pub fn stat(p: &Path) -> io::Result<FileAttr> {
Build completed unsuccessfully in 0:00:22
