plain
2019-08-30T14:38:15.5776857Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-30T14:38:15.5959214Z ##[command]git config gc.auto 0
2019-08-30T14:38:16.2323660Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-30T14:38:16.2329074Z ##[command]git config --get-all http.proxy
2019-08-30T14:38:16.2334914Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64021/merge:refs/remotes/pull/64021/merge
---
2019-08-30T14:44:54.7112070Z    Compiling rand_pcg v0.1.1
2019-08-30T14:44:55.1228023Z    Compiling rand_chacha v0.1.0
2019-08-30T14:44:55.4896955Z    Compiling rand v0.6.1
2019-08-30T14:44:55.8863712Z    Compiling parking_lot_core v0.4.0
2019-08-30T14:44:57.5759660Z error[E0599]: no method named `to_bytes` found for type `&std::ffi::CStr` in the current scope
2019-08-30T14:44:57.5760102Z   --> /cargo/registry/src/github.com-1ecc6299db9ec823/serde-1.0.99/src/ser/impls.rs:82:41
2019-08-30T14:44:57.5760379Z    |
2019-08-30T14:44:57.5760698Z 82 |         serializer.serialize_bytes(self.to_bytes())
2019-08-30T14:44:57.5764183Z 
2019-08-30T14:44:57.5809915Z error[E0599]: no method named `to_bytes` found for type `&std::ffi::CString` in the current scope
2019-08-30T14:44:57.5810314Z   --> /cargo/registry/src/github.com-1ecc6299db9ec823/serde-1.0.99/src/ser/impls.rs:93:41
2019-08-30T14:44:57.5810582Z    |
2019-08-30T14:44:57.5810582Z    |
2019-08-30T14:44:57.5810884Z 93 |         serializer.serialize_bytes(self.to_bytes())
2019-08-30T14:44:57.5811263Z    |                                         ^^^^^^^^ help: there is a method with a similar name: `as_bytes`
2019-08-30T14:44:58.4457192Z     Checking num_cpus v1.8.0
2019-08-30T14:44:58.4857164Z error: aborting due to 2 previous errors
2019-08-30T14:44:58.4857552Z 
2019-08-30T14:44:58.4869762Z For more information about this error, try `rustc --explain E0599`.
---
2019-08-30T14:44:58.5626856Z == clock drift check ==
2019-08-30T14:44:58.5644547Z   local time: Fri Aug 30 14:44:58 UTC 2019
2019-08-30T14:44:59.3409654Z   network time: Fri, 30 Aug 2019 14:44:58 GMT
2019-08-30T14:44:59.3409757Z == end clock drift check ==
2019-08-30T14:45:01.6609049Z ##[error]Bash exited with code '1'.
2019-08-30T14:45:01.6640115Z ##[section]Starting: Checkout
2019-08-30T14:45:01.6641741Z ==============================================================================
2019-08-30T14:45:01.6641795Z Task         : Get sources
2019-08-30T14:45:01.6641857Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
