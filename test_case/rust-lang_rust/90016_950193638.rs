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
Diff in /checkout/compiler/rustc_mir_transform/src/lib.rs at line 37:
 mod add_call_guards;
 mod add_moves_for_packed_drops;
 mod add_retag;
-mod refinements;
 mod check_const_item_mutation;
 mod check_packed_ref;
 pub mod check_unsafety;
Diff in /checkout/compiler/rustc_mir_transform/src/lib.rs at line 62:
 mod multiple_return_terminators;
 mod normalize_array_len;
 mod nrvo;
+mod refinements;
 mod remove_noop_landing_pads;
 mod remove_storage_markers;
 mod remove_unneeded_drops;
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir_transform/src/abort_unwinding_calls.rs" "/checkout/compiler/rustc_query_system/src/ich/impls_hir.rs" "/checkout/compiler/rustc_mir_transform/src/lib.rs" "/checkout/compiler/rustc_query_system/src/ich/hcx.rs" "/checkout/compiler/rustc_mir_transform/src/dest_prop.rs" "/checkout/compiler/rustc_query_system/src/ich/impls_syntax.rs" "/checkout/compiler/rustc_mir_transform/src/early_otherwise_branch.rs" "/checkout/compiler/rustc_query_system/src/ich/mod.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
