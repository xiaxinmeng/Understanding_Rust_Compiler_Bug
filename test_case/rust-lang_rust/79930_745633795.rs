plain
[command]/usr/bin/git submodule foreach --recursive git config --local --name-only --get-regexp 'http\.https\:\/\/github\.com\/\.extraheader' && git config --local --unset-all 'http.https://github.com/.extraheader' || :
[command]/usr/bin/git config --local http.https://github.com/.extraheader AUTHORIZATION: basic ***
##[endgroup]
##[group]Fetching the repository
[command]/usr/bin/git -c protocol.version=2 fetch --no-tags --prune --progress --no-recurse-submodules --depth=2 origin +152fb8c4586ccce5c35b1719e700971e851efa01:refs/remotes/pull/79930/merge
---
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
Diff in /checkout/library/std/src/io/buffered/bufwriter.rs at line 436:
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/io/buffered/bufwriter.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
             // `usize`. If the computation overflows, then surely the input cannot fit in our
             // buffer, so we forward to the inner writer's `write_vectored` method to let it
             // handle it appropriately.
-            let saturated_total_len = bufs.iter().fold(0usize, |acc, b|
-                acc.saturating_add(b.len())
-            );
+            let saturated_total_len =
+                bufs.iter().fold(0usize, |acc, b| acc.saturating_add(b.len()));
 
             if saturated_total_len > self.buf.capacity() - self.buf.len() {
                 // Flush if the total length of the input exceeds our buffer's spare capacity.
Build completed unsuccessfully in 0:00:19
== clock drift check ==
  local time: Tue Dec 15 23:34:54 UTC 2020
  network time: Tue, 15 Dec 2020 16:35:21 GMT
