
    Check compiletest suite=run-make mode=run-make (x86_64-pc-windows-msvc -> x86_64-pc-windows-msvc)
    Building rustdoc for stage2 (x86_64-pc-windows-msvc)

    running 159 tests
    test [run-make] run-make\allow-non-lint-warnings-cmdline ... FAILED
    test [run-make] run-make\alloc-extern-crates ... FAILED
    ...
    test [run-make] run-make\windows-subsystem ... FAILED

    failures:

    ---- [run-make] run-make\allow-non-lint-warnings-cmdline stdout ----
    	thread '[run-make] run-make\allow-non-lint-warnings-cmdline' panicked at 'failed to spawn `make`:     Error { repr: Os { code: 2, message: "The system cannot find the file specified." } }', src\libcore\result.rs:860:4
    note: Run with `RUST_BACKTRACE=1` for a backtrace.

    ---- [run-make] run-make\alloc-extern-crates stdout ----
    	thread '[run-make] run-make\alloc-extern-crates' panicked at 'failed to spawn `make`: Error { repr: Os { code: 2, message: "The system cannot find the file specified." } }', src\libcore\result.rs:860:4
    ...
    ---- [run-make] run-make\windows-subsystem stdout ----
	thread '[run-make] run-make\windows-subsystem' panicked at 'failed to spawn `make`: Error { repr: Os { code: 2, message: "The system cannot find the file specified." } }', src\libcore\result.rs:860:4

    failures:
        [run-make] run-make\a-b-a-linker-guard
        [run-make] run-make\alloc-extern-crates
    ...
        [run-make] run-make\windows-subsystem

    test result: FAILED. 0 passed; 159 failed; 0 ignored; 0 measured; 0 filtered out
