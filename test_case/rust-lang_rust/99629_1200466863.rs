plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---

######################################################################## 100.0%
extracting /checkout/obj/build/cache/2022-07-21/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz to /checkout/obj/build/x86_64-unknown-linux-gnu/stage0
skip untracked path cpu-usage.csv during rustfmt invocations
Diff in /checkout/compiler/rustc_borrowck/src/diagnostics/mod.rs at line 15:
 use rustc_middle::ty::print::Print;
 use rustc_middle::ty::{self, DefIdTree, Instance, Ty, TyCtxt};
 use rustc_mir_dataflow::move_paths::{InitLocation, LookupResult};
-use rustc_span::{symbol::sym, Span, Symbol, DUMMY_SP};
 use rustc_span::def_id::LocalDefId;
+use rustc_span::{symbol::sym, Span, Symbol, DUMMY_SP};
 use rustc_target::abi::VariantIdx;
 use rustc_trait_selection::traits::type_known_to_meet_bound_modulo_regions;
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_borrowck/src/diagnostics/conflict_errors.rs" "/checkout/compiler/rustc_borrowck/src/diagnostics/explain_borrow.rs" "/checkout/compiler/rustc_borrowck/src/diagnostics/mod.rs" "/checkout/compiler/rustc_borrowck/src/diagnostics/region_errors.rs" "/checkout/compiler/rustc_borrowck/src/diagnostics/var_name.rs" "/checkout/compiler/rustc_borrowck/src/diagnostics/bound_region_errors.rs" "/checkout/compiler/rustc_borrowck/src/diagnostics/region_name.rs" "/checkout/compiler/rustc_borrowck/src/borrow_set.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any { .. }', format.rs:174:19
Build completed unsuccessfully in 0:00:15
