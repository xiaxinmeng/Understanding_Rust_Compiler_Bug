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
Diff in /checkout/compiler/rustc_middle/src/mir/tcx.rs at line 215:
     /// whether its only shallowly initialized (`Rvalue::Box`).
     pub fn initialization_state(&self) -> RvalueInitializationState {
         match *self {
-            Rvalue::NullaryOp(NullOp::Box, _) |
-            Rvalue::ShallowInitBox(_, _) => RvalueInitializationState::Shallow,
+            Rvalue::NullaryOp(NullOp::Box, _) | Rvalue::ShallowInitBox(_, _) => {
+                RvalueInitializationState::Shallow
+            }
             _ => RvalueInitializationState::Deep,
     }
     }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_middle/src/mir/graph_cyclic_cache.rs" "/checkout/compiler/rustc_middle/src/mir/tcx.rs" "/checkout/compiler/rustc_middle/src/thir/abstract_const.rs" "/checkout/compiler/rustc_middle/src/mir/terminator.rs" "/checkout/compiler/rustc_middle/src/mir/generic_graph.rs" "/checkout/compiler/rustc_middle/src/mir/type_foldable.rs" "/checkout/compiler/rustc_middle/src/lib.rs" "/checkout/compiler/rustc_middle/src/thir/visit.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
