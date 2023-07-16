plain
##[endgroup]
##[group]Determining the checkout info
##[endgroup]
##[group]Checking out the ref
[command]"C:\Program Files\Git\bin\git.exe" checkout --progress --force -B try refs/remotes/origin/try
Updating files:  19% (5024/26440)
Updating files:  20% (5288/26440)
Updating files:  21% (5553/26440)
Updating files:  22% (5817/26440)
---
Updating files:  98% (25912/26440)
Updating files:  99% (26176/26440)
Updating files: 100% (26440/26440)
Updating files: 100% (26440/26440), done.
Branch 'try' set up to track remote branch 'try' from 'origin'.
Switched to a new branch 'try'
[command]"C:\Program Files\Git\bin\git.exe" log -1 --format='%H'
'aacc1f85950823321d1f8216b93a2236c6e11e61'
##[group]Run src/ci/scripts/setup-environment.sh
src/ci/scripts/setup-environment.sh
---
file:.git/config remote.origin.url=https://github.com/rust-lang-ci/rust
file:.git/config remote.origin.fetch=+refs/heads/*:refs/remotes/origin/*
file:.git/config gc.auto=0
file:.git/config http.https://github.com/.extraheader=AUTHORIZATION: basic ***
file:.git/config branch.try.remote=origin
file:.git/config branch.try.merge=refs/heads/try
file:.git/config submodule.library/backtrace.url=https://github.com/rust-lang/backtrace-rs.git
file:.git/config submodule.library/stdarch.active=true
file:.git/config submodule.library/stdarch.url=https://github.com/rust-lang/stdarch.git
file:.git/config submodule.src/doc/edition-guide.active=true
---
SwapTotal:       4718592 kB
SwapFree:        4718592 kB
+ python x.py dist
    Finished dev [unoptimized + debuginfo] target(s) in 0.27s
thread 'main' panicked at 'std::fs::write(dst_libdir.join("link-type.txt"), link_type) failed with The system cannot find the path specified. (os error 3)', src\bootstrap\dist.rs:1982:9
failed to run: D:\a\rust\rust\build\bootstrap\debug\bootstrap dist
Build completed unsuccessfully in 0:00:01
---
Entering 'src/tools/rustfmt'
Cleaning up orphan processes
Terminate orphan process: pid (5644) (python)
Terminate orphan process: pid (3136) (sccache)
Terminate orphan process: pid (3664) (vctip)
