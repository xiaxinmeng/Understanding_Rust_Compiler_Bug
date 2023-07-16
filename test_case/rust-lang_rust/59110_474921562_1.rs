
Assembling stage2 compiler (x86_64-unknown-linux-musl)
[src/bootstrap/compile.rs:352] Std{compiler, target,} = Std {
    target: "x86_64-unknown-linux-musl",
    compiler: Compiler {
        stage: 2,
        host: "x86_64-unknown-linux-musl"
    }
}
Uplifting stage1 std (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-musl)
Copying stage2 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-musl / x86_64-unknown-linux-musl)
[src/bootstrap/compile.rs:965] Rustc{compiler: build_compiler, target: target_compiler.host,} = Rustc {
    target: "x86_64-unknown-linux-musl",
    compiler: Compiler {
        stage: 0,
        host: "x86_64-unknown-linux-gnu"
    }
}
[src/bootstrap/compile.rs:477] Test{compiler, target,} = Test {
    target: "x86_64-unknown-linux-musl",
    compiler: Compiler {
        stage: 0,
        host: "x86_64-unknown-linux-gnu"
    }
}
[src/bootstrap/compile.rs:352] Std{compiler, target,} = Std {
    target: "x86_64-unknown-linux-musl",
    compiler: Compiler {
        stage: 0,
        host: "x86_64-unknown-linux-gnu"
    }
}
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-musl)
