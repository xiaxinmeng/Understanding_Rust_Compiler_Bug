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
skip untracked path src/llvm-project/ during rustfmt invocations
Diff in /checkout/library/std/src/sys/windows/stdio.rs at line 145:
                 return Ok(1);
             } else {
                 return Err(io::Error::new(
-                        io::ErrorKind::InvalidData,
-                        "Windows stdio in console mode does not support writing non-UTF-8 byte sequences",
-                    ));
+                    io::ErrorKind::InvalidData,
+                    "Windows stdio in console mode does not support writing non-UTF-8 byte sequences",
             }
         }
         }
         Err(e) => str::from_utf8(&data[..e.valid_up_to()]).unwrap(),
Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/library/std/src/sys/windows/args.rs" "/checkout/library/std/src/sys/windows/thread.rs" "/checkout/library/std/src/sys/windows/stack_overflow.rs" "/checkout/library/std/src/sys/windows/thread_parker.rs" "/checkout/library/std/src/sys/windows/path/tests.rs" "/checkout/library/std/src/sys/windows/io.rs" "/checkout/library/std/src/sys/windows/mutex.rs" "/checkout/library/std/src/sys/windows/stdio.rs"` failed.
If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
Build completed unsuccessfully in 0:00:15
