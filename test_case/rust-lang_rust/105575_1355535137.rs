plain
[RUSTC-TIMING] regex_automata test:false 15.002
[RUSTC-TIMING] chalk_ir test:false 4.488
[RUSTC-TIMING] crc32fast test:false 0.717
   Compiling atty v0.2.14
Assertion failed: !KeyInfoT::isEqual(Val, EmptyKey) && !KeyInfoT::isEqual(Val, TombstoneKey) && "Empty/Tombstone value shouldn't be inserted into map!", file D:\a\rust\rust\src\llvm-project\llvm\include\llvm/ADT/DenseMap.h, line 626
error: could not compile `miniz_oxide`

Caused by:
Caused by:
  process didn't exit successfully: `D:\a\rust\rust\build\bootstrap\debug\rustc --crate-name miniz_oxide --edition=2018 C:\Users\runneradmin\.cargo\registry\src\github.com-1285ae84e5963aae\miniz_oxide-0.5.3\src\lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -Zunstable-options --check-cfg "values(feature, \"alloc\", \"compiler_builtins\", \"core\", \"default\", \"rustc-dep-of-std\", \"simd\", \"simd-adler32\")" --check-cfg names() --check-cfg values() -C metadata=0a47feec5adcd076 -C extra-filename=-0a47feec5adcd076 --out-dir D:\a\rust\rust\build\i686-pc-windows-msvc\stage1-rustc\i686-pc-windows-msvc\release\deps --target i686-pc-windows-msvc -L dependency=D:\a\rust\rust\build\i686-pc-windows-msvc\stage1-rustc\i686-pc-windows-msvc\release\deps -L dependency=D:\a\rust\rust\build\i686-pc-windows-msvc\stage1-rustc\release\deps --extern adler=D:\a\rust\rust\build\i686-pc-windows-msvc\stage1-rustc\i686-pc-windows-msvc\release\deps\libadler-9a32e159c0ffb220.rmeta --cap-lints allow -Csymbol-mangling-version=v0 -Zunstable-options --check-cfg=values(bootstrap) --check-cfg=values(parallel_compiler) --check-cfg=values(no_btreemap_remove_entry) --check-cfg=values(crossbeam_loom) --check-cfg=values(span_locations) --check-cfg=values(rustix_use_libc) -Zmacro-backtrace -Csplit-debuginfo=packed -Ctarget-feature=+crt-static -Zunstable-options -Wrustc::internal -Cprefer-dynamic -Z binary-dep-depinfo` (exit code: 0xc000001d, STATUS_ILLEGAL_INSTRUCTION)
[RUSTC-TIMING] atty test:false 0.920
[RUSTC-TIMING] regex_syntax test:false 24.989
[RUSTC-TIMING] derive_more test:false 11.155
[RUSTC-TIMING] serde test:false 9.462
[RUSTC-TIMING] serde test:false 9.462
[RUSTC-TIMING] serde test:false 10.123
[RUSTC-TIMING] regex test:false 25.704
[RUSTC-TIMING] tracing_subscriber test:false 13.683
Build completed unsuccessfully in 0:17:31
make: *** [Makefile:75: ci-subset-2] Error 1
