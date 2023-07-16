plain
2019-10-14T00:00:07.0051516Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-14T00:00:07.0131602Z ##[command]git config gc.auto 0
2019-10-14T00:00:07.0210349Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-14T00:00:07.0259522Z ##[command]git config --get-all http.proxy
2019-10-14T00:00:07.0386861Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65331/merge:refs/remotes/pull/65331/merge
---
2019-10-14T00:08:40.4578865Z     Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
2019-10-14T00:08:42.5615283Z error[E0282]: type annotations needed for `&&T`
2019-10-14T00:08:42.5616241Z     --> src/librustdoc/html/render.rs:2515:25
2019-10-14T00:08:42.5616694Z      |
2019-10-14T00:08:42.5617474Z 2515 |             .partition(|i| i.inner_impl().synthetic);
2019-10-14T00:08:42.5618156Z      |                         ^ consider giving this closure parameter the explicit type `&&T`, where the type parameter `T` is specified
2019-10-14T00:08:42.5619100Z      = note: type must be known at this point
2019-10-14T00:08:42.5619341Z 
2019-10-14T00:08:42.5619341Z 
2019-10-14T00:08:42.7102286Z error[E0277]: the trait bound `std::vec::Vec<&&html::render::Impl>: std::iter::Extend<&html::render::Impl>` is not satisfied
2019-10-14T00:08:42.7102633Z     --> src/librustdoc/html/render.rs:3202:14
2019-10-14T00:08:42.7102905Z      |
2019-10-14T00:08:42.7108331Z 3202 |             .partition(|t| t.inner_impl().synthetic);
2019-10-14T00:08:42.7108993Z      |              ^^^^^^^^^ the trait `std::iter::Extend<&html::render::Impl>` is not implemented for `std::vec::Vec<&&html::render::Impl>`
2019-10-14T00:08:42.7109756Z      = help: the following implementations were found:
2019-10-14T00:08:42.7109756Z      = help: the following implementations were found:
2019-10-14T00:08:42.7110857Z                <std::vec::Vec<T> as std::iter::Extend<&'a T>>
2019-10-14T00:08:42.7111302Z                <std::vec::Vec<T> as std::iter::Extend<T>>
2019-10-14T00:08:43.1234277Z error: aborting due to 2 previous errors
2019-10-14T00:08:43.1234387Z 
2019-10-14T00:08:43.1234619Z Some errors have detailed explanations: E0277, E0282.
2019-10-14T00:08:43.1234853Z For more information about an error, try `rustc --explain E0277`.
---
2019-10-14T00:08:43.1661574Z == clock drift check ==
2019-10-14T00:08:43.1677567Z   local time: Mon Oct 14 00:08:43 UTC 2019
2019-10-14T00:08:43.3181226Z   network time: Mon, 14 Oct 2019 00:08:43 GMT
2019-10-14T00:08:43.3181328Z == end clock drift check ==
2019-10-14T00:08:44.2868646Z ##[error]Bash exited with code '1'.
2019-10-14T00:08:44.3037646Z ##[section]Starting: Checkout
2019-10-14T00:08:44.3039684Z ==============================================================================
2019-10-14T00:08:44.3040481Z Task         : Get sources
2019-10-14T00:08:44.3041186Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
