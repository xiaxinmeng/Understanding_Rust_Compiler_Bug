plain
[TIMING] TheBook { compiler: Compiler { stage: 2, host: TargetSelection { triple: "x86_64-pc-windows-gnu", file: None } }, target: TargetSelection { triple: "x86_64-pc-windows-gnu", file: None } } -- 3.075
Set({"library\\alloc", "library\\core", "library\\panic_abort", "library\\panic_unwind", "library\\proc_macro", "library\\profiler_builtins", "library\\std", "library\\test", "library\\unwind"}) not skipped for "bootstrap::doc::Std" -- not in ["src/test/ui"]
Documenting stage2 std (x86_64-pc-windows-gnu)
 Documenting core v0.0.0 (D:\a\rust\rust\library\core)
thread '<unnamed>' panicked at 'attempt to subtract with overflow', C:\Users\runneradmin\.cargo\registry\src\github.com-1ecc6299db9ec823\rustc-rayon-core-0.3.1\src\sleep\mod.rs:330:21

error: internal compiler error: unexpected panic

error: Unrecognized option: 'markdown-css'
error: Unrecognized option: 'markdown-css'

Rayon: detected unexpected panic; aborting
thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: PoisonError { .. }', C:\Users\runneradmin\.cargo\registry\src\github.com-1ecc6299db9ec823\rustc-rayon-core-0.3.1\src\sleep\mod.rs:299
error: could not document `core`
Caused by:
Caused by:
  process didn't exit successfully: `D:\a\rust\rust\build\bootstrap/debug/rustdoc --edition=2018 --crate-type lib --crate-name core library\core\src\lib.rs --target x86_64-pc-windows-gnu -o D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage2-std\x86_64-pc-windows-gnu\doc --error-format=json --json=diagnostic-rendered-ansi --markdown-css rust.css --markdown-no-toc -Z unstable-options --resource-suffix 1.58.0 --index-page D:\a\rust\rust\src/doc/index.md -L dependency=D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage2-std\x86_64-pc-windows-gnu\release\deps -L dependency=D:\a\rust\rust\build\x86_64-pc-windows-gnu\stage2-std\release\deps -Zsymbol-mangling-version=legacy -Dwarnings -Wrustdoc::invalid_codeblock_attributes --crate-version "1.58.0-nightly
  (7dbdae0c4
  2021-10-24)" "-Zcrate-attr=doc(html_root_url=\"https://doc.rust-lang.org/nightly/\")"` (exit code: 0xc0000409, STATUS_STACK_BUFFER_OVERRUN)


command did not execute successfully: "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\stage0\\bin\\cargo.exe" "rustdoc" "--target" "x86_64-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "8" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace profiler compiler-builtins-c" "--manifest-path" "D:\\a\\rust\\rust\\library/test/Cargo.toml" "-p" "core" "-Zskip-rustdoc-fingerprint" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "-Z" "unstable-options" "--resource-suffix" "1.58.0" "--index-page" "D:\\a\\rust\\rust\\src/doc/index.md"


Build completed unsuccessfully in 1:08:04
Build completed unsuccessfully in 1:08:04
make: *** [Makefile:80: ci-mingw-subset-1] Error 1
