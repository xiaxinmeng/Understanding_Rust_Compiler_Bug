text
2019-12-09T17:17:32.0473549Z `noprelude` is to be used by Cargo's build-std feature in order to use `--extern` to reference standard library crates.
2019-12-09T17:17:32.0473698Z 
2019-12-09T17:17:32.0473769Z disk usage:
2019-12-09T17:17:32.1132644Z Filesystem            Size  Used Avail Use% Mounted on
2019-12-09T17:17:32.1143754Z C:/Program Files/Git  256G  141G  116G  55% /
---
2019-12-09T19:41:15.0714238Z test workspaces::ws_warn_unused ... ok
2019-12-09T19:41:15.0716645Z 
2019-12-09T19:41:15.0717566Z failures:
2019-12-09T19:41:15.0718277Z 
2019-12-09T19:41:15.0718650Z ---- registry::bad_cksum stdout ----
2019-12-09T19:41:15.0719034Z running `D:\a\1\s\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\cargo.exe build -v`
2019-12-09T19:41:15.0719394Z thread 'registry::bad_cksum' panicked at '
2019-12-09T19:41:15.0719731Z Expected: execs
2019-12-09T19:41:15.0720041Z     but: differences:
2019-12-09T19:41:15.0720373Z   1 - |[DOWNLOADING] crates ...|
2019-12-09T19:41:15.0720715Z     + |error: process didn't exit successfully: `rustc -vV` (exit code: 0xc0000005, STATUS_ACCESS_VIOLATION)|
2019-12-09T19:41:15.0721032Z 
2019-12-09T19:41:15.0721494Z   2 - |[DOWNLOADED] bad-cksum [..]|
2019-12-09T19:41:15.0722517Z 
2019-12-09T19:41:15.0722517Z 
2019-12-09T19:41:15.0722701Z   3 - |[ERROR] failed to download replaced source registry `https://[..]`|
2019-12-09T19:41:15.0722832Z 
2019-12-09T19:41:15.0722883Z   4 - ||
2019-12-09T19:41:15.0722956Z     +
2019-12-09T19:41:15.0722986Z 
2019-12-09T19:41:15.0722986Z 
2019-12-09T19:41:15.0723060Z   5 - |Caused by:|
2019-12-09T19:41:15.0723186Z 
2019-12-09T19:41:15.0723186Z 
2019-12-09T19:41:15.0723284Z   6 - |  failed to verify the checksum of `bad-cksum v0.0.1 (registry `D:/a/1/s/build/x86_64-pc-windows-msvc/stage2-tools/x86_64-pc-windows-msvc/cit/t1236[..]`)`|
2019-12-09T19:41:15.0723435Z 
2019-12-09T19:41:15.0723465Z 
2019-12-09T19:41:15.0723537Z other output:
2019-12-09T19:41:15.0723628Z ``', src\tools\cargo\crates\cargo-test-support\src\lib.rs:849:13
---
2019-12-09T19:41:15.3352615Z   local time: Mon Dec  9 19:41:15 CUT 2019
2019-12-09T19:41:15.7086160Z   network time: Mon, 09 Dec 2019 19:41:15 GMT
2019-12-09T19:41:15.7104106Z == end clock drift check ==
2019-12-09T19:41:15.7944607Z 
2019-12-09T19:41:16.1571651Z ##[error]Bash exited with code '1'.
2019-12-09T19:41:16.2288237Z ##[section]Starting: Checkout
2019-12-09T19:41:16.2866552Z ==============================================================================
2019-12-09T19:41:16.2866680Z Task         : Get sources
2019-12-09T19:41:16.2866766Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
