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
         return None;
     }
 
-
     // Try to normalize `<X as Y>::T` to a type
     let lifted = ty.lift_to_tcx(cx.tcx).unwrap();
     match cx.tcx.try_normalize_erasing_regions(cx.param_env, lifted) {
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/clean/types.rs" "/checkout/src/librustdoc/clean/inline.rs" "/checkout/src/librustdoc/clean/simplify.rs" "/checkout/src/librustdoc/clean/cfg.rs" "/checkout/src/librustdoc/clean/blanket_impl.rs" "/checkout/src/librustdoc/clean/cfg/tests.rs" "/checkout/src/librustdoc/clean/mod.rs" "/checkout/src/librustdoc/clean/utils.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
