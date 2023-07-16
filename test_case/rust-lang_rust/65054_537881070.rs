plain
2019-10-03T10:03:05.0739037Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-03T10:03:05.0972538Z ##[command]git config gc.auto 0
2019-10-03T10:03:05.1033176Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-03T10:03:05.1081493Z ##[command]git config --get-all http.proxy
2019-10-03T10:03:05.1231932Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65054/merge:refs/remotes/pull/65054/merge
---
2019-10-03T10:09:24.5844421Z     Checking hashbrown v0.5.0
2019-10-03T10:09:27.2707568Z error[E0308]: mismatched types
2019-10-03T10:09:27.2707894Z    --> src/libstd/net/ip.rs:323:10
2019-10-03T10:09:27.2708167Z     |
2019-10-03T10:09:27.2708635Z 322 |     pub const fn new(a: u8, b: u8, c: u8, d: u8) -> Ipv4Addr {
2019-10-03T10:09:27.2709132Z     |                                                     -------- expected `net::ip::Ipv4Addr` because of return type
2019-10-03T10:09:27.2709417Z 323 |          u32::from_be_bytes([a, b, c, d])
2019-10-03T10:09:27.2709678Z     |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `net::ip::Ipv4Addr`, found u32
2019-10-03T10:09:27.2710112Z     = note: expected type `net::ip::Ipv4Addr`
2019-10-03T10:09:27.2710323Z                found type `u32`
2019-10-03T10:09:27.2710358Z 
2019-10-03T10:09:28.2013317Z error: aborting due to previous error
---
2019-10-03T10:09:28.2517259Z == clock drift check ==
2019-10-03T10:09:28.2537661Z   local time: Thu Oct  3 10:09:28 UTC 2019
2019-10-03T10:09:28.5178018Z   network time: Thu, 03 Oct 2019 10:09:28 GMT
2019-10-03T10:09:28.5178117Z == end clock drift check ==
2019-10-03T10:09:34.9715428Z ##[error]Bash exited with code '1'.
2019-10-03T10:09:34.9756365Z ##[section]Starting: Checkout
2019-10-03T10:09:34.9758607Z ==============================================================================
2019-10-03T10:09:34.9758656Z Task         : Get sources
2019-10-03T10:09:34.9758716Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
