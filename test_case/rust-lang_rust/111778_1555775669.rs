plain
curl: (35) OpenSSL SSL_connect: Connection reset by peer in connection to ci-artifacts.rust-lang.org:443 

error: failed to download llvm from ci

    help: old builds get deleted after a certain time
    help: if trying to compile an old commit of rustc, disable `download-ci-llvm` in config.toml:

    [llvm]
    download-ci-llvm = false
Build completed unsuccessfully in 0:00:00
cat: /tmp/toolstate/toolstates.json: No such file or directory
