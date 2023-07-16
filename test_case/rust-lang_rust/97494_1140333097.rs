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
Diff in /checkout/library/alloc/src/rc.rs at line 369:
         // if the weak pointer is stored inside the strong one.
             Self::from_inner(
             Self::from_inner(
-                Box::leak(Box::new(RcBox { strong: Cell::new(1), weak: Cell::new(1), value })).into(),
+                Box::leak(Box::new(RcBox { strong: Cell::new(1), weak: Cell::new(1), value }))
+                    .into(),
         }
     }
     }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/alloc/src/rc/tests.rs" "/checkout/library/core/tests/lazy.rs" "/checkout/library/core/tests/asserting.rs" "/checkout/library/alloc/src/fmt.rs" "/checkout/library/core/tests/convert.rs" "/checkout/library/alloc/src/boxed.rs" "/checkout/library/core/tests/result.rs" "/checkout/library/alloc/src/rc.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
