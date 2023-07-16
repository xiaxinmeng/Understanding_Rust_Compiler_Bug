plain
warning:                                               ^
warning: 1 warning generated.
[RUSTC-TIMING] profiler_builtins test:false 0.045
 Documenting std v0.0.0 (D:\a\rust\rust\library\std)
thread '<unnamed>' panicked at 'attempt to subtract with overflow', C:\Users\runneradmin\.cargo\registry\src\github.com-1ecc6299db9ec823\rustc-rayon-core-0.3.1\src\sleep\mod.rs:330:21

error: internal compiler error: unexpected panic

error: Unrecognized option: 'markdown-css'
error: Unrecognized option: 'markdown-css'

Rayon: detected unexpected panic; aborting
thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: PoisonError { .. }', C:\Users\runneradmin\.cargo\registry\src\github.com-1ecc6299db9ec823\rustc-rayon-core-0.3.1\src\sleep\mod.rs:183:
error: could not document `std`
Caused by:
Caused by:
  process didn't exit successfully: `D:\a\rust\rust\build\bootstrap/debug/rustdoc --edition=2018 --crate-type dylib --crate-type rlib --crate-name std library\std\src\lib.rs --target x86_64-pc-windows-msvc -o D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-std\x86_64-pc-windows-msvc\doc --cfg "feature=\"addr2line\"" --cfg "feature=\"backtrace\"" --cfg "feature=\"compiler-builtins-c\"" --cfg "feature=\"gimli-symbolize\"" --cfg "feature=\"miniz_oxide\"" --cfg "feature=\"object\"" --cfg "feature=\"panic_unwind\"" --cfg "feature=\"profiler\"" --cfg "feature=\"profiler_builtins\"" --cfg "feature=\"std_detect_dlsym_getauxval\"" --cfg "feature=\"std_detect_file_io\"" --error-format=json --json=diagnostic-rendered-ansi --markdown-css rust.css --markdown-no-toc -Z unstable-options --resource-suffix 1.58.0 --index-page D:\a\rust\rust\src/doc/index.md -L dependency=D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-std\x86_64-pc-windows-msvc\release\deps -L dependency=D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-std\release\deps --extern addr2line=D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-std\x86_64-pc-windows-msvc\release\deps\libaddr2line-cf11887e347bade5.rmeta --extern alloc=D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-std\x86_64-pc-windows-msvc\release\deps\liballoc-b283fed844d10fdc.rmeta --extern cfg_if=D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-std\x86_64-pc-windows-msvc\release\deps\libcfg_if-a1ee9feefed99f72.rmeta --extern compiler_builtins=D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-std\x86_64-pc-windows-msvc\release\deps\libcompiler_builtins-295e6ad7c6062cdb.rmeta --extern core=D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-std\x86_64-pc-windows-msvc\release\deps\libcore-32ff48abea5dad46.rmeta --extern hashbrown=D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-std\x86_64-pc-windows-msvc\release\deps\libhashbrown-9b6df23d5e4e993c.rmeta --extern libc=D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-std\x86_64-pc-windows-msvc\release\deps\liblibc-ab427bbde3411ee2.rmeta --extern miniz_oxide=D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-std\x86_64-pc-windows-msvc\release\deps\libminiz_oxide-080b325d9b90f74f.rmeta --extern object=D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-std\x86_64-pc-windows-msvc\release\deps\libobject-7ac310d5d1dc9671.rmeta --extern panic_abort=D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-std\x86_64-pc-windows-msvc\release\deps\libpanic_abort-6e3b294cc2031538.rmeta --extern panic_unwind=D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-std\x86_64-pc-windows-msvc\release\deps\libpanic_unwind-f1df0c4029384187.rmeta --extern profiler_builtins=D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-std\x86_64-pc-windows-msvc\release\deps\libprofiler_builtins-a15fa0671ead8500.rmeta --extern rustc_demangle=D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-std\x86_64-pc-windows-msvc\release\deps\librustc_demangle-bdaaf91f670914ee.rmeta --extern std_detect=D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-std\x86_64-pc-windows-msvc\release\deps\libstd_detect-418a7e03f8777010.rmeta --extern unwind=D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-std\x86_64-pc-windows-msvc\release\deps\libunwind-bc9e722e7c53e3b5.rmeta -Zsymbol-mangling-version=legacy -Dwarnings -Wrustdoc::invalid_codeblock_attributes --crate-version "1.58.0-nightly
  (84e835883
  2021-10-24)" "-Zcrate-attr=doc(html_root_url=\"https://doc.rust-lang.org/nightly/\")" --cfg backtrace_in_libstd` (exit code: 0xc0000409, STATUS_STACK_BUFFER_OVERRUN)


command did not execute successfully: "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustdoc" "--target" "x86_64-pc-windows-msvc" "-Zbinary-dep-depinfo" "-j" "8" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace profiler compiler-builtins-c" "--manifest-path" "D:\\a\\rust\\rust\\library/test/Cargo.toml" "-p" "std" "-Zskip-rustdoc-fingerprint" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "-Z" "unstable-options" "--resource-suffix" "1.58.0" "--index-page" "D:\\a\\rust\\rust\\src/doc/index.md"


Build completed unsuccessfully in 0:45:27
Build completed unsuccessfully in 0:45:27
make: *** [Makefile:74: ci-subset-2] Error 1
