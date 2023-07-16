plain
[command]/usr/bin/git submodule foreach --recursive git config --local --name-only --get-regexp 'http\.https\:\/\/github\.com\/\.extraheader' && git config --local --unset-all 'http.https://github.com/.extraheader' || :
[command]/usr/bin/git config --local http.https://github.com/.extraheader AUTHORIZATION: basic ***
##[endgroup]
##[group]Fetching the repository
[command]/usr/bin/git -c protocol.version=2 fetch --no-tags --prune --progress --no-recurse-submodules --depth=2 origin +15b803db2a327add510127d2c3200aed0fc9cf01:refs/remotes/pull/79044/merge
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
Diff in /checkout/src/bootstrap/compile.rs at line 522:
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/compile.rs"` failed.
 }
 
 
-pub fn rustc_cargo(builder: &Builder<'_>, cargo: &mut Cargo, target: TargetSelection, compiler: Compiler) {
+pub fn rustc_cargo(
+    builder: &Builder<'_>,
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
+    cargo: &mut Cargo,
+    compiler: Compiler,
+) {
     cargo
     cargo
         .arg("--features")
         .arg(builder.rustc_features())
Diff in /checkout/src/bootstrap/compile.rs at line 531:
     rustc_cargo_env(builder, cargo, target, compiler);
 
 
-pub fn rustc_cargo_env(builder: &Builder<'_>, cargo: &mut Cargo, target: TargetSelection, compiler: Compiler) {
+pub fn rustc_cargo_env(
+    builder: &Builder<'_>,
+    cargo: &mut Cargo,
+    compiler: Compiler,
+) {
+) {
     // Set some configuration variables picked up by build scripts and
     // the compiler alike
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
Build completed unsuccessfully in 0:00:20
== clock drift check ==
  local time: Sat Nov 14 15:44:20 UTC 2020
