plain
 finished in 0.460 seconds
[TIMING] test::Compiletest { compiler: Compiler { stage: 2, host: x86_64-unknown-linux-gnu }, target: x86_64-unknown-linux-gnu, mode: "codegen-units", suite: "codegen-units", path: "src/test/codegen-units", compare_mode: None } -- 0.466
[TIMING] test::CodegenUnits { compiler: Compiler { stage: 2, host: x86_64-unknown-linux-gnu }, target: x86_64-unknown-linux-gnu } -- 0.000
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
thread 'main' panicked at 'missing LLVM component: loongarch', src/tools/compiletest/src/header.rs:1090:17
Build completed unsuccessfully in 0:13:00
