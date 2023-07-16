
v4v-t7-1j-prg06 11:34 /builds/psumbera/userland/components/desktop/firefox/firefox-68.11.0: /builds/psumbera/rustc-1.44.0/bin/rustc --crate-name gkrust toolkit/library/rust/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type staticlib --emit=dep-info,link -C opt-level=2 -C panic=abort -C codegen-units=1 -C lto --cfg 'feature="bindgen"' --cfg 'feature="cubeb_pulse_rust"' --cfg 'feature="moz_memory"' --cfg 'feature="moz_places"' --cfg 'feature="quantum_render"' --cfg 'feature="servo"' -C metadata=02c56202166b3eec -C extra-filename=-02c56202166b3eec --out-dir    /builds/psumbera/userland/components/desktop/firefox/build/sparcv9/sparcv9-sun-solaris/release/deps --target sparcv9-sun-solaris -C linker=/builds/psumbera/userland/components/desktop/firefox/firefox-68.11.0/build/cargo-linker -L dependency=/builds/psumbera/userland/components/desktop/firefox/build/sparcv9/sparcv9-sun-solaris/release/deps -L   dependency=/builds/psumbera/userland/components/desktop/firefox/build/sparcv9/release/deps --extern   gkrust_shared=/builds/psumbera/userland/components/desktop/firefox/build/sparcv9/sparcv9-sun-solaris/release/deps/libgkrust_shared-9712e354bc55934b.rlib --extern    mozilla_central_workspace_hack=/builds/psumbera/userland/components/desktop/firefox/build/sparcv9/sparcv9-sun-solaris/release/deps/libmozilla_central_workspace_hack-4044d7b7b8cbdafc.rlib -C opt-level=2 --cap-lints warn -L  native=/builds/psumbera/userland/components/desktop/firefox/build/sparcv9/sparcv9-sun-solaris/release/build/lmdb-rkv-sys-c379f0737c738302/out
Segmentation Fault (core dumped)
v4v-t7-1j-prg06 11:34 /builds/psumbera/userland/components/desktop/firefox/firefox-68.11.0: mdb core
Loading modules: [ libc.so.1 ld.so.1 ]
rustc:core> $G
C++ symbol demangling enabled
rustc:core> $C
001ffc587e7edd01 libc.so.1`_memcpy%sun4v-hwcap4+0x178(800000f72ce636e0, 2, 10, 1, 10, 800000f72ce636e0)
001ffc587e7eddb1 libstd-c4661e6d7c8d6da6.so`core::slice::_$LT$impl$u20$$u5b$T$u5d$$GT$::copy_from_slice::h166d66d659ae0c54+0x34(800000f72ce636e0, 10, 1, 10, 1ffc5888e5a000, 0)
001ffc587e7edf11 libstd-c4661e6d7c8d6da6.so`_$LT$$RF$str$u20$as$u20$std..ffi..c_str..CString..new..SpecIntoVec$GT$::into_vec::h4795adcfb8bd244e+0xb4(1ffc587e7eecc0, 1, 10, 0, 0,
b00000f72ed3cfe0)
001ffc587e7edff1 librustc_driver-3c8c0481c1cf7450.so`rustc_codegen_llvm::back::lto::run_fat::ha2464fbe0a02f1ea+0xfe4(c00000f72b235990, 2, 1ffc587e7eecc0, c00000f72b2359c8, 1ffc587e7eec28,
1ffc587e7eec21)
001ffc587e7ee4f1 librustc_driver-3c8c0481c1cf7450.so`rustc_codegen_ssa::back::write::generate_lto_work::h2a65c10501651b11+0xbc(1ffc587e7ef430, 1ffc587e7ef088, 1ffc587e7ef5b0, 1ffc587e7ef490,
1ffc587e7ef680, b00000f72d2b3210)
001ffc587e7ee691 librustc_driver-3c8c0481c1cf7450.so`std::sys_common::backtrace::__rust_begin_short_backtrace::h9e597b6c1402aedf+0x400(1ffc587e7ef430, 1ffc587e7ef688, 1ffc587e7ef498, 0, 0,
1ffc587e7ef6b8)
001ffc587e7eefa1 librustc_driver-3c8c0481c1cf7450.so`core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::h89fb6c455081aef0+0x98(1ffc587e7efce8, 1ffc587e7efc00,
600000f72d28f5b0, 1ffc587e7ef930, 168, 0)
001ffc587e7ef661 libstd-c4661e6d7c8d6da6.so`std::sys::unix::thread::Thread::new::thread_start::h363b2080c72fef07+0x20(1ffc5888e86000, 1ffc588cb19bc4, b00000f72d28f770, 600000f72d28f5b0,
1ffc5892985470, 0)
001ffc587e7ef731 libc.so.1`_lwp_start(0, 0, 0, 0, 0, 0)
rustc:core>
