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
                 _ => continue,
             };
 
-
             match module.kind {
                 ModuleKind::Block(..) => {} // We can see through blocks
                 _ => break,
Diff in /checkout/compiler/rustc_resolve/src/lib.rs at line 1916:
         }
 
-
-
         self.early_resolve_ident_in_lexical_scope(
             orig_ident,
             ScopeSet::Late(ns, module, record_used_id),
Diff in /checkout/compiler/rustc_resolve/src/lib.rs at line 2406:
                                     .is_some()
                                 suggestion = Some((
-                                    vec![(
-                                        path_span,
-                                        format!("{}", ident),
-                                        format!("{}", ident),
-                                    )],
+                                    vec![(path_span, format!("{}", ident))],
                                     String::from("try using variable instead of path"),
                                     Applicability::MaybeIncorrect,
                                 ));
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_resolve/src/lib.rs" "/checkout/compiler/rustc_resolve/src/late.rs" "/checkout/compiler/rustc_feature/src/active.rs" "/checkout/compiler/rustc_resolve/src/diagnostics.rs" "/checkout/compiler/rustc_feature/src/tests.rs" "/checkout/compiler/rustc_feature/src/removed.rs" "/checkout/compiler/rustc_resolve/src/macros.rs" "/checkout/compiler/rustc_feature/src/lib.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:20
