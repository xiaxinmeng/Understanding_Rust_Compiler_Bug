plain
2019-12-08T09:34:50.8323339Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-08T09:34:50.8500206Z ##[command]git config gc.auto 0
2019-12-08T09:34:50.8590112Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-08T09:34:50.8643357Z ##[command]git config --get-all http.proxy
2019-12-08T09:34:50.8791238Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67143/merge:refs/remotes/pull/67143/merge
---
2019-12-08T09:39:44.8346780Z     Checking core v0.0.0 (/checkout/src/libcore)
2019-12-08T09:39:52.7691255Z error[E0061]: this function takes 1 parameter but 0 parameters were supplied
2019-12-08T09:39:52.7692474Z     --> src/libcore/char/methods.rs:1251:17
2019-12-08T09:39:52.7692939Z      |
2019-12-08T09:39:52.7693485Z 60   |     pub fn is_digit(self, radix: u32) -> bool {
2019-12-08T09:39:52.7695003Z ...
2019-12-08T09:39:52.7696157Z 1251 |         if self.is_digit() {
2019-12-08T09:39:52.7696782Z      |                 ^^^^^^^^ expected 1 parameter
2019-12-08T09:39:52.7696970Z 
2019-12-08T09:39:52.7696970Z 
2019-12-08T09:39:52.7728692Z error[E0606]: casting `&char` as `u8` is invalid
2019-12-08T09:39:52.7729750Z      |
2019-12-08T09:39:52.7729750Z      |
2019-12-08T09:39:52.7730195Z 1254 |         let code = (self as u8) & !0x20; // 0x20 is the case bit
2019-12-08T09:39:52.7731398Z      |
2019-12-08T09:39:52.7731986Z      = help: cast through a raw pointer first
2019-12-08T09:39:52.7732167Z 
2019-12-08T09:39:53.3960579Z    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
---
2019-12-08T09:39:57.0082595Z   local time: Sun Dec  8 09:39:57 UTC 2019
2019-12-08T09:39:57.2870828Z   network time: Sun, 08 Dec 2019 09:39:57 GMT
2019-12-08T09:39:57.2875246Z == end clock drift check ==
2019-12-08T09:40:10.7295533Z 
2019-12-08T09:40:10.7405103Z ##[error]Bash exited with code '1'.
2019-12-08T09:40:10.7434155Z ##[section]Starting: Checkout
2019-12-08T09:40:10.7436131Z ==============================================================================
2019-12-08T09:40:10.7436186Z Task         : Get sources
2019-12-08T09:40:10.7436250Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
