plain
[command]/usr/bin/git submodule foreach --recursive git config --local --name-only --get-regexp 'http\.https\:\/\/github\.com\/\.extraheader' && git config --local --unset-all 'http.https://github.com/.extraheader' || :
[command]/usr/bin/git config --local http.https://github.com/.extraheader AUTHORIZATION: basic ***
##[endgroup]
##[group]Fetching the repository
[command]/usr/bin/git -c protocol.version=2 fetch --no-tags --prune --progress --no-recurse-submodules --depth=2 origin +984fad9aedf016228ac4adceb16d0934076a5217:refs/remotes/pull/80099/merge
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
Diff in /checkout/src/librustdoc/html/format.rs at line 1089:
 
 
 impl clean::Visibility {
-    crate fn print_with_space<'tcx>(
-        self,
-        tcx: TyCtxt<'tcx>,
-    ) -> impl fmt::Display + 'tcx {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/html/format.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
+    crate fn print_with_space<'tcx>(self, tcx: TyCtxt<'tcx>) -> impl fmt::Display + 'tcx {
         use rustc_span::symbol::kw;
 
         display_fn(move |f| match self {
Build completed unsuccessfully in 0:00:21
== clock drift check ==
  local time: Thu Dec 17 00:12:31 UTC 2020
  network time: Wed, 16 Dec 2020 10:06:49 GMT
