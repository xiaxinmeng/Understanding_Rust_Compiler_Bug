plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
Diff in /checkout/library/core/tests/any.rs at line 125:
         type_name::<T>()
     }
 
-    assert_ne!(
-        type_name_of_val(Velocity),
-        type_name_of_val(Velocity(0.0, -9.8)),
-    );
+    assert_ne!(type_name_of_val(Velocity), type_name_of_val(Velocity(0.0, -9.8)),);
 
 
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/core/tests/cmp.rs" "/checkout/library/core/src/slice/memchr.rs" "/checkout/library/core/tests/any.rs" "/checkout/library/core/src/slice/sort.rs" "/checkout/library/core/src/slice/index.rs" "/checkout/library/core/tests/lazy.rs" "/checkout/library/core/src/slice/iter.rs" "/checkout/library/core/tests/iter/adapters/scan.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:18
