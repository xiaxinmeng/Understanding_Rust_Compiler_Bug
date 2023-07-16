sh
#!/bin/sh
unset CARGO_TARGET_DIR

cargo new repro-76720
cd repro-76720
cargo add serde_derive@1.0.162

build() {
    worktree=$1
    commit=$2
    serde_version=$3
    toolchain=$4

    # We can't use RTIM because this only repros with `omit-git-hash`.
    (cd ../$worktree && git checkout $commit && x build std proc_macro)
    cargo update -p serde_derive --precise $serde_version
    cargo clean -p repro-76720 -p serde_derive
    cargo +$toolchain-stage1 build
}

build rust2 6874f4e3fc2a16be7c78e702d068bbc1daa90e16 1.0.162 r2
build rust3 999ac5f7770bff68bd65f490990d32c3ec1faaa6 1.0.163 r3
