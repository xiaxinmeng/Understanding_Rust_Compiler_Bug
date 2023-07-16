plain
2020-03-01T20:25:28.2108978Z ========================== Starting Command Output ===========================
2020-03-01T20:25:28.2113207Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/f1f6f15d-ad62-40ec-8b07-f9a3a455108e.sh
2020-03-01T20:25:28.2114012Z 
2020-03-01T20:25:28.2118652Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-01T20:25:28.2139781Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69171/merge to s
2020-03-01T20:25:28.2143440Z Task         : Get sources
2020-03-01T20:25:28.2143801Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-01T20:25:28.2144099Z Version      : 1.0.0
2020-03-01T20:25:28.2144294Z Author       : Microsoft
---
2020-03-01T20:25:29.8600555Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-01T20:25:29.8608259Z ##[command]git config gc.auto 0
2020-03-01T20:25:29.8613994Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-01T20:25:29.8618344Z ##[command]git config --get-all http.proxy
2020-03-01T20:25:29.8626782Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69171/merge:refs/remotes/pull/69171/merge
---
2020-03-01T21:01:57.6075193Z    Compiling backtrace v0.3.44
2020-03-01T21:01:59.9123957Z    Compiling rls-data v0.19.0
2020-03-01T21:02:02.7145683Z    Compiling rand_chacha v0.2.1
2020-03-01T21:02:02.9598661Z    Compiling crossbeam-epoch v0.7.2
2020-03-01T21:02:04.4609946Z error: legacy asm! syntax is no longer supported
2020-03-01T21:02:04.4610784Z   --> /cargo/registry/src/github.com-1ecc6299db9ec823/parking_lot-0.9.0/src/elision.rs:70:13
2020-03-01T21:02:04.4611384Z    |
2020-03-01T21:02:04.4611970Z 70 |               asm!("xacquire; lock; cmpxchgq $2, $1"
2020-03-01T21:02:04.4613429Z    |               |
2020-03-01T21:02:04.4613429Z    |               |
2020-03-01T21:02:04.4614315Z    |  _____________help: replace with: `llvm_asm!`
2020-03-01T21:02:04.4614950Z    | |
2020-03-01T21:02:04.4615823Z 71 | |                  : "={rax}" (prev), "+*m" (self)
2020-03-01T21:02:04.4616906Z 72 | |                  : "r" (new), "{rax}" (current)
2020-03-01T21:02:04.4617739Z 73 | |                  : "memory"
2020-03-01T21:02:04.4618528Z 74 | |                  : "volatile");
2020-03-01T21:02:04.4623511Z 
2020-03-01T21:02:04.4623511Z 
2020-03-01T21:02:04.4632806Z error: legacy asm! syntax is no longer supported
2020-03-01T21:02:04.4633940Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/parking_lot-0.9.0/src/elision.rs:97:13
2020-03-01T21:02:04.4634510Z     |
2020-03-01T21:02:04.4635084Z 97  |               asm!("xrelease; lock; xaddq $2, $1"
2020-03-01T21:02:04.4636500Z     |               |
2020-03-01T21:02:04.4636500Z     |               |
2020-03-01T21:02:04.4637213Z     |  _____________help: replace with: `llvm_asm!`
2020-03-01T21:02:04.4637854Z     | |
2020-03-01T21:02:04.4638553Z 98  | |                  : "=r" (prev), "+*m" (self)
2020-03-01T21:02:04.4639423Z 99  | |                  : "0" (val.wrapping_neg())
2020-03-01T21:02:04.4640281Z 100 | |                  : "memory"
2020-03-01T21:02:04.4641246Z 101 | |                  : "volatile");
2020-03-01T21:02:04.4646341Z 
2020-03-01T21:02:04.6037384Z error: aborting due to 2 previous errors
2020-03-01T21:02:04.6041795Z 
2020-03-01T21:02:04.6114232Z error: could not compile `parking_lot`.
---
2020-03-01T21:02:42.1008218Z   local time: Sun Mar  1 21:02:42 UTC 2020
2020-03-01T21:02:42.3867392Z   network time: Sun, 01 Mar 2020 21:02:42 GMT
2020-03-01T21:02:42.3869998Z == end clock drift check ==
2020-03-01T21:02:43.0450005Z 
2020-03-01T21:02:43.0529484Z ##[error]Bash exited with code '1'.
2020-03-01T21:02:43.0546061Z ##[section]Finishing: Run build
2020-03-01T21:02:43.0599185Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69171/merge to s
2020-03-01T21:02:43.0604747Z Task         : Get sources
2020-03-01T21:02:43.0605111Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-01T21:02:43.0605461Z Version      : 1.0.0
2020-03-01T21:02:43.0605691Z Author       : Microsoft
2020-03-01T21:02:43.0605691Z Author       : Microsoft
2020-03-01T21:02:43.0606065Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-01T21:02:43.0606514Z ==============================================================================
2020-03-01T21:02:43.4737092Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-01T21:02:43.4789593Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69171/merge to s
2020-03-01T21:02:43.4904994Z Cleaning up task key
2020-03-01T21:02:43.4906464Z Start cleaning up orphan processes.
2020-03-01T21:02:43.5151756Z Terminate orphan process: pid (4238) (python)
2020-03-01T21:02:43.5443095Z ##[section]Finishing: Finalize Job
