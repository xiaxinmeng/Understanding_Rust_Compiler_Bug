
PS D:\RustProjects\frank-wolfe-prefer> cargo run --example local-storage --verbose
       Fresh cfg-if v0.1.10
       Fresh ppv-lite86 v0.2.6
       Fresh getrandom v0.1.13
       Fresh c2-chacha v0.2.3
       Fresh rand_core v0.5.1
       Fresh rand_chacha v0.2.1
       Fresh rand v0.7.2
   Compiling frank-wolfe-prefer v0.1.0 (D:\RustProjects\frank-wolfe-prefer)
     Running `rustc --edition=2018 --crate-name frank_wolfe_prefer src\lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata,link -C debuginfo=2 -C metadata=74fbd18809d153c3 -C extra-filename=-74fbd18809d153c3 --out-dir D:\RustProjects\frank-wolfe-prefer\target\debug\deps -C incremental=D:\RustProjects\frank-wolfe-prefer\target\debug\incremental -L dependency=D:\RustProjects\frank-wolfe-prefer\target\debug\deps --extern rand=D:\RustProjects\frank-wolfe-prefer\target\debug\deps\librand-93099efcd584b049.rmeta`
error: internal compiler error: src\librustc\ty\subst.rs:651: const parameter `R/#1` (Const { ty: usize, val: Param(R/#1) }/1) out of range when substituting substs=[]

thread 'rustc' panicked at 'Box<Any>', src\librustc_errors\lib.rs:890:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.40.0-nightly (1423bec54 2019-11-05) running on x86_64-pc-windows-msvc

note: compiler flags: -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

error: aborting due to previous error

error: could not compile `frank-wolfe-prefer`.

Caused by:
  process didn't exit successfully: `rustc --edition=2018 --crate-name frank_wolfe_prefer src\lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata,link -C debuginfo=2 -C metadata=74fbd18809d153c3 -C extra-filename=-74fbd18809d153c3 --out-dir D:\RustProjects\frank-wolfe-prefer\target\debug\deps -C incremental=D:\RustProjects\frank-wolfe-prefer\target\debug\incremental -L dependency=D:\RustProjects\frank-wolfe-prefer\target\debug\deps --extern rand=D:\RustProjects\frank-wolfe-prefer\target\debug\deps\librand-93099efcd584b049.rmeta` (exit code: 101)
