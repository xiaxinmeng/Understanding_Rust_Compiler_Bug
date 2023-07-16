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
Diff in /checkout/src/librustdoc/json/mod.rs at line 203:
                 | types::ItemEnum::AssocConst { .. }
                 | types::ItemEnum::AssocType { .. } => true,
                 types::ItemEnum::Module(_)
-
                 | types::ItemEnum::ExternCrate { .. }
                 | types::ItemEnum::Import(_)
                 | types::ItemEnum::StructField(_)
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustdoc/json/mod.rs" "/checkout/src/librustdoc/json/conversions.rs" "/checkout/src/librustdoc/visit.rs" "/checkout/src/librustdoc/docfs.rs" "/checkout/src/librustdoc/visit_ast.rs" "/checkout/library/panic_unwind/src/dwarf/eh.rs" "/checkout/library/rustc-std-workspace-alloc/lib.rs" "/checkout/src/librustdoc/formats/item_type.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
