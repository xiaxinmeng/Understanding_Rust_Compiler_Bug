plain
stage-build INFO: Section `Stage 1 (LLVM PGO) > Gather profiles` starts
stage-build INFO: Running benchmarks with PGO instrumented LLVM
stage-build DEBUG: Changing working dir from `/checkout/obj` to `/checkout/obj`
stage-build INFO: Executing `RUSTC_BOOTSTRAP=1 /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc --edition 2021 --crate-type lib /checkout/library/core/src/lib.rs --out-dir /tmp/tmp-multistage/opt-artifacts`
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/../lib/librustc_driver-bdb8f22938953afe.so(+0x1cff453)[0x7fdd7266f453]
/lib64/libpthread.so.0(+0xf630)[0x7fdd70357630]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/../lib/librustc_driver-bdb8f22938953afe.so(+0x28f8849)[0x7fdd73268849]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/../lib/librustc_driver-bdb8f22938953afe.so(+0x1c5192e)[0x7fdd725c192e]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/../lib/librustc_driver-bdb8f22938953afe.so(_RNvXs8_NtCs2FDg0MlkKez_19rustc_mir_transform10const_propNtB5_15ConstPropagatorNtNtNtCscpSs464Bm1i_12rustc_middle3mir5visit10MutVisitor16visit_terminator+0x1ea)[0x7fdd7367290a]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/../lib/librustc_driver-bdb8f22938953afe.so(_RNvXs8_NtCs2FDg0MlkKez_19rustc_mir_transform10const_propNtB5_15ConstPropagatorNtNtNtCscpSs464Bm1i_12rustc_middle3mir5visit10MutVisitor22visit_basic_block_data+0x59)[0x7fdd73672ef9]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/../lib/librustc_driver-bdb8f22938953afe.so(_RNvXNtCs2FDg0MlkKez_19rustc_mir_transform10const_propNtB2_9ConstPropNtNtCscpSs464Bm1i_12rustc_middle3mir7MirPass8run_pass+0xc89)[0x7fdd736787b9]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/../lib/librustc_driver-bdb8f22938953afe.so(+0x2d22a61)[0x7fdd73692a61]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/../lib/librustc_driver-bdb8f22938953afe.so(+0x2c4b275)[0x7fdd735bb275]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/../lib/librustc_driver-bdb8f22938953afe.so(+0x30d6df4)[0x7fdd73a46df4]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/../lib/librustc_driver-bdb8f22938953afe.so(+0x3321687)[0x7fdd73c91687]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/../lib/librustc_driver-bdb8f22938953afe.so(+0x27d73f5)[0x7fdd731473f5]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/../lib/librustc_driver-bdb8f22938953afe.so(+0x27e99a1)[0x7fdd731599a1]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/../lib/librustc_driver-bdb8f22938953afe.so(+0x2714a3c)[0x7fdd73084a3c]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/../lib/librustc_driver-bdb8f22938953afe.so(_RNvNtNtCsbkGdgfzJi5X_14rustc_metadata5rmeta7encoder15encode_metadata+0x83)[0x7fdd73158e93]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/../lib/librustc_driver-bdb8f22938953afe.so(_RNvNtCsbkGdgfzJi5X_14rustc_metadata2fs25encode_and_write_metadata+0x378)[0x7fdd73090b68]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/../lib/librustc_driver-bdb8f22938953afe.so(_RNvNtCsamg3R950zah_15rustc_interface6passes13start_codegen+0x17f)[0x7fdd72e8c6ff]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/../lib/librustc_driver-bdb8f22938953afe.so(+0x2523318)[0x7fdd72e93318]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/../lib/librustc_driver-bdb8f22938953afe.so(_RNvMs3_NtCsamg3R950zah_15rustc_interface7queriesNtB5_7Queries15ongoing_codegen+0x60)[0x7fdd72f132b0]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/../lib/librustc_driver-bdb8f22938953afe.so(+0x1d37a13)[0x7fdd726a7a13]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/../lib/librustc_driver-bdb8f22938953afe.so(+0x1cf1a88)[0x7fdd72661a88]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/../lib/librustc_driver-bdb8f22938953afe.so(+0x1cf478d)[0x7fdd7266478d]
/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/../lib/libstd-8c5537594e196f18.so(rust_metadata_std_4983dc6ebaf4b336+0x10c793)[0x7fdd74a6b793]
/lib64/libpthread.so.0(+0x7ea5)[0x7fdd7034fea5]
/lib64/libc.so.6(clone+0x6d)[0x7fdd70078b0d]
stage-build INFO: Section `Stage 1 (LLVM PGO) > Gather profiles` ended: FAIL (14.68s)
stage-build INFO: Section `Stage 1 (LLVM PGO)` ended: FAIL (1397.34s)
stage-build ERROR: The multi-stage build has failed
stage-build INFO: Timer results
---
Total duration:                          23m 17s
------------------------------------------------
root INFO: Free disk space: 587.31 GiB out of total 666.61 GiB (11.89% used)
Traceback (most recent call last):
  File "../src/ci/stage-build.py", line 839, in <module>
    raise e
  File "../src/ci/stage-build.py", line 836, in <module>
    execute_build_pipeline(timer, pipeline, build_args)
  File "../src/ci/stage-build.py", line 765, in execute_build_pipeline
    gather_llvm_profiles(pipeline)
  File "../src/ci/stage-build.py", line 589, in gather_llvm_profiles
    crates=LLVM_PGO_CRATES
  File "../src/ci/stage-build.py", line 472, in run_compiler_benchmarks
    ], env=dict(RUSTC_BOOTSTRAP="1", **env))
  File "../src/ci/stage-build.py", line 452, in cmd
    return subprocess.run(args, env=environment, check=True)
  File "/usr/lib64/python3.6/subprocess.py", line 438, in run
    output=stdout, stderr=stderr)
subprocess.CalledProcessError: Command '['/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc', '--edition', '2021', '--crate-type', 'lib', '/checkout/library/core/src/lib.rs', '--out-dir', '/tmp/tmp-multistage/opt-artifacts']' died with <Signals.SIGSEGV: 11>.
