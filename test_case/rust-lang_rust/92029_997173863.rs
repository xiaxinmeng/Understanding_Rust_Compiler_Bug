plain
Updating files:  98% (30997/31629)
Updating files:  99% (31313/31629)
Updating files: 100% (31629/31629)
Updating files: 100% (31629/31629), done.
Switched to a new branch 'try'
Branch 'try' set up to track remote branch 'try' from 'origin'.
[command]/usr/local/bin/git log -1 --format='%H'
'3f2645cb3ad797a0fa23bed459b6be1d6daeb5b5'
##[group]Run src/ci/scripts/setup-environment.sh
src/ci/scripts/setup-environment.sh
---
   Compiling toml v0.5.7
    Finished dev [unoptimized] target(s) in 2m 34s

Build completed unsuccessfully in 0:02:59
Couldn't find required command: ninja (or ninja-build)

You should install ninja as described at
<https://github.com/ninja-build/ninja/wiki/Pre-built-Ninja-packages>,
or set `ninja = false` in the `[llvm]` section of `config.toml`.
Alternatively, set `download-ci-llvm = true` in that `[llvm]` section
to download LLVM rather than building it.
make: *** [prepare] Error 1
Command failed. Attempt 2/5:
    Finished dev [unoptimized] target(s) in 0.33s


Build completed unsuccessfully in 0:00:00
Couldn't find required command: ninja (or ninja-build)

You should install ninja as described at
<https://github.com/ninja-build/ninja/wiki/Pre-built-Ninja-packages>,
or set `ninja = false` in the `[llvm]` section of `config.toml`.
Alternatively, set `download-ci-llvm = true` in that `[llvm]` section
to download LLVM rather than building it.
make: *** [prepare] Error 1
Command failed. Attempt 3/5:
    Finished dev [unoptimized] target(s) in 0.34s


Build completed unsuccessfully in 0:00:00
Couldn't find required command: ninja (or ninja-build)

You should install ninja as described at
<https://github.com/ninja-build/ninja/wiki/Pre-built-Ninja-packages>,
or set `ninja = false` in the `[llvm]` section of `config.toml`.
Alternatively, set `download-ci-llvm = true` in that `[llvm]` section
to download LLVM rather than building it.
make: *** [prepare] Error 1
Command failed. Attempt 4/5:
    Finished dev [unoptimized] target(s) in 0.33s


Build completed unsuccessfully in 0:00:00
Couldn't find required command: ninja (or ninja-build)

You should install ninja as described at
<https://github.com/ninja-build/ninja/wiki/Pre-built-Ninja-packages>,
or set `ninja = false` in the `[llvm]` section of `config.toml`.
Alternatively, set `download-ci-llvm = true` in that `[llvm]` section
to download LLVM rather than building it.
make: *** [prepare] Error 1
Command failed. Attempt 5/5:
    Finished dev [unoptimized] target(s) in 0.43s


Build completed unsuccessfully in 0:00:01
Couldn't find required command: ninja (or ninja-build)

---
or set `ninja = false` in the `[llvm]` section of `config.toml`.
  network time: Sat, 18 Dec 2021 09:09:42 GMT
Alternatively, set `download-ci-llvm = true` in that `[llvm]` section
== end clock drift check ==
to download LLVM rather than building it.
make: *** [prepare] Error 1
##[error]Process completed with exit code 1.
Post job cleanup.
[command]/usr/local/bin/git version
