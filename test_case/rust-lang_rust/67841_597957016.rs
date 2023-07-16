plain
2020-03-12T01:03:41.1405515Z ========================== Starting Command Output ===========================
2020-03-12T01:03:41.1424806Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/7cb7b642-d728-414d-9095-0bf46407a825.sh
2020-03-12T01:03:41.3577165Z 
2020-03-12T01:03:41.3665341Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-12T01:03:41.3690771Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67841/merge to s
2020-03-12T01:03:41.3698737Z Task         : Get sources
2020-03-12T01:03:41.3699071Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-12T01:03:41.3699403Z Version      : 1.0.0
2020-03-12T01:03:41.3699632Z Author       : Microsoft
---
2020-03-12T01:03:43.7169019Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-12T01:03:43.7320637Z ##[command]git config gc.auto 0
2020-03-12T01:03:43.7359922Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-12T01:03:43.7385968Z ##[command]git config --get-all http.proxy
2020-03-12T01:03:43.7496858Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67841/merge:refs/remotes/pull/67841/merge
---
2020-03-12T01:09:54.7394874Z     Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2020-03-12T01:09:55.8777993Z error[E0407]: method `can_read_vectored` is not a member of trait `io::Read`
2020-03-12T01:09:55.8779811Z    --> src/libstd/sys/unix/ext/net.rs:617:5
2020-03-12T01:09:55.8780880Z     |
2020-03-12T01:09:55.8782060Z 617 | /     fn can_read_vectored(&self) -> bool {
2020-03-12T01:09:55.8783965Z 618 | |         io::Read::can_read_vectored(&&*self)
2020-03-12T01:09:55.8786741Z     | |_____^ not a member of trait `io::Read`
2020-03-12T01:09:55.8790602Z 
2020-03-12T01:09:55.8798536Z error[E0407]: method `can_read_vectored` is not a member of trait `io::Read`
2020-03-12T01:09:55.8799519Z    --> src/libstd/sys/unix/ext/net.rs:638:5
---
2020-03-12T01:09:55.8808688Z 
2020-03-12T01:09:57.7755833Z error[E0599]: no function or associated item named `can_read_vectored` found for trait object `dyn io::Read` in the current scope
2020-03-12T01:09:57.7756959Z    --> src/libstd/sys/unix/ext/net.rs:618:19
2020-03-12T01:09:57.7757572Z     |
2020-03-12T01:09:57.7758325Z 618 |         io::Read::can_read_vectored(&&*self)
2020-03-12T01:09:57.7759960Z     |                   |
2020-03-12T01:09:57.7760924Z     |                   function or associated item not found in `dyn io::Read`
2020-03-12T01:09:57.7762073Z     |                   help: there is a method with a similar name: `is_read_vectored`
2020-03-12T01:09:57.7762543Z 
---
2020-03-12T01:09:57.7796082Z     |
2020-03-12T01:09:57.7796731Z 21  | pub struct Socket(FileDesc);
2020-03-12T01:09:57.7797830Z     | ---------------------------- method `can_read_vectored` not found for this
2020-03-12T01:09:57.7798274Z 
2020-03-12T01:09:57.7827262Z error[E0599]: no method named `can_write_vectored` found for struct `sys::unix::net::Socket` in the current scope
2020-03-12T01:09:57.7828823Z     |
2020-03-12T01:09:57.7828823Z     |
2020-03-12T01:09:57.7829521Z 680 |         self.0.can_write_vectored()
2020-03-12T01:09:57.7830689Z     |                ^^^^^^^^^^^^^^^^^^ help: there is a method with a similar name: `is_write_vectored`
2020-03-12T01:09:57.7832211Z    ::: src/libstd/sys/unix/net.rs:21:1
2020-03-12T01:09:57.7832806Z     |
2020-03-12T01:09:57.7833472Z 21  | pub struct Socket(FileDesc);
2020-03-12T01:09:57.7833472Z 21  | pub struct Socket(FileDesc);
2020-03-12T01:09:57.7834465Z     | ---------------------------- method `can_write_vectored` not found for this
2020-03-12T01:09:58.0171999Z error: aborting due to 5 previous errors
2020-03-12T01:09:58.0177531Z 
2020-03-12T01:09:58.0178431Z Some errors have detailed explanations: E0407, E0599.
2020-03-12T01:09:58.0179123Z For more information about an error, try `rustc --explain E0407`.
---
2020-03-12T01:09:58.0404575Z   local time: Thu Mar 12 01:09:58 UTC 2020
2020-03-12T01:09:58.3289300Z   network time: Thu, 12 Mar 2020 01:09:58 GMT
2020-03-12T01:09:58.3289932Z == end clock drift check ==
2020-03-12T01:09:59.3536898Z 
2020-03-12T01:09:59.3603387Z ##[error]Bash exited with code '1'.
2020-03-12T01:09:59.3621777Z ##[section]Finishing: Run build
2020-03-12T01:09:59.3673584Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67841/merge to s
2020-03-12T01:09:59.3678362Z Task         : Get sources
2020-03-12T01:09:59.3678709Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-12T01:09:59.3679046Z Version      : 1.0.0
2020-03-12T01:09:59.3679268Z Author       : Microsoft
2020-03-12T01:09:59.3679268Z Author       : Microsoft
2020-03-12T01:09:59.3679621Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-12T01:09:59.3680049Z ==============================================================================
2020-03-12T01:09:59.7322377Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-12T01:09:59.7368549Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/67841/merge to s
2020-03-12T01:09:59.7471974Z Cleaning up task key
2020-03-12T01:09:59.7473360Z Start cleaning up orphan processes.
2020-03-12T01:09:59.7677973Z Terminate orphan process: pid (3755) (python)
2020-03-12T01:09:59.7835613Z ##[section]Finishing: Finalize Job
