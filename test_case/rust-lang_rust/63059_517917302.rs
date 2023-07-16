
   Compiling proc-macro2 v0.4.26
   Compiling autocfg v0.1.4
error: linker `lld-link.exe` not found
  |
  = note: The system cannot find the file specified. (os error 2)

note: the msvc targets depend on the msvc linker but `link.exe` was not found

note: please ensure that VS 2013, VS 2015, VS 2017 or VS 2019 was installed with the Visual C++ option

error: aborting due to previous error

error: Could not compile `proc-macro2`.
warning: build failed, waiting for other jobs to finish...
error: build failed
thread 'main' panicked at 'tests failed for https://github.com/servo/servo', src\tools\cargotest\main.rs:86:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
