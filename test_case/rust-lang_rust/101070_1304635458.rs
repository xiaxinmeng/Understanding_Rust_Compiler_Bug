json
    "rust-analyzer.checkOnSave.enable": true,
    "rust-analyzer.checkOnSave.overrideCommand": [
        "python3",
        "x.py",
        "check",
        "--json-output"
    ],
    "rust-analyzer.rustfmt.overrideCommand": [
        "./build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt",
        "--edition=2021"
    ],
    "rust-analyzer.cargo.buildScripts.enable": true,
    "rust-analyzer.cargo.buildScripts.invocationLocation": "root",
    "rust-analyzer.cargo.buildScripts.invocationStrategy": "once",
    "rust-analyzer.cargo.buildScripts.overrideCommand": [
        "python3",
        "x.py",
        "check",
        "--json-output"
    ],
    "rust-analyzer.rustc.source": "./Cargo.toml",
    "rust-analyzer.cargo.sysroot": "./build/x86_64-unknown-linux-gnu/stage0-sysroot",
    "rust-analyzer.procMacro.enable": true,
    "rust-analyzer.linkedProjects": ["Cargo.toml", "src/bootstrap/Cargo.toml"],
