plain
2019-11-16T17:48:25.6699247Z ## edition-guide
2019-11-16T17:48:25.6699548Z ## nomicon
2019-11-16T17:48:25.6699719Z ## reference
2019-11-16T17:48:25.6700108Z ## rust-by-example
2019-11-16T17:48:25.6700315Z - Add VS Code user dir to .gitignore (rust-lang/cargo#7578)
2019-11-16T17:48:25.6700502Z - Add back support for `BROWSER` envvar in `cargo doc --open`. (rust-lang/cargo#7576)
2019-11-16T17:48:25.6700705Z - Add github action to replace Travis.yml (rust-lang-nursery/nomicon#172)
2019-11-16T17:48:25.6700908Z - Added aliases to subcommand typo suggestions. (rust-lang/cargo#7486)
2019-11-16T17:48:25.6701082Z - Audit code blocks. (rust-lang-nursery/reference#715)
2019-11-16T17:48:25.6701270Z - Change my-buddy to github-handle (rust-lang/cargo#7553)
2019-11-16T17:48:25.6701438Z - Don't panic when parsing `/proc/stat` (rust-lang/cargo#7580)
2019-11-16T17:48:25.6701692Z - Expand documentation on build scripts. (rust-lang/cargo#7565)
2019-11-16T17:48:25.6701898Z - Fix unused configuration key warning for a few keys under `build`. (rust-lang/cargo#7575)
2019-11-16T17:48:25.6702076Z - Migrate to GitHub Actions. (rust-lang-nursery/reference#705)
2019-11-16T17:48:25.6702265Z - No need for an iterator here to fetch values (rust-lang/book#1957)
2019-11-16T17:48:25.6702632Z - Only include "already existing ..." comment in gitignore on conflict (rust-lang/cargo#7570)
2019-11-16T17:48:25.6703007Z - Port from Travis to GitHub Actions (rust-lang-nursery/edition-guide#192)
2019-11-16T17:48:25.6703487Z - State that no_implicit_prelude also applies to nested modules (rust-lang-nursery/reference#707)
2019-11-16T17:48:25.6703710Z - Update coherence and orphan rules documentation to match RFC 2451 (rust-lang-nursery/reference#703)
2019-11-16T17:48:25.6707042Z - Update crossbeam-utils requirement from 0.6 to 0.7 (rust-lang/cargo#7566)
2019-11-16T17:48:25.6707466Z - Update verison to 0.42 (rust-lang/cargo#7568)
2019-11-16T17:48:25.6707545Z - Use multiple requirement syntax consistently (rust-lang/cargo#7573)
2019-11-16T17:48:25.6707642Z - don't download std-docs on CI (rust-lang/cargo#7513)
2019-11-16T17:48:25.6707718Z - expand Copy docs (rust-lang-nursery/reference#711)
---
2019-11-16T19:47:56.5825227Z    Compiling filetime v0.2.4
2019-11-16T19:47:56.6058242Z [RUSTC-TIMING] smallvec test:false 9.164
2019-11-16T19:47:56.6090606Z    Compiling thread_local v0.3.6
2019-11-16T19:48:00.3288943Z    Compiling crossbeam-utils v0.6.5
2019-11-16T19:48:01.0774473Z memory allocation of 2147483656 bytes failed[RUSTC-TIMING] hex test:false 7.820
2019-11-16T19:48:01.1021070Z error: could not compile `hex`.
2019-11-16T19:48:01.1021977Z Caused by:
2019-11-16T19:48:01.1021977Z Caused by:
2019-11-16T19:48:01.1022767Z   process didn't exit successfully: `D:\a\1\s\build\bootstrap/debug/rustc --crate-name hex C:\Users\VssAdministrator\.cargo\registry\src\github.com-1ecc6299db9ec823\hex-0.3.2\src\lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=2 -C debuginfo=0 -C metadata=489fb71c6d12f4d0 -C extra-filename=-489fb71c6d12f4d0 --out-dir D:\a\1\s\build\x86_64-pc-windows-msvc\stage1-tools\x86_64-pc-windows-msvc\release\deps --target x86_64-pc-windows-msvc -L dependency=D:\a\1\s\build\x86_64-pc-windows-msvc\stage1-tools\x86_64-pc-windows-msvc\release\deps -L dependency=D:\a\1\s\build\x86_64-pc-windows-msvc\stage1-tools\release\deps --cap-lints allow -Zexternal-macro-backtrace -Ctarget-feature=+crt-static -Zbinary-dep-depinfo` (exit code: 0xc0000409, STATUS_STACK_BUFFER_OVERRUN)
2019-11-16T19:48:01.2148301Z [RUSTC-TIMING] quick_error test:false 12.638
2019-11-16T19:48:01.2243370Z [RUSTC-TIMING] fnv test:false 12.593
2019-11-16T19:48:04.1903700Z [RUSTC-TIMING] adler32 test:false 12.591
2019-11-16T19:48:05.8636300Z [RUSTC-TIMING] utf8parse test:false 13.822
---
2019-11-16T19:48:07.4552589Z   local time: Sat Nov 16 19:48:07 CUT 2019
2019-11-16T19:48:07.9359312Z   network time: Sat, 16 Nov 2019 19:48:07 GMT
2019-11-16T19:48:07.9387930Z == end clock drift check ==
2019-11-16T19:48:08.0447203Z 
2019-11-16T19:48:08.5161625Z ##[error]Bash exited with code '1'.
2019-11-16T19:48:08.5787747Z ##[section]Starting: Checkout
2019-11-16T19:48:08.6853694Z ==============================================================================
2019-11-16T19:48:08.6854012Z Task         : Get sources
2019-11-16T19:48:08.6854314Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
