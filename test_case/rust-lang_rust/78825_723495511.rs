plain
[command]/usr/bin/git submodule foreach --recursive git config --local --name-only --get-regexp 'http\.https\:\/\/github\.com\/\.extraheader' && git config --local --unset-all 'http.https://github.com/.extraheader' || :
[command]/usr/bin/git config --local http.https://github.com/.extraheader AUTHORIZATION: basic ***
##[endgroup]
##[group]Fetching the repository
[command]/usr/bin/git -c protocol.version=2 fetch --no-tags --prune --progress --no-recurse-submodules --depth=2 origin +a1480fe8dc3999b78d2d46fc4b19d21261056ff3:refs/remotes/pull/78825/merge
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
Diff in /checkout/compiler/rustc_codegen_llvm/src/intrinsic.rs at line 1724:
 // stuffs.
 fn int_type_width_signed(ty: Ty<'_>, cx: &CodegenCx<'_, '_>) -> Option<(u64, bool)> {
     match ty.kind() {
-        ty::Int(t) => Some((
-            t.bit_width().unwrap_or(u64::from(cx.tcx.sess.target.pointer_width)),
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_codegen_llvm/src/intrinsic.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
-        )),
-        )),
-        ty::Uint(t) => Some((
-            t.bit_width().unwrap_or(u64::from(cx.tcx.sess.target.pointer_width)),
-        )),
-        )),
+        ty::Int(t) => {
+            Some((t.bit_width().unwrap_or(u64::from(cx.tcx.sess.target.pointer_width)), true))
+        }
+        ty::Uint(t) => {
+            Some((t.bit_width().unwrap_or(u64::from(cx.tcx.sess.target.pointer_width)), false))
         _ => None,
     }
 }
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/tidy
Build completed unsuccessfully in 0:00:14
== clock drift check ==
  local time: Sat Nov  7 21:01:25 UTC 2020
  network time: Sat, 07 Nov 2020 00:15:56 GMT
== end clock drift check ==
##[error]Process completed with exit code 1.
[command]/usr/bin/git version
git version 2.29.2
[command]/usr/bin/git config --local --name-only --get-regexp core\.sshCommand
[command]/usr/bin/git submodule foreach --recursive git config --local --name-only --get-regexp 'core\.sshCommand' && git config --local --unset-all 'core.sshCommand' || :
