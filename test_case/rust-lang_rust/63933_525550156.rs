plain
2019-08-28T01:32:44.5372667Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-28T01:32:44.5593014Z ##[command]git config gc.auto 0
2019-08-28T01:32:44.5673101Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-28T01:32:44.5727648Z ##[command]git config --get-all http.proxy
2019-08-28T01:32:44.5870990Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63933/merge:refs/remotes/pull/63933/merge
---
2019-08-28T01:33:18.7408556Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-28T01:33:18.7408602Z 
2019-08-28T01:33:18.7409106Z   git checkout -b <new-branch-name>
2019-08-28T01:33:18.7409152Z 
2019-08-28T01:33:18.7409201Z HEAD is now at 5d00317ef Merge 8b6318ac976c67a966ca1863a48376c8f558ad88 into 53df91a9b24ad999e7ca896447af6f5f74fe43bc
2019-08-28T01:33:18.7624436Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-28T01:33:18.7627414Z ==============================================================================
2019-08-28T01:33:18.7627465Z Task         : Bash
2019-08-28T01:33:18.7627505Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-28T01:42:23.3505602Z 
2019-08-28T01:42:23.3532774Z error: incorrect close delimiter: `)`
2019-08-28T01:42:23.3534074Z    --> src/librustc_metadata/decoder.rs:916:52
2019-08-28T01:42:23.3534615Z     |
2019-08-28T01:42:23.3535142Z 911 |     pub fn get_optimized_mir(&self, tcx: TyCtxt<'tcx>, id: DefIndex) -> Body<'tcx> {
2019-08-28T01:42:23.3535748Z     |                                                                                    - un-closed delimiter
2019-08-28T01:42:23.3536168Z ...
2019-08-28T01:42:23.3536669Z 916 |             .mir.map(|mir| mir.decode((self, tcx))))
2019-08-28T01:42:23.3537243Z     |                                                    ^ incorrect close delimiter
2019-08-28T01:42:23.3553469Z error: aborting due to 2 previous errors
2019-08-28T01:42:23.3554346Z 
2019-08-28T01:42:23.3574912Z error: Could not compile `rustc_metadata`.
2019-08-28T01:42:23.3575518Z warning: build failed, waiting for other jobs to finish...
---
2019-08-28T01:42:31.6076733Z == clock drift check ==
2019-08-28T01:42:31.6094782Z   local time: Wed Aug 28 01:42:31 UTC 2019
2019-08-28T01:42:31.7602954Z   network time: Wed, 28 Aug 2019 01:42:31 GMT
2019-08-28T01:42:31.7603488Z == end clock drift check ==
2019-08-28T01:42:32.9870620Z ##[error]Bash exited with code '1'.
2019-08-28T01:42:32.9910787Z ##[section]Starting: Checkout
2019-08-28T01:42:32.9912433Z ==============================================================================
2019-08-28T01:42:32.9912485Z Task         : Get sources
2019-08-28T01:42:32.9912548Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
