
error: internal compiler error: src\librustc\dep_graph\graph.rs:688: DepNode Hir(screensnap[29a7]::screengrab[0]::Screenshot[0]::data[0]) should have been pre-allocated but wasn't.

thread 'rustc' panicked at 'Box<Any>', src\librustc_errors\lib.rs:637:9
note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.36.0 (a53f9df32 2019-07-03) running on x86_64-pc-windows-gnu

note: compiler flags: -C debuginfo=2 -C ar=D:\Programmes\msys64\mingw64\bin\ar.exe -C linker=D:\Programmes\msys64\mingw64\bin\gcc.exe -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `screensnap`.
