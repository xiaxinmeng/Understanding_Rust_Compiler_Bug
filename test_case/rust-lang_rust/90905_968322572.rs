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
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/src/librustdoc/clean/utils.rs at line 412:
         }
         // Each of these have their own page.
         Res::Def(
-            kind @ (Fn | TyAlias | Enum | Trait | Struct | Union | Mod | ForeignTy | Const | Static
+            @
+            @
+            (Fn | TyAlias | Enum | Trait | Struct | Union | Mod | ForeignTy | Const | Static
             | Macro(..) | TraitAlias),
             i,
         ) => (i, kind.into()),
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/clean/utils/tests.rs" "/checkout/src/librustdoc/clean/simplify.rs" "/checkout/src/librustdoc/clean/mod.rs" "/checkout/src/librustdoc/clean/utils.rs" "/checkout/src/librustdoc/clean/inline.rs" "/checkout/src/librustdoc/clean/types.rs" "/checkout/src/librustdoc/clean/cfg.rs" "/checkout/src/librustdoc/clean/auto_trait.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
