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
Diff in /checkout/library/std/src/sys/sgx/abi/usercalls/tests.rs at line 22:
             // Verify copy
             for byte in 0..size {
-                    assert_eq!(
-                    assert_eq!(
-                        *dst.as_ptr().offset(offset + byte as isize),
-                        src[byte as usize]
-                    );
+                    assert_eq!(*dst.as_ptr().offset(offset + byte as isize), src[byte as usize]);
             }
         }
         }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2021" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/sys/sgx/abi/tls/sync_bitset/tests.rs" "/checkout/library/std/src/sys/windows/process/tests.rs" "/checkout/library/std/src/sys/windows/c/errors.rs" "/checkout/library/std/src/sys/sgx/abi/mod.rs" "/checkout/library/std/src/sys/sgx/abi/mem.rs" "/checkout/library/std/src/sys/windows/mod.rs" "/checkout/library/std/src/sys/sgx/abi/panic.rs" "/checkout/library/std/src/sys/sgx/abi/usercalls/tests.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
