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
skip untracked path cpu-usage.csv during rustfmt invocations
skip untracked path src/doc/book/ during rustfmt invocations
skip untracked path src/doc/rust-by-example/ during rustfmt invocations
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/library/std/src/panic.rs at line 135:
 #[stable(feature = "catch_unwind", since = "1.9.0")]
 pub fn catch_unwind<F: FnOnce() -> R + UnwindSafe, R>(f: F) -> Result<R> {
     unsafe { panicking::r#try(f) }
-    // So, there is a footgun with `catch_unwind()`'s API:
-    //
-    // 1. when running "untrusted" / arbitrary code, it may panic and unwind.
-    // 2. this, in turn, could lead to an impromptu interruption of your own code.
-    // 3. thence `panic::catch_unwind()`, to act as an unwinding boundary and
-    //    ensure the unwinding can be stopped.
-    // 4. but the so obtained `Result::<R, Box<dyn Any…>>::Err` variant contains,
-    //    in that case, an arbitrary payload (_e.g._, a boxed `42` in case of
-    //    `panic!(42)`), with arbitrary drop glue, such as a `PanicOnDrop` bomb.
-    // 5. this means that if the `Err` variant is just dismissed or discarded, when
-    //    it gets dropped, it can still cause an unwind which goes against the
-    //    caller's intent to block them.
-    //
-    // See #86027 for more context.
-    //
-    // In order to tackle this, the idea behind this `.map_err()`, and the
-    // `DropNoUnwindSameAnyTypeId` type from `::core`, is to do something similar
-    // to what was suggested over
-    // https://github.com/rust-lang/rust/issues/86027#issuecomment-858083087:
-    // To cheat a little bit and tweak/amend the obtained `Box<dyn Any…>`:
-    //   - keep the `.type_id()` the same to avoid breakage with downcasting;
-    //   - but make it so the virtual `.drop_in_place()` is somehow overridden with
-    //     a shim around the real drop glue that prevents unwinding (_e.g._, by
-    //     aborting when that happens).
-    //
-    // This is achieved through the `DropNoUnwindSameAnyTypeId<T>`, wrapper:
-    //   - with the very same layout as the `T` it wraps;
-    //   - with an overridden/fake `.type_id()` so as to impersonate its inner `T`;
-    //   - with a manual `impl Drop` which uses an abort bomb to ensure no
-    //     unwinding can happen.
-    //
-    // That way, the `Box<DropNoUnwindSameAnyTypeId<T>>`, when box-erased to a
-    // `Box<dyn Any…>`, becomes, both layout-wise, and `type_id`-wise,
-    // undistinguishable from a `Box<T>`, thence avoiding any breakage.
-    //
-    // And yet, when that `Box<dyn Any…>` is implicitly dropped with
-    // `catch_unwind`s, no further unwinding can happen.
+        // So, there is a footgun with `catch_unwind()`'s API:
+        //
+        // 1. when running "untrusted" / arbitrary code, it may panic and unwind.
+        // 2. this, in turn, could lead to an impromptu interruption of your own code.
+        // 3. thence `panic::catch_unwind()`, to act as an unwinding boundary and
+        //    ensure the unwinding can be stopped.
+        // 4. but the so obtained `Result::<R, Box<dyn Any…>>::Err` variant contains,
+        //    in that case, an arbitrary payload (_e.g._, a boxed `42` in case of
+        //    `panic!(42)`), with arbitrary drop glue, such as a `PanicOnDrop` bomb.
+        // 5. this means that if the `Err` variant is just dismissed or discarded, when
+        //    it gets dropped, it can still cause an unwind which goes against the
+        //    caller's intent to block them.
+        //
+        // See #86027 for more context.
+        //
+        // In order to tackle this, the idea behind this `.map_err()`, and the
+        // `DropNoUnwindSameAnyTypeId` type from `::core`, is to do something similar
+        // to what was suggested over
+        // https://github.com/rust-lang/rust/issues/86027#issuecomment-858083087:
+        // To cheat a little bit and tweak/amend the obtained `Box<dyn Any…>`:
+        //   - keep the `.type_id()` the same to avoid breakage with downcasting;
+        //   - but make it so the virtual `.drop_in_place()` is somehow overridden with
+        //     a shim around the real drop glue that prevents unwinding (_e.g._, by
+        //     aborting when that happens).
+        //
+        // This is achieved through the `DropNoUnwindSameAnyTypeId<T>`, wrapper:
+        //   - with the very same layout as the `T` it wraps;
+        //   - with an overridden/fake `.type_id()` so as to impersonate its inner `T`;
+        //   - with a manual `impl Drop` which uses an abort bomb to ensure no
+        //     unwinding can happen.
+        //
+        // That way, the `Box<DropNoUnwindSameAnyTypeId<T>>`, when box-erased to a
+        // `Box<dyn Any…>`, becomes, both layout-wise, and `type_id`-wise,
+        // undistinguishable from a `Box<T>`, thence avoiding any breakage.
+        //
+        // And yet, when that `Box<dyn Any…>` is implicitly dropped with
+        // `catch_unwind`s, no further unwinding can happen.
         .map_err(|any| {
             // *Project* the `Box<M>` to a `Box<DropNoUnwindSameAnyTypeId<M>>`.
             // Safety: if `M : Send`, then `DropNoUnwindSameAnyTypeId<M> : Send`.
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/sys/windows/locks/mod.rs" "/checkout/library/std/src/ascii.rs" "/checkout/library/std/src/lib.rs" "/checkout/library/std/src/keyword_docs.rs" "/checkout/library/std/src/panic.rs" "/checkout/library/std/src/backtrace/tests.rs" "/checkout/library/std/src/sync/poison.rs" "/checkout/library/std/src/f64.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
thread '<unnamed>' panicked at 'tx.send(entry.into_path()) failed with sending on a closed channel', format.rs:166:17
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any { .. }', /cargo/registry/src/github.com-1ecc6299db9ec823/ignore-0.4.18/src/walk.rs:1302:31
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any { .. }', /cargo/registry/src/github.com-1ecc6299db9ec823/ignore-0.4.18/src/walk.rs:1302:31
thread '<unnamed>' panicked at 'tx.send(entry.into_path()) failed with sending on a closed channel', format.rs:166:17
