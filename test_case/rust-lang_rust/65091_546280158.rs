plain
2019-10-25T08:10:16.1731451Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-25T08:10:16.9643138Z ##[command]git config gc.auto 0
2019-10-25T08:10:16.9650162Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-25T08:10:16.9651711Z ##[command]git config --get-all http.proxy
2019-10-25T08:10:16.9655063Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65091/merge:refs/remotes/pull/65091/merge
---
2019-10-25T09:09:45.8871760Z .................................................................................................... 1600/9249
2019-10-25T09:09:51.0402240Z .................................................................................................... 1700/9249
2019-10-25T09:10:03.0844713Z ........................................................i...............i........................... 1800/9249
2019-10-25T09:10:10.7645635Z .................................................................................................... 1900/9249
2019-10-25T09:10:25.0160627Z ..............................................iiiii................................................. 2000/9249
2019-10-25T09:10:35.8204967Z .................................................................................................... 2200/9249
2019-10-25T09:10:38.3833645Z .................................................................................................... 2300/9249
2019-10-25T09:10:42.3141451Z .................................................................................................... 2400/9249
2019-10-25T09:11:05.0851973Z .................................................................................................... 2500/9249
---
2019-10-25T09:13:55.1331001Z ..................................................i...............i................................. 4800/9249
2019-10-25T09:14:03.6339141Z .................................................................................................... 4900/9249
2019-10-25T09:14:12.2413828Z .................................................................................................... 5000/9249
2019-10-25T09:14:18.3565486Z .................................................................................................... 5100/9249
2019-10-25T09:14:28.5838722Z ...................................................ii.ii............................................ 5200/9249
2019-10-25T09:14:38.6523731Z .................................................................................................... 5400/9249
2019-10-25T09:14:47.6560966Z .................................................................................................... 5500/9249
2019-10-25T09:14:55.2389443Z ..................i................................................................................. 5600/9249
2019-10-25T09:15:00.6902789Z .................................................................................................... 5700/9249
2019-10-25T09:15:00.6902789Z .................................................................................................... 5700/9249
2019-10-25T09:15:12.6515700Z .................................................................................................... 5800/9249
2019-10-25T09:15:23.9352187Z ...............ii...i..ii...........i............................................................... 5900/9249
2019-10-25T09:15:45.4953002Z .................................................................................................... 6100/9249
2019-10-25T09:15:53.9015795Z .................................................................................................... 6200/9249
2019-10-25T09:15:53.9015795Z .................................................................................................... 6200/9249
2019-10-25T09:16:06.3491447Z ......................................i..ii......................................................... 6300/9249
2019-10-25T09:16:28.7581298Z .................................................................................................... 6500/9249
2019-10-25T09:16:31.0243458Z ....i............................................................................................... 6600/9249
2019-10-25T09:16:33.3488150Z ...............................................................................i.................... 6700/9249
2019-10-25T09:16:35.9903008Z .................................................................................................... 6800/9249
---
2019-10-25T09:21:03.8264001Z  finished in 5.797
2019-10-25T09:21:03.8463095Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-25T09:21:04.0147986Z 
2019-10-25T09:21:04.0148964Z running 153 tests
2019-10-25T09:21:07.1949917Z i....iii......iii..iiii...i.............................i..i..................i....i...........ii.i. 100/153
2019-10-25T09:21:09.1651045Z i..iiii..............i.........iii.i.........ii......
2019-10-25T09:21:09.1652884Z 
2019-10-25T09:21:09.1657157Z  finished in 5.319
2019-10-25T09:21:09.1842865Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-25T09:21:09.3355735Z 
---
2019-10-25T09:21:12.1982723Z  finished in 2.118
2019-10-25T09:21:12.1983078Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-25T09:21:12.1983294Z 
2019-10-25T09:21:12.1983362Z running 9 tests
2019-10-25T09:21:12.1983664Z iiiiiiiii
2019-10-25T09:21:12.1983976Z 
2019-10-25T09:21:12.1988255Z  finished in 0.168
2019-10-25T09:21:12.1988736Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-25T09:21:12.1988777Z 
---
2019-10-25T09:21:30.1245915Z  finished in 18.610
2019-10-25T09:21:30.1449189Z Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-25T09:21:31.1903919Z 
2019-10-25T09:21:31.1904913Z running 123 tests
2019-10-25T09:21:53.2703163Z .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....ii..........iiii..........i...ii...i.......ii. 100/123
2019-10-25T09:21:57.6819566Z i.i.i......iii.i.....ii
2019-10-25T09:21:57.6821886Z 
2019-10-25T09:21:57.6822187Z  finished in 27.537
2019-10-25T09:21:57.6830809Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-25T09:21:57.6831476Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-10-25T09:31:07.4466178Z    Compiling alloc v0.0.0 (/checkout/src/liballoc)
2019-10-25T09:31:08.3752168Z error: unused imports: `AssertUnwindSafe`, `self`
2019-10-25T09:31:08.3752754Z  --> src/liballoc/../liballoc/tests/binary_heap.rs:3:18
2019-10-25T09:31:08.3753044Z   |
2019-10-25T09:31:08.3756758Z 3 | use std::panic::{self, AssertUnwindSafe};
2019-10-25T09:31:08.3757050Z   |                  ^^^^  ^^^^^^^^^^^^^^^^
2019-10-25T09:31:08.3757500Z   = note: `-D unused-imports` implied by `-D warnings`
2019-10-25T09:31:08.3757532Z 
2019-10-25T09:31:08.3757532Z 
2019-10-25T09:31:08.3772014Z error: unused imports: `AtomicUsize`, `Ordering`
2019-10-25T09:31:08.3776232Z   |
2019-10-25T09:31:08.3776517Z 4 | use std::sync::atomic::{AtomicUsize, Ordering};
2019-10-25T09:31:08.3776987Z   |                         ^^^^^^^^^^^  ^^^^^^^^
2019-10-25T09:31:08.3777047Z 
2019-10-25T09:31:08.3777047Z 
2019-10-25T09:31:08.3777597Z error: unused import: `thread_rng`
2019-10-25T09:31:08.3805243Z   |
2019-10-25T09:31:08.3805243Z   |
2019-10-25T09:31:08.3805489Z 7 | use rand::{thread_rng, seq::SliceRandom};
2019-10-25T09:31:08.3805791Z 
2019-10-25T09:31:08.3805791Z 
2019-10-25T09:31:13.2523770Z error: unused import: `seq::SliceRandom`
2019-10-25T09:31:13.2525188Z   |
2019-10-25T09:31:13.2525188Z   |
2019-10-25T09:31:13.2525493Z 7 | use rand::{thread_rng, seq::SliceRandom};
2019-10-25T09:31:13.2525861Z 
2019-10-25T09:31:13.2542745Z error: aborting due to 4 previous errors
2019-10-25T09:31:13.2542827Z 
2019-10-25T09:31:13.3300534Z error: could not compile `alloc`.
---
2019-10-25T09:31:17.9554682Z   local time: Fri Oct 25 09:31:17 UTC 2019
2019-10-25T09:31:18.1052667Z   network time: Fri, 25 Oct 2019 09:31:18 GMT
2019-10-25T09:31:18.1057804Z == end clock drift check ==
2019-10-25T09:31:19.7431138Z 
2019-10-25T09:31:19.7536703Z ##[error]Bash exited with code '1'.
2019-10-25T09:31:19.7575782Z ##[section]Starting: Checkout
2019-10-25T09:31:19.7577646Z ==============================================================================
2019-10-25T09:31:19.7577703Z Task         : Get sources
2019-10-25T09:31:19.7577751Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
