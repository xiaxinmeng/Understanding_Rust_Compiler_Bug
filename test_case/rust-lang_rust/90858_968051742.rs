plain
[RUSTC-TIMING] pin_project_lite test:false 0.065
   Compiling either v1.6.0
[RUSTC-TIMING] rustc_hash test:false 0.080
   Compiling rustc-rayon-core v0.3.1
/checkout/obj/build/i686-unknown-linux-gnu/stage0/bin/../lib/librustc_driver-64918735d27864fa.so(+0x6adfc3)[0xef6fafc3]
[0xf7f6ab50]
/checkout/obj/build/i686-unknown-linux-gnu/stage0/bin/../lib/librustc_driver-64918735d27864fa.so(+0x27ff4e6)[0xf184c4e6]
/checkout/obj/build/i686-unknown-linux-gnu/stage0/bin/../lib/librustc_driver-64918735d27864fa.so(+0x2800974)[0xf184d974]
/checkout/obj/build/i686-unknown-linux-gnu/stage0/bin/../lib/librustc_driver-64918735d27864fa.so(+0xd71de1)[0xefdbede1]
/checkout/obj/build/i686-unknown-linux-gnu/stage0/bin/../lib/librustc_driver-64918735d27864fa.so(+0x2800399)[0xf184d399]
/checkout/obj/build/i686-unknown-linux-gnu/stage0/bin/../lib/librustc_driver-64918735d27864fa.so(+0x25813a7)[0xf15ce3a7]
/checkout/obj/build/i686-unknown-linux-gnu/stage0/bin/../lib/librustc_driver-64918735d27864fa.so(+0xa64890)[0xefab1890]
/checkout/obj/build/i686-unknown-linux-gnu/stage0/bin/../lib/librustc_driver-64918735d27864fa.so(+0xa56b76)[0xefaa3b76]
   Compiling arrayvec v0.7.0
[RUSTC-TIMING] build_script_build test:false 0.361
   Compiling remove_dir_all v0.5.3
[RUSTC-TIMING] build_script_build test:false 0.408
[RUSTC-TIMING] build_script_build test:false 0.408
   Compiling stable_deref_trait v1.2.0
[RUSTC-TIMING] remove_dir_all test:false 0.055
   Compiling rustc_graphviz v0.0.0 (/checkout/compiler/rustc_graphviz)
[RUSTC-TIMING] smallvec test:false 0.450
rustc exited with signal: 11 (core dumped)
error: could not compile `smallvec`
Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name smallvec --edition=2018 /cargo/registry/src/github.com-1285ae84e5963aae/smallvec-1.7.0/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on --cfg 'feature="may_dangle"' --cfg 'feature="union"' -C metadata=b563e70fe347d393 -C extra-filename=-b563e70fe347d393 --out-dir /checkout/obj/build/i686-unknown-linux-gnu/stage0-rustc/i686-unknown-linux-gnu/release/deps --target i686-unknown-linux-gnu -L dependency=/checkout/obj/build/i686-unknown-linux-gnu/stage0-rustc/i686-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/i686-unknown-linux-gnu/stage0-rustc/release/deps --cap-lints allow --cfg=bootstrap -Zsymbol-mangling-version=v0 -Zmacro-backtrace '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Ztls-model=initial-exec -Zunstable-options '-Wrustc::internal' -Cprefer-dynamic -Z binary-dep-depinfo` (exit status: 254)
[RUSTC-TIMING] stable_deref_trait test:false 0.043
[RUSTC-TIMING] build_script_build test:false 0.268
[RUSTC-TIMING] build_script_build test:false 0.529
[RUSTC-TIMING] ppv_lite86 test:false 0.487
