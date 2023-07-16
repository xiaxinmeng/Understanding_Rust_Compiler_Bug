plain
2020-03-17T09:57:08.0864669Z ========================== Starting Command Output ===========================
2020-03-17T09:57:08.0869378Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/fba1a2a3-76fb-421f-b7bd-0e18c5ff2f2b.sh
2020-03-17T09:57:08.0869828Z 
2020-03-17T09:57:08.0874574Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-17T09:57:08.0893461Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69171/merge to s
2020-03-17T09:57:08.0896696Z Task         : Get sources
2020-03-17T09:57:08.0896989Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-17T09:57:08.0897291Z Version      : 1.0.0
2020-03-17T09:57:08.0897483Z Author       : Microsoft
---
2020-03-17T09:57:09.7157435Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-17T09:57:09.7163962Z ##[command]git config gc.auto 0
2020-03-17T09:57:09.7167782Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-17T09:57:09.7171967Z ##[command]git config --get-all http.proxy
2020-03-17T09:57:09.7179444Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69171/merge:refs/remotes/pull/69171/merge
---
2020-03-17T10:30:37.3538448Z    Compiling backtrace v0.3.44
2020-03-17T10:30:40.0415306Z    Compiling rls-data v0.19.0
2020-03-17T10:30:43.0333009Z    Compiling rand_chacha v0.2.1
2020-03-17T10:30:43.2939544Z    Compiling crossbeam-epoch v0.7.2
2020-03-17T10:30:44.8174803Z error: legacy asm! syntax is no longer supported
2020-03-17T10:30:44.8175668Z   --> /cargo/registry/src/github.com-1ecc6299db9ec823/parking_lot-0.9.0/src/elision.rs:70:13
2020-03-17T10:30:44.8176311Z    |
2020-03-17T10:30:44.8176991Z 70 |               asm!("xacquire; lock; cmpxchgq $2, $1"
2020-03-17T10:30:44.8178397Z    |               |
2020-03-17T10:30:44.8178397Z    |               |
2020-03-17T10:30:44.8179191Z    |  _____________help: replace with: `llvm_asm!`
2020-03-17T10:30:44.8180156Z    | |
2020-03-17T10:30:44.8180972Z 71 | |                  : "={rax}" (prev), "+*m" (self)
2020-03-17T10:30:44.8181945Z 72 | |                  : "r" (new), "{rax}" (current)
2020-03-17T10:30:44.8183012Z 73 | |                  : "memory"
2020-03-17T10:30:44.8183968Z 74 | |                  : "volatile");
2020-03-17T10:30:44.8184979Z 
2020-03-17T10:30:44.8184979Z 
2020-03-17T10:30:44.8190380Z error: legacy asm! syntax is no longer supported
2020-03-17T10:30:44.8191188Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/parking_lot-0.9.0/src/elision.rs:97:13
2020-03-17T10:30:44.8191795Z     |
2020-03-17T10:30:44.8192436Z 97  |               asm!("xrelease; lock; xaddq $2, $1"
2020-03-17T10:30:44.8193841Z     |               |
2020-03-17T10:30:44.8193841Z     |               |
2020-03-17T10:30:44.8194635Z     |  _____________help: replace with: `llvm_asm!`
2020-03-17T10:30:44.8195325Z     | |
2020-03-17T10:30:44.8196120Z 98  | |                  : "=r" (prev), "+*m" (self)
2020-03-17T10:30:44.8197095Z 99  | |                  : "0" (val.wrapping_neg())
2020-03-17T10:30:44.8198034Z 100 | |                  : "memory"
2020-03-17T10:30:44.8198944Z 101 | |                  : "volatile");
2020-03-17T10:30:44.8199980Z 
2020-03-17T10:30:44.9822261Z error: aborting due to 2 previous errors
2020-03-17T10:30:44.9822615Z 
2020-03-17T10:30:44.9881054Z error: could not compile `parking_lot`.
---
2020-03-17T10:31:20.7407322Z   local time: Tue Mar 17 10:31:20 UTC 2020
2020-03-17T10:31:20.9101392Z   network time: Tue, 17 Mar 2020 10:31:20 GMT
2020-03-17T10:31:20.9102410Z == end clock drift check ==
2020-03-17T10:31:21.4953642Z 
2020-03-17T10:31:21.5038167Z ##[error]Bash exited with code '1'.
2020-03-17T10:31:21.5054401Z ##[section]Finishing: Run build
2020-03-17T10:31:21.5108530Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69171/merge to s
2020-03-17T10:31:21.5114136Z Task         : Get sources
2020-03-17T10:31:21.5114479Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-17T10:31:21.5114810Z Version      : 1.0.0
2020-03-17T10:31:21.5115035Z Author       : Microsoft
2020-03-17T10:31:21.5115035Z Author       : Microsoft
2020-03-17T10:31:21.5115386Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-17T10:31:21.5115809Z ==============================================================================
2020-03-17T10:31:21.8693752Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-17T10:31:21.8740777Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69171/merge to s
2020-03-17T10:31:21.8830622Z Cleaning up task key
2020-03-17T10:31:21.8832084Z Start cleaning up orphan processes.
2020-03-17T10:31:21.9029676Z Terminate orphan process: pid (4035) (python)
2020-03-17T10:31:21.9205096Z ##[section]Finishing: Finalize Job
