
{
    "rust-analyzer.rustc.source": "./Cargo.toml",
    "rust-analyzer.linkedProjects": [
        "./Cargo.toml",
        //"./src/bootstrap/Cargo.toml"
    ],
    "rust-analyzer.checkOnSave.overrideCommand": [
        "python3",
        "x.py",
        "check",
        "--json-output",
        "library/std",
        "compiler/rustc",
    ],
    // This also affects proc macros
    "rust-analyzer.cargo.buildScripts.overrideCommand": [
        "python3",
        "x.py",
        "check",
        "--json-output",
        "compiler/rustc",
    ],
    "rust-analyzer.rustfmt.overrideCommand": [
        "./build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt",
        "--edition=2021"
    ],
    "files.watcherExclude": {
        "*rustc*/src/llvm-project/**": true,
        "*rustc*/build/**": true,
    },
    "files.exclude": {
        "src/llvm-project/**": true,
        "build/**": true
    },
}
