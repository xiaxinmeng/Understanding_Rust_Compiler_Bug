plain
2019-12-11T17:02:21.3386446Z ========================== Starting Command Output ===========================
2019-12-11T17:02:21.3387563Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/260f8301-421f-4d8e-96ec-2ebd11c37aa9.sh
2019-12-11T17:02:21.3387594Z 
2019-12-11T17:02:21.3390024Z ##[section]Finishing: Disable git automatic line ending conversion
2019-12-11T17:02:21.3395221Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67233/merge to s
2019-12-11T17:02:21.3396555Z Task         : Get sources
2019-12-11T17:02:21.3396622Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2019-12-11T17:02:21.3396646Z Version      : 1.0.0
2019-12-11T17:02:21.3396670Z Author       : Microsoft
---
2019-12-11T17:02:23.0426855Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-11T17:02:23.0598291Z ##[command]git config gc.auto 0
2019-12-11T17:02:23.0670176Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-11T17:02:23.0711702Z ##[command]git config --get-all http.proxy
2019-12-11T17:02:23.0832708Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67233/merge:refs/remotes/pull/67233/merge
---
2019-12-11T17:06:45.7085495Z     Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2019-12-11T17:06:49.5511965Z error: impl has missing stability attribute
2019-12-11T17:06:49.5512933Z    --> src/libstd/io/cursor.rs:394:1
2019-12-11T17:06:49.5513430Z     |
2019-12-11T17:06:49.5513932Z 394 | / impl<T: PartialEq> PartialEq for Cursor<T> {
2019-12-11T17:06:49.5514442Z 395 | |     fn eq(&self, other: &Self) -> bool {
2019-12-11T17:06:49.5515010Z 396 | |         self.get_ref() == other.get_ref() && self.pos == other.pos
2019-12-11T17:06:49.5515945Z 398 | | }
2019-12-11T17:06:49.5516363Z     | |_^
2019-12-11T17:06:49.5516607Z 
2019-12-11T17:06:49.5517011Z error: impl has missing stability attribute
2019-12-11T17:06:49.5517011Z error: impl has missing stability attribute
2019-12-11T17:06:49.5517430Z    --> src/libstd/io/cursor.rs:400:1
2019-12-11T17:06:49.5517847Z     |
2019-12-11T17:06:49.5518281Z 400 | impl<T: Eq> Eq for Cursor<T> {}
2019-12-11T17:06:49.5518960Z 
2019-12-11T17:06:49.6596764Z error: aborting due to 2 previous errors
2019-12-11T17:06:50.2098810Z 
2019-12-11T17:06:50.2099830Z error: could not compile `std`.
---
2019-12-11T17:06:50.2101632Z   local time: Wed Dec 11 17:06:49 UTC 2019
2019-12-11T17:06:50.2101808Z   network time: Wed, 11 Dec 2019 17:06:49 GMT
2019-12-11T17:06:50.2102025Z == end clock drift check ==
2019-12-11T17:06:52.5516450Z 
2019-12-11T17:06:52.5592288Z ##[error]Bash exited with code '1'.
2019-12-11T17:06:52.5602079Z ##[section]Finishing: Run build
2019-12-11T17:06:52.5614744Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67233/merge to s
2019-12-11T17:06:52.5616269Z Task         : Get sources
2019-12-11T17:06:52.5616304Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2019-12-11T17:06:52.5616341Z Version      : 1.0.0
2019-12-11T17:06:52.5616389Z Author       : Microsoft
2019-12-11T17:06:52.5616389Z Author       : Microsoft
2019-12-11T17:06:52.5616425Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2019-12-11T17:06:52.5616585Z ==============================================================================
2019-12-11T17:06:52.8680867Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2019-12-11T17:06:52.8711608Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/67233/merge to s
2019-12-11T17:06:52.8797443Z Start cleaning up orphan processes.
2019-12-11T17:06:52.8877797Z Terminate orphan process: pid (5159) (python)
2019-12-11T17:06:52.9018043Z ##[section]Finishing: Finalize Job
2019-12-11T17:06:52.9063377Z ##[section]Finishing: Linux mingw-check
