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
Diff in /checkout/compiler/rustc_mir_transform/src/normalize_array_len.rs at line 165:
                     if add_deref {
                         place = self.tcx.mk_place_deref(place);
                     }
-                    len_statement.kind = StatementKind::Assign(Box::new((*into, Rvalue::Len(place))));
+                    len_statement.kind =
+                        StatementKind::Assign(Box::new((*into, Rvalue::Len(place))));
                     statements.push(len_statement);
                     // make temporary dead
                     // make temporary dead
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_const_eval/src/interpret/intrinsics/caller_location.rs" "/checkout/compiler/rustc_span/src/lev_distance.rs" "/checkout/compiler/rustc_trait_selection/src/infer.rs" "/checkout/compiler/rustc_mir_transform/src/generator.rs" "/checkout/compiler/rustc_trait_selection/src/lib.rs" "/checkout/compiler/rustc_mir_transform/src/normalize_array_len.rs" "/checkout/compiler/rustc_trait_selection/src/autoderef.rs" "/checkout/compiler/rustc_const_eval/src/interpret/intrinsics/type_name.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
