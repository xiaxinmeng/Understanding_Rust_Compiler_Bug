
rm config.toml  # download-ci-llvm is on by default
x build --stage 0 rustc
build/host/stage1/bin/rustc --print=target-spec-json --target loongarch64-unknown-linux-gnu -Zunstable-options
