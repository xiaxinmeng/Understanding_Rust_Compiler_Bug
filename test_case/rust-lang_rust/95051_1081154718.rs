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
Diff in /checkout/library/alloc/src/vec/mod.rs at line 2571:
         &mut self,
         iter: I,
     ) -> Result<(), TryReserveError> {
-        <Self as SpecExtend<T, I::IntoIter>>::spec_extend::<TryReserveError>(
-            iter.into_iter(),
-        )
-        )
+        <Self as SpecExtend<T, I::IntoIter>>::spec_extend::<TryReserveError>(self, iter.into_iter())
 }
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/alloc/tests/string.rs" "/checkout/library/alloc/tests/str.rs" "/checkout/library/alloc/src/vec/drain.rs" "/checkout/library/alloc/src/vec/mod.rs" "/checkout/library/alloc/src/vec/in_place_collect.rs" "/checkout/library/profiler_builtins/build.rs" "/checkout/library/alloc/src/vec/is_zero.rs" "/checkout/library/alloc/src/boxed.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
