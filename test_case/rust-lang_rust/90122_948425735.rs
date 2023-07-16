plain
info: removing cargo home
info: removing rustup binaries
info: rustup is uninstalled
Attempting to download https://ci-caches.rust-lang.org/docker/3fb94d85a74064d8a33969492b4fad5619a1ce4f9ef2b4c7776dd3c4255dbb95185e47a56b22ab9a6fddd34c13487353888c4a8f5889f8df54f73c2235202b9c
Attempting with retry: curl --max-time 600 -y 30 -Y 10 --connect-timeout 30 -f -L -C - -o /tmp/rustci_docker_cache https://ci-caches.rust-lang.org/docker/3fb94d85a74064d8a33969492b4fad5619a1ce4f9ef2b4c7776dd3c4255dbb95185e47a56b22ab9a6fddd34c13487353888c4a8f5889f8df54f73c2235202b9c
                                 Dload  Upload   Total   Spent    Left  Speed

  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  3  410M    3 12.6M    0     0  75.9M      0  0:00:05 --:--:--  0:00:05 75.4M
---
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
tidy: Skipping binary file check, read-only filesystem
Checking which error codes lack tests...
* 627 error codes
* highest error code: E0785
tidy error: /checkout/src/ci/docker/run.sh:73: line longer than 100 chars
Found 0 error codes with no tests
Done!
* 353 features
some tidy checks failed
some tidy checks failed


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rust-tidy" "/checkout" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build" "16"


Build completed unsuccessfully in 0:00:13
