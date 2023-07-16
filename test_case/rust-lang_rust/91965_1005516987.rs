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
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/src/bootstrap/builder.rs at line 1719:
         }
 
         // Only execute if it's supposed to run as default
-        if desc.default && should_run.is_really_default() {
-            self.ensure(step)
-            None
-        }
-        }
+        if desc.default && should_run.is_really_default() { self.ensure(step) } else { None }
 
 
     /// Checks if any of the "should_run" paths is in the `Builder` paths.
Diff in /checkout/src/bootstrap/dist.rs at line 16:
 
 use build_helper::{output, t};
 
-use crate::builder::{Builder, RunConfig, ShouldRun, Step, Kind};
+use crate::builder::{Builder, Kind, RunConfig, ShouldRun, Step};
 use crate::cache::{Interned, INTERNER};
 use crate::compile;
Diff in /checkout/src/bootstrap/doc.rs at line 15:
 use crate::Mode;
 use crate::Mode;
 use build_helper::{t, up_to_date};
 
-use crate::builder::{Builder, Compiler, RunConfig, ShouldRun, Step, Kind};
+use crate::builder::{Builder, Compiler, Kind, RunConfig, ShouldRun, Step};
 use crate::cache::{Interned, INTERNER};
 use crate::compile;
 use crate::config::{Config, TargetSelection};
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_feature/src/tests.rs" "/checkout/src/bootstrap/compile.rs" "/checkout/src/bootstrap/install.rs" "/checkout/src/etc/test-float-parse/src/lib.rs" "/checkout/src/bootstrap/dist.rs" "/checkout/src/etc/test-float-parse/src/bin/rand-f64.rs" "/checkout/src/etc/test-float-parse/src/bin/u64-pow2.rs" "/checkout/compiler/rustc_ast_passes/src/ast_validation.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
