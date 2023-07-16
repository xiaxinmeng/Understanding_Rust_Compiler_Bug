plain
[command]/usr/bin/git submodule foreach --recursive git config --local --name-only --get-regexp 'http\.https\:\/\/github\.com\/\.extraheader' && git config --local --unset-all 'http.https://github.com/.extraheader' || :
[command]/usr/bin/git config --local http.https://github.com/.extraheader AUTHORIZATION: basic ***
##[endgroup]
##[group]Fetching the repository
[command]/usr/bin/git -c protocol.version=2 fetch --no-tags --prune --progress --no-recurse-submodules --depth=2 origin +c6d985906f40216fddc21a88ac0fb6a2259281db:refs/remotes/pull/79043/merge
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
Diff in /checkout/src/bootstrap/compile.rs at line 522:
     }
 }
 
-pub fn rustc_cargo(builder: &Builder<'_>, cargo: &mut Cargo, target: TargetSelection, compiler: Compiler) {
+pub fn rustc_cargo(
+    builder: &Builder<'_>,
+    cargo: &mut Cargo,
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/compile.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
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
Build completed unsuccessfully in 0:00:14
== clock drift check ==
  local time: Sat Nov 14 15:53:43 UTC 2020
