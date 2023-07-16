
2021-11-12T22:34:46.0890595Z [0m[0m[1m[32m   Compiling[0m autocfg v1.0.0
2021-11-12T22:34:46.0892018Z [0m[0m[1m[32m   Compiling[0m libc v0.2.106
2021-11-12T22:34:46.0893233Z [0m[0m[1m[32m   Compiling[0m proc-macro2 v1.0.30
2021-11-12T22:34:46.0894476Z [0m[0m[1m[32m   Compiling[0m unicode-xid v0.2.2
2021-11-12T22:34:46.0895652Z [0m[0m[1m[32m   Compiling[0m cfg-if v0.1.10
2021-11-12T22:34:46.0897143Z [0m[0m[1m[32m   Compiling[0m syn v1.0.80
2021-11-12T22:34:46.0898332Z [0m[0m[1m[32m   Compiling[0m lazy_static v1.4.0
2021-11-12T22:34:46.0899500Z [0m[0m[1m[32m   Compiling[0m cfg-if v1.0.0
2021-11-12T22:34:46.0900691Z [0m[0m[1m[32m   Compiling[0m scopeguard v1.1.0
2021-11-12T22:34:46.0901841Z [0m[0m[1m[32m   Compiling[0m log v0.4.14
2021-11-12T22:34:46.0960995Z [0m[0m[1m[32m   Compiling[0m smallvec v1.7.0
2021-11-12T22:34:46.1008915Z [0m[0m[1m[32m   Compiling[0m cc v1.0.69
2021-11-12T22:34:46.1017627Z [0m[0m[1m[32m   Compiling[0m instant v0.1.6
2021-11-12T22:34:46.1039324Z [0m[0m[1m[32m   Compiling[0m maybe-uninit v2.0.0
2021-11-12T22:34:46.1040354Z [0m[0m[1m[32m   Compiling[0m getrandom v0.2.0
2021-11-12T22:34:46.1041268Z [0m[0m[1m[32m   Compiling[0m hashbrown v0.11.0
2021-11-12T22:34:46.1398646Z [RUSTC-TIMING] cfg_if test:false 0.040
2021-11-12T22:34:46.1402728Z [0m[0m[1m[32m   Compiling[0m bitflags v1.2.1
2021-11-12T22:34:46.1563110Z [RUSTC-TIMING] cfg_if test:false 0.043
2021-11-12T22:34:46.1568409Z [0m[0m[1m[32m   Compiling[0m ppv-lite86 v0.2.8
2021-11-12T22:34:46.1621281Z [RUSTC-TIMING] scopeguard test:false 0.058
2021-11-12T22:34:46.1628826Z [0m[0m[1m[32m   Compiling[0m pin-project-lite v0.2.4
2021-11-12T22:34:46.1813489Z [RUSTC-TIMING] lazy_static test:false 0.080
2021-11-12T22:34:46.1836993Z [RUSTC-TIMING] instant test:false 0.057
2021-11-12T22:34:46.1841694Z [0m[0m[1m[32m   Compiling[0m typenum v1.12.0
2021-11-12T22:34:46.1844092Z [0m[0m[1m[32m   Compiling[0m version_check v0.9.3
2021-11-12T22:34:46.2124837Z [RUSTC-TIMING] unicode_xid test:false 0.115
2021-11-12T22:34:46.2129790Z [0m[0m[1m[32m   Compiling[0m either v1.6.0
2021-11-12T22:34:46.2448673Z [RUSTC-TIMING] pin_project_lite test:false 0.076
2021-11-12T22:34:46.2453805Z [0m[0m[1m[32m   Compiling[0m rustc-hash v1.1.0
2021-11-12T22:34:46.3323728Z [RUSTC-TIMING] build_script_build test:false 0.221
2021-11-12T22:34:46.3327818Z rustc exited with signal: 11 (core dumped)
2021-11-12T22:34:46.3335183Z [0m[0m[1m[31merror[0m[1m:[0m could not compile `getrandom`
2021-11-12T22:34:46.3336017Z 
2021-11-12T22:34:46.3337149Z Caused by:
2021-11-12T22:34:46.3344316Z   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name build_script_build --edition=2018 /cargo/registry/src/github.com-1ecc6299db9ec823/getrandom-0.2.0/build.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=off --cfg 'feature="std"' -C metadata=301bf632479934c1 -C extra-filename=-301bf632479934c1 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/build/getrandom-301bf632479934c1 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --cap-lints allow -Z binary-dep-depinfo` (exit status: 254)
2021-11-12T22:34:46.3348971Z [0m[0m[1m[33mwarning[0m[1m:[0m build failed, waiting for other jobs to finish...
2021-11-12T22:34:46.3478946Z [RUSTC-TIMING] rustc_hash test:false 0.096
2021-11-12T22:34:46.3712607Z [RUSTC-TIMING] either test:false 0.153
2021-11-12T22:34:46.3963633Z [RUSTC-TIMING] smallvec test:false 0.288
2021-11-12T22:34:46.4509201Z [RUSTC-TIMING] autocfg test:false 0.355
2021-11-12T22:34:46.4656422Z [RUSTC-TIMING] build_script_build test:false 0.355
2021-11-12T22:34:46.4867457Z [RUSTC-TIMING] version_check test:false 0.294
2021-11-12T22:34:46.5465364Z [RUSTC-TIMING] build_script_build test:false 0.418
2021-11-12T22:34:46.5740029Z [RUSTC-TIMING] build_script_build test:false 0.477
2021-11-12T22:34:46.5827616Z [RUSTC-TIMING] build_script_build test:false 0.423
2021-11-12T22:34:46.5871691Z [RUSTC-TIMING] ppv_lite86 test:false 0.425
2021-11-12T22:34:46.5902842Z [RUSTC-TIMING] build_script_build test:false 0.494
2021-11-12T22:34:46.5997624Z [RUSTC-TIMING] build_script_build test:false 0.500
2021-11-12T22:34:46.6786739Z [RUSTC-TIMING] hashbrown test:false 0.568
2021-11-12T22:34:46.7561868Z [RUSTC-TIMING] build_script_main test:false 0.565
2021-11-12T22:34:46.9026757Z [RUSTC-TIMING] cc test:false 0.796
2021-11-12T22:34:46.9186028Z [0m[0m[1m[31merror[0m[1m:[0m build failed
2021-11-12T22:34:46.9220757Z Build completed unsuccessfully in 0:02:51
2021-11-12T22:34:46.9343164Z == clock drift check ==
2021-11-12T22:34:46.9351870Z   local time: Fri Nov 12 22:34:46 UTC 2021
2021-11-12T22:34:47.2017603Z   network time: Thu, 11 Nov 2021 23:29:52 GMT
2021-11-12T22:34:47.2018813Z == end clock drift check ==
2021-11-12T22:34:49.3059588Z ##[error]Process completed with exit code 1.
