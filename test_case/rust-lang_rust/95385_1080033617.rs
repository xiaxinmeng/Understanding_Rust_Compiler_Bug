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
Diff in /checkout/library/core/src/mem/mod.rs at line 725:
     // SAFETY: because the caller must guarantee that it's inhabited and zero-sized,
     // there's nothing in the representation that needs to be set.
     // `assume_init` calls `assert_inhabited`, so we don't need to here.
-        MaybeUninit::uninit().assume_init()
-    }
+    unsafe { MaybeUninit::uninit().assume_init() }
 }
 }
 
 /// Swaps the values at two mutable locations, without deinitializing either one.
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/src/mem/manually_drop.rs" "/checkout/library/core/src/internal_macros.rs" "/checkout/library/core/src/mem/mod.rs" "/checkout/library/core/src/primitive.rs" "/checkout/library/core/src/mem/maybe_uninit.rs" "/checkout/library/core/src/ptr/metadata.rs" "/checkout/library/core/src/tuple.rs" "/checkout/library/core/src/future/join.rs"` failed.
Build completed unsuccessfully in 0:00:12
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
