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
Diff in /checkout/src/bootstrap/test.rs at line 1877:
         let builder = run.builder;
         let host = run.build_triple();
         let compiler = builder.compiler_for(builder.top_stage, host, host);
-        let crates = run.paths.iter().map(|p| {
-            builder.crate_paths[&p.assert_single_path().path].clone()
-         }).collect();
+        let crates = run
+            .paths
+            .iter()
+            .map(|p| builder.crate_paths[&p.assert_single_path().path].clone())
+            .collect();
         let test_kind = builder.kind.into();
 
         builder.ensure(CrateLibrustc { compiler, target: run.target, test_kind, crates });
Diff in /checkout/src/bootstrap/test.rs at line 1918:
         let host = run.build_triple();
         let compiler = builder.compiler_for(builder.top_stage, host, host);
         let test_kind = builder.kind.into();
-        let crates = run.paths.iter().map(|p| builder.crate_paths[&p.assert_single_path().path].clone()).collect();
+        let crates = run
+            .paths
+            .iter()
+            .map(|p| builder.crate_paths[&p.assert_single_path().path].clone())
+            .collect();
 
         builder.ensure(Crate { compiler, target: run.target, mode: Mode::Std, test_kind, crates });
Diff in /checkout/src/bootstrap/builder.rs at line 233:
Diff in /checkout/src/bootstrap/builder.rs at line 233:
             PathSet::Set(set) => {
                 assert_eq!(set.len(), 1, "called assert_single_path on multiple paths");
                 set.iter().next().unwrap()
+            }
+            }
             PathSet::Suite(_) => unreachable!("called assert_single_path on a Suite path"),
+        }
     }
 }
 
 
Diff in /checkout/src/bootstrap/lib.rs at line 170:
     pub unsafe fn setup(_build: &mut crate::Build) {}
 
 
-use crate::cache::{Interned, INTERNER};
 pub use crate::builder::PathSet;
+use crate::cache::{Interned, INTERNER};
 pub use crate::config::Config;
 pub use crate::flags::Subcommand;
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/bin/rustc.rs" "/checkout/src/bootstrap/bin/rustdoc.rs" "/checkout/src/bootstrap/test.rs" "/checkout/src/bootstrap/bin/sccache-plus-cl.rs" "/checkout/src/bootstrap/build.rs" "/checkout/src/bootstrap/bin/main.rs" "/checkout/src/bootstrap/run.rs" "/checkout/src/bootstrap/bin/llvm-config-wrapper.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
