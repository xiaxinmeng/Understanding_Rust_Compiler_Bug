plain
2019-10-05T16:53:59.1422605Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-05T16:53:59.1609391Z ##[command]git config gc.auto 0
2019-10-05T16:53:59.1682877Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-05T16:53:59.1742965Z ##[command]git config --get-all http.proxy
2019-10-05T16:53:59.1898621Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65135/merge:refs/remotes/pull/65135/merge
---
2019-10-05T16:58:49.2587583Z    Compiling serde_derive v1.0.81
2019-10-05T16:59:06.7220218Z    Compiling serde_json v1.0.40
2019-10-05T16:59:07.1082498Z    Compiling toml v0.5.3
2019-10-05T16:59:10.0615708Z    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
2019-10-05T16:59:11.6228081Z error[E0609]: no field `stage` on type `test::ErrorCodes`
2019-10-05T16:59:11.6238384Z    --> src/bootstrap/test.rs:539:26
2019-10-05T16:59:11.6343798Z 539 |         let stage = self.stage;
2019-10-05T16:59:11.6344135Z     |                          ^^^^^ unknown field
2019-10-05T16:59:11.6347593Z     |
2019-10-05T16:59:11.6347853Z     = note: available fields are: `compiler`
2019-10-05T16:59:11.6347853Z     = note: available fields are: `compiler`
2019-10-05T16:59:11.6347981Z 
2019-10-05T16:59:11.6348166Z error[E0609]: no field `host` on type `test::ErrorCodes`
2019-10-05T16:59:11.6348893Z    --> src/bootstrap/test.rs:540:25
2019-10-05T16:59:11.6349622Z 540 |         let host = self.host;
2019-10-05T16:59:11.6350132Z     |                         ^^^^ unknown field
2019-10-05T16:59:11.6350390Z     |
2019-10-05T16:59:11.6350672Z     = note: available fields are: `compiler`
---
2019-10-05T16:59:13.2590846Z == clock drift check ==
2019-10-05T16:59:13.2605888Z   local time: Sat Oct  5 16:59:13 UTC 2019
2019-10-05T16:59:13.4114678Z   network time: Sat, 05 Oct 2019 16:59:13 GMT
2019-10-05T16:59:13.4114949Z == end clock drift check ==
2019-10-05T16:59:15.9883608Z ##[error]Bash exited with code '1'.
2019-10-05T16:59:15.9918081Z ##[section]Starting: Checkout
2019-10-05T16:59:15.9919701Z ==============================================================================
2019-10-05T16:59:15.9919771Z Task         : Get sources
2019-10-05T16:59:15.9919817Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
