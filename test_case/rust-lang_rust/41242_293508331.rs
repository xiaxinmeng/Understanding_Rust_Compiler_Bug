
[01:07:42] failures:
[01:07:42] 
[01:07:42] ---- [run-pass] run-pass/simple_global_asm.rs stdout ----
[01:07:42] 	
[01:07:42] error: compilation failed!
[01:07:42] status: exit code: 101
[01:07:42] command: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc /checkout/src/test/run-pass/simple_global_asm.rs -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass --target=arm-unknown-linux-gnueabihf --error-format json -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/simple_global_asm.stage2-arm-unknown-linux-gnueabihf.run-pass.libaux -C prefer-dynamic -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/simple_global_asm.stage2-arm-unknown-linux-gnueabihf -Clinker=arm-linux-gnueabihf-gcc -Crpath -O -Lnative=/checkout/obj/build/arm-unknown-linux-gnueabihf/native/rust-test-helpers
[01:07:42] thread 'main' panicked at 'Some tests failed', /checkout/src/tools/compiletest/src/main.rs:329
[01:07:42] stdout:
[01:07:42] ------------------------------------------
[01:07:42] 
[01:07:42] ------------------------------------------
[01:07:42] stderr:
[01:07:42] ------------------------------------------
[01:07:42] {"message":"linking with `arm-linux-gnueabihf-gcc` failed: exit code: 1","code":null,"level":"error","spans":[],"children":[{"message":"\"arm-linux-gnueabihf-gcc\" \"-Wl,--as-needed\" \"-Wl,-z,noexecstack\" \"-L\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-unknown-linux-gnueabihf/lib\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/simple_global_asm.0.o\" \"-o\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/simple_global_asm.stage2-arm-unknown-linux-gnueabihf\" \"-Wl,--gc-sections\" \"-pie\" \"-Wl,-O1\" \"-nodefaultlibs\" \"-L\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass\" \"-L\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/simple_global_asm.stage2-arm-unknown-linux-gnueabihf.run-pass.libaux\" \"-L\" \"/checkout/obj/build/arm-unknown-linux-gnueabihf/native/rust-test-helpers\" \"-L\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-unknown-linux-gnueabihf/lib\" \"-L\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-unknown-linux-gnueabihf/lib\" \"-l\" \"std-f4594d3e53dcb114\" \"-Wl,-Bstatic\" \"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-unknown-linux-gnueabihf/lib/libcompiler_builtins-987729be881d4d32.rlib\" \"-Wl,-Bdynamic\" \"-l\" \"dl\" \"-l\" \"rt\" \"-l\" \"pthread\" \"-l\" \"gcc_s\" \"-l\" \"pthread\" \"-l\" \"c\" \"-l\" \"m\" \"-l\" \"rt\" \"-l\" \"util\" \"-Wl,-rpath,$ORIGIN/../../stage2/lib/rustlib/arm-unknown-linux-gnueabihf/lib\" \"-Wl,-rpath,/usr/local/lib/rustlib/arm-unknown-linux-gnueabihf/lib\" \"-Wl,--enable-new-dtags\"","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/simple_global_asm.0.o: In function `simple_global_asm::main::h8f5a5f4abd38360a':\nsimple_global_asm.cgu-0.rs:(.text._ZN17simple_global_asm4main17h8f5a5f4abd38360aE+0x0): undefined reference to `foo'\ncollect2: error: ld returned 1 exit status\n","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":null}
[01:07:42] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":null}
[01:07:42] 
[01:07:42] ------------------------------------------
[01:07:42] 
[01:07:42] thread '[run-pass] run-pass/simple_global_asm.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2621
[01:07:42] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:07:42] 
[01:07:42] 
[01:07:42] failures:
[01:07:42]     [run-pass] run-pass/simple_global_asm.rs
[01:07:42] 
[01:07:42] test result: FAILED. 2647 passed; 1 failed; 8 ignored; 0 measured
