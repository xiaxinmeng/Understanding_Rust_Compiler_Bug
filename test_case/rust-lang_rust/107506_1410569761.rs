
downloading https://ci-artifacts.rust-lang.org/rustc-builds/c35035cefc709abddabfb28ecc6a326458d46ce2/rust-dev-nightly-x86_64-apple-darwin.tar.xz
curl: (22) The requested URL returned error: 404                                                                                                                                                          

error: failed to download llvm from ci

    help: old builds get deleted after a certain time
    help: if trying to compile an old commit of rustc, disable `download-ci-llvm` in config.toml:

    [llvm]
    download-ci-llvm = false
