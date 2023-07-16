plain
2020-01-24T22:56:41.4393298Z ========================== Starting Command Output ===========================
2020-01-24T22:56:41.4633253Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/fcf6e515-16cb-471f-bbb7-863f981935d7.sh
2020-01-24T22:56:41.4634937Z 
2020-01-24T22:56:41.4638368Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-24T22:56:41.4645223Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68524/merge to s
2020-01-24T22:56:41.4647064Z Task         : Get sources
2020-01-24T22:56:41.4647158Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-24T22:56:41.4647200Z Version      : 1.0.0
2020-01-24T22:56:41.4647240Z Author       : Microsoft
---
2020-01-24T22:56:42.2690080Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-24T22:56:42.2698576Z ##[command]git config gc.auto 0
2020-01-24T22:56:42.2700213Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-24T22:56:42.2701524Z ##[command]git config --get-all http.proxy
2020-01-24T22:56:42.2705885Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68524/merge:refs/remotes/pull/68524/merge
---
2020-01-24T23:20:04.4074261Z    Compiling rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
2020-01-24T23:20:04.8217282Z error[E0061]: this function takes 1 parameter but 0 parameters were supplied
2020-01-24T23:20:04.8221216Z   --> src/librustc_data_structures/box_region.rs:53:58
2020-01-24T23:20:04.8221469Z    |
2020-01-24T23:20:04.8221729Z 53 |         let init = match Pin::new(&mut result.generator).resume() {
2020-01-24T23:20:04.8222210Z    |
2020-01-24T23:20:04.8222210Z    |
2020-01-24T23:20:04.8222436Z help: expected the unit value `()`; create it with empty parentheses
2020-01-24T23:20:04.8222631Z    |
2020-01-24T23:20:04.8223054Z 53 |         let init = match Pin::new(&mut result.generator).resume(()) {
2020-01-24T23:20:04.8223377Z 
2020-01-24T23:20:04.8267270Z error[E0061]: this function takes 1 parameter but 0 parameters were supplied
2020-01-24T23:20:04.8267566Z   --> src/librustc_data_structures/box_region.rs:67:76
2020-01-24T23:20:04.8267778Z    |
2020-01-24T23:20:04.8267778Z    |
2020-01-24T23:20:04.8268071Z 67 |         if let GeneratorState::Complete(_) = Pin::new(&mut self.generator).resume() {
2020-01-24T23:20:04.8268734Z    |
2020-01-24T23:20:04.8268734Z    |
2020-01-24T23:20:04.8268971Z help: expected the unit value `()`; create it with empty parentheses
2020-01-24T23:20:04.8269154Z    |
2020-01-24T23:20:04.8269647Z 67 |         if let GeneratorState::Complete(_) = Pin::new(&mut self.generator).resume(()) {
2020-01-24T23:20:04.8269998Z 
2020-01-24T23:20:04.8306927Z error[E0061]: this function takes 1 parameter but 0 parameters were supplied
2020-01-24T23:20:04.8307225Z   --> src/librustc_data_structures/box_region.rs:76:52
2020-01-24T23:20:04.8307415Z    |
2020-01-24T23:20:04.8307415Z    |
2020-01-24T23:20:04.8307703Z 76 |         let result = Pin::new(&mut self.generator).resume();
2020-01-24T23:20:04.8308170Z    |
2020-01-24T23:20:04.8308170Z    |
2020-01-24T23:20:04.8308400Z help: expected the unit value `()`; create it with empty parentheses
2020-01-24T23:20:04.8308580Z    |
2020-01-24T23:20:04.8308829Z 76 |         let result = Pin::new(&mut self.generator).resume(());
2020-01-24T23:20:04.8309136Z 
2020-01-24T23:20:05.2904145Z error: aborting due to 3 previous errors
2020-01-24T23:20:05.2904240Z 
2020-01-24T23:20:05.2904486Z For more information about this error, try `rustc --explain E0061`.
---
2020-01-24T23:20:12.7986935Z   local time: Fri Jan 24 23:20:12 UTC 2020
2020-01-24T23:20:13.3516644Z   network time: Fri, 24 Jan 2020 23:20:13 GMT
2020-01-24T23:20:13.3517419Z == end clock drift check ==
2020-01-24T23:20:14.1024248Z 
2020-01-24T23:20:14.1094568Z ##[error]Bash exited with code '1'.
2020-01-24T23:20:14.1108581Z ##[section]Finishing: Run build
2020-01-24T23:20:14.1127540Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68524/merge to s
2020-01-24T23:20:14.1129255Z Task         : Get sources
2020-01-24T23:20:14.1129404Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-24T23:20:14.1129462Z Version      : 1.0.0
2020-01-24T23:20:14.1129497Z Author       : Microsoft
2020-01-24T23:20:14.1129497Z Author       : Microsoft
2020-01-24T23:20:14.1129538Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-24T23:20:14.1129597Z ==============================================================================
2020-01-24T23:20:14.4731705Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-24T23:20:14.4767714Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68524/merge to s
2020-01-24T23:20:14.4853199Z Cleaning up task key
2020-01-24T23:20:14.4853859Z Start cleaning up orphan processes.
2020-01-24T23:20:14.4937182Z Terminate orphan process: pid (3845) (python)
2020-01-24T23:20:14.5100414Z ##[section]Finishing: Finalize Job
