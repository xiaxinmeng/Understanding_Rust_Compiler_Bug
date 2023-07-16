plain
curl: (22) The requested URL returned error: 404

error: failed to download llvm from ci

    help: old builds get deleted after a certain time
    help: if trying to compile an old commit of rustc, disable `download-ci-llvm` in config.toml:

    [llvm]
    download-ci-llvm = false
Build completed unsuccessfully in 0:00:00
