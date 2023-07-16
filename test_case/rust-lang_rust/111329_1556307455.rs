
#!/bin/sh
unset CARGO_TARGET_DIR CARGO_REGISTRIES_CRATES_IO_PROTOCOL

cd $(dirname $0)
cargo new repro-76720
cd repro-76720
cargo add serde_derive@1.0.162
cargo clean

build_rustc() {
    worktree=$1
    commit=$2
    # We can't use RTIM because this only repros with `omit-git-hash`.
    (cd ../$worktree && git checkout $commit && x build std proc_macro)
}

build() {
    worktree=$1
    commit=$2
    serde_version=$3
    toolchain=$4

    build_rustc $worktree $commit
    cargo update -p serde_derive --precise $serde_version
    cargo +$toolchain-stage1 build -Zbinary-dep-depinfo
}

build rust2 6874f4e3fc2a16be7c78e702d068bbc1daa90e16 1.0.162 r2
build rust2 999ac5 1.0.163 r2
