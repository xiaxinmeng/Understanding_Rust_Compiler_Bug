plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:697bea7ddceb6696743da8f159f268aef8bfb3c6)
Complete job name: PR (x86_64-gnu-tools, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
   Compiling ignore v0.4.18
   Compiling xz2 v0.1.6
   Compiling toml v0.5.9
    Finished dev [unoptimized] target(s) in 18.97s
thread 'main' panicked at 'std::fs::read_to_string(&path) failed with No such file or directory (os error 2)', bin/main.rs:21:19
Build completed unsuccessfully in 0:00:38
make: *** [Makefile:58: prepare] Error 1
Command failed. Attempt 2/5:
Building bootstrap
Building bootstrap
    Finished dev [unoptimized] target(s) in 0.04s
thread 'main' panicked at 'std::fs::read_to_string(&path) failed with No such file or directory (os error 2)', bin/main.rs:21:19
Build completed unsuccessfully in 0:00:00
make: *** [Makefile:58: prepare] Error 1
Command failed. Attempt 3/5:
Building bootstrap
Building bootstrap
    Finished dev [unoptimized] target(s) in 0.04s
thread 'main' panicked at 'std::fs::read_to_string(&path) failed with No such file or directory (os error 2)', bin/main.rs:21:19
Build completed unsuccessfully in 0:00:00
make: *** [Makefile:58: prepare] Error 1
Command failed. Attempt 4/5:
Building bootstrap
Building bootstrap
    Finished dev [unoptimized] target(s) in 0.04s
thread 'main' panicked at 'std::fs::read_to_string(&path) failed with No such file or directory (os error 2)', bin/main.rs:21:19
Build completed unsuccessfully in 0:00:00
make: *** [Makefile:58: prepare] Error 1
Command failed. Attempt 5/5:
Building bootstrap
Building bootstrap
    Finished dev [unoptimized] target(s) in 0.04s
thread 'main' panicked at 'std::fs::read_to_string(&path) failed with No such file or directory (os error 2)', bin/main.rs:21:19
Build completed unsuccessfully in 0:00:00
make: *** [Makefile:58: prepare] Error 1
The command has failed after 5 attempts.
