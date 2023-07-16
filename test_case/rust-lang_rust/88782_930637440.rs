plain
    Finished dev [unoptimized + debuginfo] target(s) in 0.41s
Set({"src/tools/tidy"}) not skipped for "bootstrap::test::Tidy" -- not in ["src/test/ui", "src/tools/linkchecker"]
Skipping Suite("src/test/ui") because it is excluded
Suite("src/test/run-pass-valgrind") not skipped for "bootstrap::test::RunPassValgrind" -- not in ["src/test/ui", "src/tools/linkchecker"]
thread 'main' panicked at '"lldb" "-P" failed Output { status: ExitStatus(ExitStatus(3221225781)), stdout: "", stderr: "" }', src\bootstrap\test.rs:1394:40
Build completed unsuccessfully in 0:00:01
Build completed unsuccessfully in 0:00:01
make: *** [Makefile:72: ci-subset-1] Error 1
