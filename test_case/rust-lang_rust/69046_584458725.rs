plain
2020-02-11T02:48:19.4355003Z ========================== Starting Command Output ===========================
2020-02-11T02:48:19.4356438Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/54a98abc-9a8e-4cca-96bb-55240f1edc03.sh
2020-02-11T02:48:19.4356477Z 
2020-02-11T02:48:19.4359168Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-11T02:48:19.4365297Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69046/merge to s
2020-02-11T02:48:19.4366920Z Task         : Get sources
2020-02-11T02:48:19.4366995Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-11T02:48:19.4367027Z Version      : 1.0.0
2020-02-11T02:48:19.4367060Z Author       : Microsoft
---
2020-02-11T02:48:20.3133605Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-11T02:48:20.3222743Z ##[command]git config gc.auto 0
2020-02-11T02:48:20.3303886Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-11T02:48:20.3384219Z ##[command]git config --get-all http.proxy
2020-02-11T02:48:20.3592210Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69046/merge:refs/remotes/pull/69046/merge
---
2020-02-11T02:53:21.2232293Z     Checking alloc v0.0.0 (/checkout/src/liballoc)
2020-02-11T02:53:21.4963495Z error[E0658]: raw address of syntax is experimental
2020-02-11T02:53:21.4963841Z    --> src/liballoc/rc.rs:932:20
2020-02-11T02:53:21.4964444Z     |
2020-02-11T02:53:21.4964951Z 932 |         ptr::write(&raw mut (*inner).strong, Cell::new(1));
2020-02-11T02:53:21.4966067Z     |
2020-02-11T02:53:21.4966067Z     |
2020-02-11T02:53:21.4966547Z     = note: for more information, see ***/issues/64490
2020-02-11T02:53:21.4966870Z     = help: add `#![feature(raw_ref_op)]` to the crate attributes to enable
2020-02-11T02:53:21.4997143Z error[E0658]: raw address of syntax is experimental
2020-02-11T02:53:21.4997454Z    --> src/liballoc/rc.rs:933:20
2020-02-11T02:53:21.4997704Z     |
2020-02-11T02:53:21.4997704Z     |
2020-02-11T02:53:21.4998167Z 933 |         ptr::write(&raw mut (*inner).weak, Cell::new(1));
2020-02-11T02:53:21.4999434Z     |
2020-02-11T02:53:21.4999434Z     |
2020-02-11T02:53:21.4999898Z     = note: for more information, see ***/issues/64490
2020-02-11T02:53:21.5000258Z     = help: add `#![feature(raw_ref_op)]` to the crate attributes to enable
2020-02-11T02:53:21.5029380Z error[E0658]: raw address of syntax is experimental
2020-02-11T02:53:21.5029772Z    --> src/liballoc/sync.rs:793:20
2020-02-11T02:53:21.5030042Z     |
2020-02-11T02:53:21.5030042Z     |
2020-02-11T02:53:21.5030378Z 793 |         ptr::write(&raw mut (*inner).strong, atomic::AtomicUsize::new(1));
2020-02-11T02:53:21.5030938Z     |
2020-02-11T02:53:21.5030938Z     |
2020-02-11T02:53:21.5031331Z     = note: for more information, see ***/issues/64490
2020-02-11T02:53:21.5031692Z     = help: add `#![feature(raw_ref_op)]` to the crate attributes to enable
2020-02-11T02:53:21.5104364Z error[E0658]: raw address of syntax is experimental
2020-02-11T02:53:21.5104871Z    --> src/liballoc/sync.rs:794:20
2020-02-11T02:53:21.5105088Z     |
2020-02-11T02:53:21.5105088Z     |
2020-02-11T02:53:21.5105578Z 794 |         ptr::write(&raw mut (*inner).weak, atomic::AtomicUsize::new(1));
2020-02-11T02:53:21.5106100Z     |
2020-02-11T02:53:21.5106100Z     |
2020-02-11T02:53:21.5106510Z     = note: for more information, see ***/issues/64490
2020-02-11T02:53:21.5106855Z     = help: add `#![feature(raw_ref_op)]` to the crate attributes to enable
2020-02-11T02:53:22.4537600Z     Checking cfg-if v0.1.10
2020-02-11T02:53:22.4944148Z     Checking rustc-demangle v0.1.16
2020-02-11T02:53:22.7370317Z error: aborting due to 4 previous errors
2020-02-11T02:53:22.7370478Z 
---
2020-02-11T02:53:22.8203633Z   local time: Tue Feb 11 02:53:22 UTC 2020
2020-02-11T02:53:22.9684618Z   network time: Tue, 11 Feb 2020 02:53:22 GMT
2020-02-11T02:53:22.9690001Z == end clock drift check ==
2020-02-11T02:53:23.9210598Z 
2020-02-11T02:53:23.9325588Z ##[error]Bash exited with code '1'.
2020-02-11T02:53:23.9351817Z ##[section]Finishing: Run build
2020-02-11T02:53:23.9367289Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69046/merge to s
2020-02-11T02:53:23.9369551Z Task         : Get sources
2020-02-11T02:53:23.9369599Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-11T02:53:23.9369647Z Version      : 1.0.0
2020-02-11T02:53:23.9369712Z Author       : Microsoft
2020-02-11T02:53:23.9369712Z Author       : Microsoft
2020-02-11T02:53:23.9369760Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-11T02:53:23.9369812Z ==============================================================================
2020-02-11T02:53:24.3781475Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-11T02:53:24.3816668Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69046/merge to s
2020-02-11T02:53:24.3951892Z Cleaning up task key
2020-02-11T02:53:24.3952855Z Start cleaning up orphan processes.
2020-02-11T02:53:24.4110128Z Terminate orphan process: pid (3386) (python)
2020-02-11T02:53:24.4323783Z ##[section]Finishing: Finalize Job
