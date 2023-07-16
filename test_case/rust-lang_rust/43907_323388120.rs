
Check compiletest suite=debuginfo mode=debuginfo-gdb (x86_64-unknown-linux-gnu -> arm-linux-androideabi)
[00:58:24] arm-linux-androideabi debug-info test uses tcp 5039 port.please reserve it
[00:58:24] 
[00:58:24] running 108 tests
[00:58:24] test [debuginfo-gdb] debuginfo-gdb/associated-types.rs ... [100%] /data/tmp/work/associated-types.stage2-arm-linux-androideabi
[00:58:24] /checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/associated-types.stage2-arm-linux-androideabi: 1 file pushed. 1.8 MB/s (18292 bytes in 0.010s)
[00:58:24] FAILED
[00:58:24] test [debuginfo-gdb] debuginfo-gdb/basic-types-globals-metadata.rs ... Process /data/tmp/work/associated-types.stage2-arm-linux-androideabi created; pid = 31093
[00:58:24] Listening on port 5039
[00:58:24] [100%] /data/tmp/work/basic-types-globals-metadata.stage2-arm-linux-androideabi
[00:58:24] /checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/basic-types-globals-metadata.stage2-arm-linux-androideabi: 1 file pushed. 1.5 MB/s (12668 bytes in 0.008s)
[00:58:24] FAILED
[00:58:24] test [debuginfo-gdb] debuginfo-gdb/basic-types-globals.rs ... Process /data/tmp/work/basic-types-globals-metadata.stage2-arm-linux-androideabi created; pid = 31098
[00:58:24] Can't bind address: Address already in use.
[00:58:24] Killing process(es): 31098
[00:58:24] [100%] /data/tmp/work/basic-types-globals.stage2-arm-linux-androideabi
[00:58:24] /checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/basic-types-globals.stage2-arm-linux-androideabi: 1 file pushed. 2.0 MB/s (12332 bytes in 0.006s)
[00:58:24] FAILED
<snip>
[00:58:53] test [debuginfo-gdb] debuginfo-gdb/vec.rs ... Process /data/tmp/work/vec-slices.stage2-arm-linux-androideabi created; pid = 31573
[00:58:53] Can't bind address: Address already in use.
[00:58:53] Killing process(es): 31573
[00:58:53] [100%] /data/tmp/work/vec.stage2-arm-linux-androideabi
[00:58:53] /checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/vec.stage2-arm-linux-androideabi: 1 file pushed. 1.6 MB/s (9412 bytes in 0.005s)
[00:58:53] FAILED
[00:58:53] 
[00:58:53] failures:
[00:58:53] 
[00:58:53] ---- [debuginfo-gdb] debuginfo-gdb/associated-types.rs stdout ----
[00:58:53] 	NOTE: compiletest thinks it is using GDB without native rust support
[00:58:53] thread '[debuginfo-gdb] debuginfo-gdb/associated-types.rs' panicked at 'called `Option::unwrap()` on a `None` value', /checkout/src/libcore/option.rs:335:20
[00:58:53] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:53] 
[00:58:53] ---- [debuginfo-gdb] debuginfo-gdb/basic-types-globals-metadata.rs stdout ----
[00:58:53] 	NOTE: compiletest thinks it is using GDB without native rust support
[00:58:53] thread '[debuginfo-gdb] debuginfo-gdb/basic-types-globals-metadata.rs' panicked at 'called `Option::unwrap()` on a `None` value', /checkout/src/libcore/option.rs:335:20
[00:58:53] 
<snip>
[00:58:53] ---- [debuginfo-gdb] debuginfo-gdb/vec.rs stdout ----
[00:58:53] 	NOTE: compiletest thinks it is using GDB without native rust support
[00:58:53] thread '[debuginfo-gdb] debuginfo-gdb/vec.rs' panicked at 'called `Option::unwrap()` on a `None` value', /checkout/src/libcore/option.rs:335:20
[00:58:53] 
[00:58:53] 
[00:58:53] failures:
[00:58:53]     [debuginfo-gdb] debuginfo-gdb/associated-types.rs
[00:58:53]     [debuginfo-gdb] debuginfo-gdb/basic-types-globals-metadata.rs
<snip>
[00:58:53]     [debuginfo-gdb] debuginfo-gdb/vec.rs
[00:58:53] 
[00:58:53] test result: FAILED. 0 passed; 98 failed; 10 ignored; 0 measured; 0 filtered out
