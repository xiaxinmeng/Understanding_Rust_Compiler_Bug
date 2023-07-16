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
Diff in /checkout/compiler/rustc_resolve/src/late/lifetimes.rs at line 320:
             Scope::Elision { elide, .. } => {
                 debug!("Scope::Elision: elide {:?}", elide);
 
-                if let Elide::Exact(_) = elide {
-                    Some(LifetimeScopeForPath::Elided)
-                    None
-                }
-                }
+                if let Elide::Exact(_) = elide { Some(LifetimeScopeForPath::Elided) } else { None }
             }
             Scope::ObjectLifetimeDefault { s, .. } => {
                 debug!("Scope::ObjectLifetimeDefault: s {:?}", s);
Diff in /checkout/compiler/rustc_resolve/src/late/lifetimes.rs at line 1427:
 
 
     fn expression_label(ex: &hir::Expr<'_>) -> Option<Ident> {
-        if let hir::ExprKind::Loop(_, Some(label), ..) = ex.kind {
-            Some(label.ident)
-            None
-        }
-        }
+        if let hir::ExprKind::Loop(_, Some(label), ..) = ex.kind { Some(label.ident) } else { None }
 
 
     fn check_if_label_shadows_lifetime(tcx: TyCtxt<'_>, mut scope: ScopeRef<'_>, label: Ident) {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_resolve/src/late/lifetimes.rs" "/checkout/library/core/src/task/poll.rs" "/checkout/library/core/src/task/mod.rs" "/checkout/library/core/src/task/ready.rs" "/checkout/library/core/src/task/wake.rs" "/checkout/library/core/src/macros/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:16
