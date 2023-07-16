
downloading https://ci-artifacts.rust-lang.org/rustc-builds/8ce3204af9463db3192ea1eb31c45c2f6d4b5ae6/rust-dev-nightly-x86_64-unknown-linux-gnu.tar.xz
curl: (22) The requested URL returned error: 404                                                                                

error: failed to download llvm from ci

help: old builds get deleted after a certain time
help: if trying to compile an old commit of rustc, disable `download-ci-llvm` in config.toml:

[llvm]
download-ci-llvm = false
