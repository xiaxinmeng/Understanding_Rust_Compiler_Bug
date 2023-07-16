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
Diff in /checkout/compiler/rustc_typeck/src/check/upvar.rs at line 1541:
             ty::ImmBorrow => false,
         });
-
-
         // if this is an implicit deref of an
         // upvar, then we need to modify the
         // borrow_kind of the upvar to make sure it
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_typeck/src/check/wfcheck.rs" "/checkout/compiler/rustc_typeck/src/check/demand.rs" "/checkout/compiler/rustc_typeck/src/check/upvar.rs" "/checkout/compiler/rustc_typeck/src/check/check.rs" "/checkout/compiler/rustc_typeck/src/check/compare_method.rs" "/checkout/compiler/rustc_typeck/src/check/autoderef.rs" "/checkout/compiler/rustc_typeck/src/check/inherited.rs" "/checkout/compiler/rustc_typeck/src/check/pat.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
