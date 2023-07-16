plain
    Finished release [optimized + debuginfo] target(s) in 46.27s
Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[TIMING] StdLink { compiler: Compiler { stage: 0, host: x86_64-unknown-linux-gnu }, target_compiler: Compiler { stage: 0, host: x86_64-unknown-linux-gnu }, target: x86_64-unknown-linux-gnu } -- 0.000
[TIMING] Std { target: x86_64-unknown-linux-gnu, compiler: Compiler { stage: 0, host: x86_64-unknown-linux-gnu } } -- 46.296
thread 'main' panicked at 'assertion failed: self.llvm_from_ci', src/bootstrap/config.rs:1229:9
Build completed unsuccessfully in 0:00:46
