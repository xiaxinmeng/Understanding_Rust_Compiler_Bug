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
extracting /checkout/obj/build/cache/2022-07-21/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz to /checkout/obj/build/x86_64-unknown-linux-gnu/stage0
skip untracked path cpu-usage.csv during rustfmt invocations
Diff in /checkout/compiler/rustc_ast/src/mut_visit.rs at line 1573:
 
 impl DummyAstNode for Stmt {
     fn dummy() -> Self {
-        Stmt { id: DUMMY_NODE_ID, kind: StmtKind::Empty, span: Default::default(),
-            b: B::b(),
-        }
+        Stmt { id: DUMMY_NODE_ID, kind: StmtKind::Empty, span: Default::default(), b: B::b() }
 }
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/compiler/rustc_ast/src/util/unicode.rs" "/checkout/compiler/rustc_ast/src/util/comments.rs" "/checkout/compiler/rustc_ast/src/util/literal.rs" "/checkout/compiler/rustc_ast/src/util/parser.rs" "/checkout/compiler/rustc_ast/src/util/classify.rs" "/checkout/compiler/rustc_ast/src/mut_visit.rs" "/checkout/compiler/rustc_ast/src/attr/mod.rs" "/checkout/compiler/rustc_ast/src/util/comments/tests.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
thread 'thread '<unnamed>' panicked at 'tx.send(entry.into_path()) failed with sending on a closed channel', format.rs:166:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
<unnamed>' panicked at 'tx.send(entry.into_path()) failed with sending on a closed channel', format.rs:166:17
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any { .. }', /cargo/registry/src/github.com-1ecc6299db9ec823/ignore-0.4.18/src/walk.rs:1302:31
