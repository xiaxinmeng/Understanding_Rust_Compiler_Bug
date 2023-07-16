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
Diff in /checkout/src/bootstrap/builder.rs at line 718:
             Subcommand::Dist { ref paths } => (Kind::Dist, &paths[..]),
             Subcommand::Install { ref paths } => (Kind::Install, &paths[..]),
             Subcommand::Run { ref paths } => (Kind::Run, &paths[..]),
-            Subcommand::Setup { ref path } => (Kind::Setup, if let Some(p) = path { std::slice::from_ref(p) } else { &[] }),
+            Subcommand::Setup { ref path } => {
+                (Kind::Setup, if let Some(p) = path { std::slice::from_ref(p) } else { &[] })
+            }
             Subcommand::Format { .. } | Subcommand::Clean { .. } => {
             }
             }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/src/bootstrap/cache.rs" "/checkout/library/alloc/src/task.rs" "/checkout/src/bootstrap/native.rs" "/checkout/library/alloc/src/sync.rs" "/checkout/library/alloc/src/macros.rs" "/checkout/src/bootstrap/builder.rs" "/checkout/library/alloc/src/tests.rs" "/checkout/library/alloc/src/lib.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
