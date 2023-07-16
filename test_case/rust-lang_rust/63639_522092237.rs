plain
2019-08-16T16:20:17.5359706Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-16T16:20:17.5531449Z ##[command]git config gc.auto 0
2019-08-16T16:20:17.5598622Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-16T16:20:17.5666167Z ##[command]git config --get-all http.proxy
2019-08-16T16:20:17.5806706Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63639/merge:refs/remotes/pull/63639/merge
---
2019-08-16T16:20:54.0135749Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-16T16:20:54.0135773Z 
2019-08-16T16:20:54.0135959Z   git checkout -b <new-branch-name>
2019-08-16T16:20:54.0135986Z 
2019-08-16T16:20:54.0136022Z HEAD is now at 4a69ee04b Merge fb313ff4175a011f6718b1721928277dd238e5bc into 9dd5c191993aab6c2f1538eb8ab69afdc4b6e67a
2019-08-16T16:20:54.0286424Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-16T16:20:54.0289581Z ==============================================================================
2019-08-16T16:20:54.0289641Z Task         : Bash
2019-08-16T16:20:54.0289685Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-16T16:30:07.5851052Z     Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
2019-08-16T16:30:08.1152944Z error: hidden lifetime parameters in types are deprecated
2019-08-16T16:30:08.1153305Z    --> src/librustdoc/clean/mod.rs:139:27
2019-08-16T16:30:08.1153494Z     |
2019-08-16T16:30:08.1153764Z 139 | pub fn krate(mut cx: &mut DocContext) -> Crate {
2019-08-16T16:30:08.1154119Z     |                           ^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-08-16T16:30:10.6277559Z error: aborting due to previous error
2019-08-16T16:30:10.6278363Z 
2019-08-16T16:30:10.6625880Z error: Could not compile `rustdoc`.
2019-08-16T16:30:10.6626042Z 
---
2019-08-16T16:30:10.6715571Z == clock drift check ==
2019-08-16T16:30:10.6731719Z   local time: Fri Aug 16 16:30:10 UTC 2019
2019-08-16T16:30:10.8303690Z   network time: Fri, 16 Aug 2019 16:30:10 GMT
2019-08-16T16:30:10.8306088Z == end clock drift check ==
2019-08-16T16:30:12.8828655Z ##[error]Bash exited with code '1'.
2019-08-16T16:30:12.8862032Z ##[section]Starting: Checkout
2019-08-16T16:30:12.8863634Z ==============================================================================
2019-08-16T16:30:12.8863699Z Task         : Get sources
2019-08-16T16:30:12.8863735Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
