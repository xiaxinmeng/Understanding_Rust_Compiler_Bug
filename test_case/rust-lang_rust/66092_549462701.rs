plain
2019-11-04T17:23:58.7372421Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-04T17:23:58.7581797Z ##[command]git config gc.auto 0
2019-11-04T17:23:58.7659002Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-04T17:23:58.7726173Z ##[command]git config --get-all http.proxy
2019-11-04T17:23:58.7881493Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66092/merge:refs/remotes/pull/66092/merge
---
2019-11-04T17:31:07.1728770Z    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2019-11-04T17:31:09.1484026Z error: multiple `cfg` predicates are specified
2019-11-04T17:31:09.1493089Z    --> src/libstd/sys/unix/rand.rs:146:30
2019-11-04T17:31:09.1493563Z     |
2019-11-04T17:31:09.1493853Z 146 | #[cfg(target_os = "freebsd", target_os = "netbsd")]
2019-11-04T17:31:09.1494211Z 
2019-11-04T17:31:09.1494646Z error[E0428]: the name `imp` is defined multiple times
2019-11-04T17:31:09.1494928Z    --> src/libstd/sys/unix/rand.rs:147:1
2019-11-04T17:31:09.1496078Z     |
2019-11-04T17:31:09.1496078Z     |
2019-11-04T17:31:09.1496437Z 21  | mod imp {
2019-11-04T17:31:09.1496801Z     | ------- previous definition of the module `imp` here
2019-11-04T17:31:09.1497015Z ...
2019-11-04T17:31:09.1497415Z 147 | mod imp {
2019-11-04T17:31:09.1497734Z     | ^^^^^^^ `imp` redefined here
2019-11-04T17:31:09.1498258Z     = note: `imp` must be defined only once in the type namespace of this module
2019-11-04T17:31:09.1498320Z 
2019-11-04T17:31:09.1498320Z 
2019-11-04T17:31:09.1498583Z error[E0425]: cannot find value `CTL_KERN` in crate `libc`
2019-11-04T17:31:09.1499043Z    --> src/libstd/sys/unix/rand.rs:151:26
2019-11-04T17:31:09.1499275Z     |
2019-11-04T17:31:09.1499559Z 151 |         let mib = [libc::CTL_KERN, libc::KERN_ARND];
2019-11-04T17:31:09.1499877Z     |                          ^^^^^^^^ not found in `libc`
2019-11-04T17:31:09.1499951Z 
2019-11-04T17:31:09.1500215Z error[E0425]: cannot find value `KERN_ARND` in crate `libc`
2019-11-04T17:31:09.1500466Z    --> src/libstd/sys/unix/rand.rs:151:42
2019-11-04T17:31:09.1500690Z     |
2019-11-04T17:31:09.1500971Z 151 |         let mib = [libc::CTL_KERN, libc::KERN_ARND];
2019-11-04T17:31:09.1501313Z     |                                          ^^^^^^^^^ not found in `libc`
2019-11-04T17:31:11.2315310Z error[E0308]: mismatched types
2019-11-04T17:31:11.2316639Z    --> src/libstd/sys/unix/rand.rs:158:30
2019-11-04T17:31:11.2317149Z     |
2019-11-04T17:31:11.2317607Z 158 | ...                   ptr::null(), 0)
2019-11-04T17:31:11.2317607Z 158 | ...                   ptr::null(), 0)
2019-11-04T17:31:11.2318195Z     |                       ^^^^^^^^^^^ types differ in mutability
2019-11-04T17:31:11.2318633Z     |
2019-11-04T17:31:11.2319134Z     = note: expected type `*mut libc::c_void`
2019-11-04T17:31:11.2319573Z                found type `*const _`
2019-11-04T17:31:11.3021461Z error: aborting due to 5 previous errors
2019-11-04T17:31:11.3022380Z 
2019-11-04T17:31:11.3022989Z Some errors have detailed explanations: E0308, E0425, E0428.
2019-11-04T17:31:11.3023425Z For more information about an error, try `rustc --explain E0308`.
---
2019-11-04T17:31:11.3518027Z   local time: Mon Nov  4 17:31:11 UTC 2019
2019-11-04T17:31:11.5109221Z   network time: Mon, 04 Nov 2019 17:31:11 GMT
2019-11-04T17:31:11.5116188Z == end clock drift check ==
2019-11-04T17:31:14.3831320Z 
2019-11-04T17:31:14.3941353Z ##[error]Bash exited with code '1'.
2019-11-04T17:31:14.3968050Z ##[section]Starting: Checkout
2019-11-04T17:31:14.3969694Z ==============================================================================
2019-11-04T17:31:14.3969921Z Task         : Get sources
2019-11-04T17:31:14.3969966Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
