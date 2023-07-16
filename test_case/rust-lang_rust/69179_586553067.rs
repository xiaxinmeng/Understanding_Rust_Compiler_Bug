plain
2020-02-15T03:20:02.9311196Z ========================== Starting Command Output ===========================
2020-02-15T03:20:02.9314048Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/5a95e7ca-50de-44c0-bcaa-16e9c59a53cf.sh
2020-02-15T03:20:02.9314222Z 
2020-02-15T03:20:02.9319230Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-15T03:20:02.9325002Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69179/merge to s
2020-02-15T03:20:02.9326694Z Task         : Get sources
2020-02-15T03:20:02.9326723Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-15T03:20:02.9326754Z Version      : 1.0.0
2020-02-15T03:20:02.9326826Z Author       : Microsoft
---
2020-02-15T03:20:03.7224789Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-15T03:20:03.7340332Z ##[command]git config gc.auto 0
2020-02-15T03:20:03.7406943Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-15T03:20:03.7462192Z ##[command]git config --get-all http.proxy
2020-02-15T03:20:03.7600643Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69179/merge:refs/remotes/pull/69179/merge
---
2020-02-15T03:29:39.2139393Z     Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
2020-02-15T03:29:40.1816255Z error[E0412]: cannot find type `FunctionRetTy` in crate `hir`
2020-02-15T03:29:40.1817257Z     --> src/librustdoc/clean/mod.rs:1004:36
2020-02-15T03:29:40.1817780Z      |
2020-02-15T03:29:40.1818332Z 1004 | impl Clean<FunctionRetTy> for hir::FunctionRetTy<'_> {
2020-02-15T03:29:40.1819434Z      |
2020-02-15T03:29:40.1819961Z help: possible candidate is found in another module, you can import it into scope
2020-02-15T03:29:40.1820498Z      |
2020-02-15T03:29:40.1821066Z 12   | use crate::clean::types::FunctionRetTy;
2020-02-15T03:29:40.1821066Z 12   | use crate::clean::types::FunctionRetTy;
2020-02-15T03:29:40.1821556Z      |
2020-02-15T03:29:40.1821766Z 
2020-02-15T03:29:40.5005999Z error[E0599]: no method named `clean` found for enum `rustc_hir::FnRetTy<'a>` in the current scope
2020-02-15T03:29:40.5006449Z    --> src/librustdoc/clean/mod.rs:969:35
2020-02-15T03:29:40.5006754Z     |
2020-02-15T03:29:40.5007094Z 969 |             output: self.0.output.clean(cx),
2020-02-15T03:29:40.5007537Z     |                                   ^^^^^ method not found in `rustc_hir::FnRetTy<'a>`
2020-02-15T03:29:40.5008187Z     = help: items from traits can only be used if the trait is implemented and in scope
2020-02-15T03:29:40.5008187Z     = help: items from traits can only be used if the trait is implemented and in scope
2020-02-15T03:29:40.5008783Z     = note: the following trait defines an item `clean`, perhaps you need to implement it:
2020-02-15T03:29:40.5009457Z             candidate #1: `clean::Clean`
2020-02-15T03:29:42.1402936Z error: aborting due to 2 previous errors
2020-02-15T03:29:42.1404191Z 
2020-02-15T03:29:42.1404885Z Some errors have detailed explanations: E0412, E0599.
2020-02-15T03:29:42.1405611Z For more information about an error, try `rustc --explain E0412`.
---
2020-02-15T03:29:42.1674678Z   local time: Sat Feb 15 03:29:42 UTC 2020
2020-02-15T03:29:42.4511288Z   network time: Sat, 15 Feb 2020 03:29:42 GMT
2020-02-15T03:29:42.4517493Z == end clock drift check ==
2020-02-15T03:29:42.9994074Z 
2020-02-15T03:29:43.0113011Z ##[error]Bash exited with code '1'.
2020-02-15T03:29:43.0131845Z ##[section]Finishing: Run build
2020-02-15T03:29:43.0150541Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69179/merge to s
2020-02-15T03:29:43.0153232Z Task         : Get sources
2020-02-15T03:29:43.0153301Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-15T03:29:43.0153346Z Version      : 1.0.0
2020-02-15T03:29:43.0153385Z Author       : Microsoft
2020-02-15T03:29:43.0153385Z Author       : Microsoft
2020-02-15T03:29:43.0153454Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-15T03:29:43.0153517Z ==============================================================================
2020-02-15T03:29:43.3947589Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-15T03:29:43.3996784Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69179/merge to s
2020-02-15T03:29:43.4120797Z Cleaning up task key
2020-02-15T03:29:43.4124837Z Start cleaning up orphan processes.
2020-02-15T03:29:43.4247405Z Terminate orphan process: pid (3672) (python)
2020-02-15T03:29:43.4454988Z ##[section]Finishing: Finalize Job
