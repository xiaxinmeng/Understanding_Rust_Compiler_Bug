plain
2019-09-05T15:20:39.1459230Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-05T15:20:39.1475179Z ##[command]git config gc.auto 0
2019-09-05T15:20:39.1477933Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-05T15:20:39.1485568Z ##[command]git config --get-all http.proxy
2019-09-05T15:20:39.1502083Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64177/merge:refs/remotes/pull/64177/merge
---
2019-09-05T15:54:34.7729889Z    Compiling parking_lot_core v0.4.0
2019-09-05T15:54:35.1779425Z    Compiling ena v0.13.0
2019-09-05T15:54:35.6115102Z    Compiling polonius-engine v0.10.0
2019-09-05T15:54:36.7051056Z    Compiling num_cpus v1.8.0
2019-09-05T15:54:37.6407876Z error[E0599]: no method named `serialize_element` found for type `<Self as ser::Serializer>::SerializeSeq` in the current scope
2019-09-05T15:54:37.6408301Z     --> /cargo/registry/src/github.com-1ecc6299db9ec823/serde-1.0.99/src/ser/mod.rs:1275:29
2019-09-05T15:54:37.6408541Z      |
2019-09-05T15:54:37.6408790Z 1275 |             try!(serializer.serialize_element(&item));
2019-09-05T15:54:37.6409237Z      |
2019-09-05T15:54:37.6409639Z      = help: items from traits can only be used if the trait is in scope
2019-09-05T15:54:37.6409639Z      = help: items from traits can only be used if the trait is in scope
2019-09-05T15:54:37.6410276Z      = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
2019-09-05T15:54:37.6410502Z              `use ser::SerializeSeq;`
2019-09-05T15:54:37.6410600Z 
2019-09-05T15:54:37.6578533Z error[E0599]: no method named `serialize_entry` found for type `<Self as ser::Serializer>::SerializeMap` in the current scope
2019-09-05T15:54:37.6578927Z     --> /cargo/registry/src/github.com-1ecc6299db9ec823/serde-1.0.99/src/ser/mod.rs:1315:29
2019-09-05T15:54:37.6579188Z      |
2019-09-05T15:54:37.6579481Z 1315 |             try!(serializer.serialize_entry(&key, &value));
2019-09-05T15:54:37.6580017Z      |
2019-09-05T15:54:37.6580309Z      = help: items from traits can only be used if the trait is in scope
2019-09-05T15:54:37.6580309Z      = help: items from traits can only be used if the trait is in scope
2019-09-05T15:54:37.6580623Z      = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
2019-09-05T15:54:37.6580869Z              `use ser::SerializeMap;`
2019-09-05T15:54:38.3507533Z    Compiling jobserver v0.1.16
2019-09-05T15:54:38.7847341Z error: aborting due to 2 previous errors
2019-09-05T15:54:38.7853180Z 
2019-09-05T15:54:38.7859684Z For more information about this error, try `rustc --explain E0599`.
---
2019-09-05T15:54:40.5035858Z == clock drift check ==
2019-09-05T15:54:40.5055612Z   local time: Thu Sep  5 15:54:40 UTC 2019
2019-09-05T15:54:40.6630938Z   network time: Thu, 05 Sep 2019 15:54:40 GMT
2019-09-05T15:54:40.6635965Z == end clock drift check ==
2019-09-05T15:54:42.0522237Z ##[error]Bash exited with code '1'.
2019-09-05T15:54:42.0562440Z ##[section]Starting: Checkout
2019-09-05T15:54:42.0565032Z ==============================================================================
2019-09-05T15:54:42.0565090Z Task         : Get sources
2019-09-05T15:54:42.0565157Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
