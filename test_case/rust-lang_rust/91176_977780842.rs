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
Diff in /checkout/library/std/src/sys/hermit/mutex.rs at line 53:
                 hint::spin_loop();
             } else {
                 counter = 0;
-                unsafe { abi::yield_now(); }
+                unsafe {
+                    abi::yield_now();
             }
         }
     }
     }
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/sys/hermit/os.rs" "/checkout/library/std/src/sys/sgx/stdio.rs" "/checkout/library/std/src/sys/hermit/args.rs" "/checkout/library/std/src/sys/sgx/os.rs" "/checkout/library/std/src/sys/hermit/mutex.rs" "/checkout/library/std/src/sys/sgx/args.rs" "/checkout/library/std/src/sys/hermit/net.rs" "/checkout/library/std/src/sys/sgx/thread_local_key.rs"` failed.
Build completed unsuccessfully in 0:00:14
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
