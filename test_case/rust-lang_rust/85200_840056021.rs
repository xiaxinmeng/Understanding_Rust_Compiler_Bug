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
Diff in /checkout/compiler/rustc_mir_build/src/build/matches/simplify.rs at line 172:
                 Ok(())
             }
 
-            PatKind::Binding { name: _, mutability: _, mode, var, ty: _, ref subpattern, is_primary: _ } => {
+            PatKind::Binding {
+                name: _,
+                mutability: _,
+                var,
+                ty: _,
+                ref subpattern,
+                is_primary: _,
+                is_primary: _,
+            } => {
                 candidate.bindings.push(Binding {
                     span: match_pair.pattern.span,
                     source: match_pair.place.clone().into_place(self.tcx, self.typeck_results),
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir_build/src/build/matches/simplify.rs" "/checkout/compiler/rustc_mir_build/src/build/block.rs" "/checkout/compiler/rustc_mir_build/src/build/misc.rs" "/checkout/compiler/rustc_mir_build/src/build/mod.rs" "/checkout/compiler/rustc_mir_build/src/build/scope.rs" "/checkout/compiler/rustc_mir_build/src/build/expr/as_temp.rs" "/checkout/compiler/rustc_mir_build/src/build/expr/as_operand.rs" "/checkout/compiler/rustc_mir_build/src/build/matches/util.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:14
