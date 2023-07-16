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
Diff in /checkout/compiler/rustc_mir_transform/src/generator.rs at line 289:
         let self_place = Place::from(SELF_ARG);
         let assign = Statement {
             source_info: SourceInfo::outermost(body.span),
-            kind: StatementKind::Assign(Box::new((temp, Rvalue::Discriminant { place: self_place, relative: false }))),
+            kind: StatementKind::Assign(Box::new((
+                temp,
+                Rvalue::Discriminant { place: self_place, relative: false },
+            ))),
         (assign, temp)
     }
     }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_mir_transform/src/uninhabited_enum_branching.rs" "/checkout/compiler/rustc_mir_transform/src/dest_prop.rs" "/checkout/compiler/rustc_mir_transform/src/coverage/counters.rs" "/checkout/compiler/rustc_mir_transform/src/add_niche_cases.rs" "/checkout/compiler/rustc_mir_transform/src/coverage/tests.rs" "/checkout/compiler/rustc_mir_transform/src/generator.rs" "/checkout/compiler/rustc_mir_transform/src/coverage/spans.rs" "/checkout/compiler/rustc_mir_transform/src/reveal_all.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
