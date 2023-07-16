plain
Updating files:  98% (32856/33526)
Updating files:  99% (33191/33526)
Updating files: 100% (33526/33526)
Updating files: 100% (33526/33526), done.
branch 'try' set up to track 'origin/try'.
Switched to a new branch 'try'
[command]"C:\Program Files\Git\bin\git.exe" log -1 --format='%H'
'e013e5e066692f32e251c6ed1df96dacf69b6a71'
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
file:.git/config submodule.src/doc/book.active=true
---
* highest error code: E0788
Found 505 error codes
Found 0 error(s) in error codes
Done!
tidy error: D:\a\rust\rust\src\ci\scripts\install-mingw.sh:67: line longer than 100 chars
some tidy checks failed
Build completed unsuccessfully in 0:01:41
Build completed unsuccessfully in 0:01:41
make: *** [Makefile:78: ci-mingw-subset-1] Error 1
