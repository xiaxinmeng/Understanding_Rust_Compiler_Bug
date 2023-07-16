plain
2020-04-06T22:56:31.1000806Z ========================== Starting Command Output ===========================
2020-04-06T22:56:31.1003328Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/332a7114-8a87-4ee1-ad2a-41d7711c12d8.sh
2020-04-06T22:56:31.1003544Z 
2020-04-06T22:56:31.1008360Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-06T22:56:31.1027536Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70855/merge to s
2020-04-06T22:56:31.1030829Z Task         : Get sources
2020-04-06T22:56:31.1031083Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-06T22:56:31.1031323Z Version      : 1.0.0
2020-04-06T22:56:31.1031486Z Author       : Microsoft
---
2020-04-06T22:56:32.3693572Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-06T22:56:32.3699065Z ##[command]git config gc.auto 0
2020-04-06T22:56:32.3702094Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-06T22:56:32.3705111Z ##[command]git config --get-all http.proxy
2020-04-06T22:56:32.3711792Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70855/merge:refs/remotes/pull/70855/merge
---
2020-04-06T22:59:44.4171574Z  ---> 3fc1b512c57b
2020-04-06T22:59:44.4171912Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-06T22:59:44.4172400Z  ---> Using cache
2020-04-06T22:59:44.4172851Z  ---> 5ee4295733f4
2020-04-06T22:59:44.4174200Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-06T22:59:44.4175702Z  ---> 3d07a0fa42fe
2020-04-06T22:59:44.4176015Z Successfully built 3d07a0fa42fe
2020-04-06T22:59:44.4176503Z Successfully tagged rust-ci:latest
2020-04-06T22:59:44.4177183Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-06T22:59:44.4177183Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-06T22:59:44.4177839Z Looks like docker image is the same as before, not uploading
2020-04-06T22:59:51.8821276Z [CI_JOB_NAME=mingw-check]
2020-04-06T22:59:51.9051452Z [CI_JOB_NAME=mingw-check]
2020-04-06T22:59:51.9076407Z == clock drift check ==
2020-04-06T22:59:51.9086440Z   local time: Mon Apr  6 22:59:51 UTC 2020
2020-04-06T22:59:52.1955674Z   network time: Mon, 06 Apr 2020 22:59:52 GMT
2020-04-06T22:59:52.1979333Z Starting sccache server...
2020-04-06T22:59:52.2849429Z configure: processing command line
2020-04-06T22:59:52.2849663Z configure: 
2020-04-06T22:59:52.2850708Z configure: rust.parallel-compiler := True
---
2020-04-06T23:01:51.8686540Z    Compiling compiler_builtins v0.1.25
2020-04-06T23:01:54.6456358Z error: implementation has missing stability attribute
2020-04-06T23:01:54.6457047Z    --> src/libcore/num/mod.rs:114:13
2020-04-06T23:01:54.6457486Z     |
2020-04-06T23:01:54.6458088Z 45  |  / macro_rules! nonzero_integers {
2020-04-06T23:01:54.6458962Z 46  |  |     ( $( #[$stability: meta] $Ty: ident($Int: ty); )+ ) => {
2020-04-06T23:01:54.6460523Z 48  |  |             doc_comment! {
2020-04-06T23:01:54.6461092Z ...    |
2020-04-06T23:01:54.6461092Z ...    |
2020-04-06T23:01:54.6461976Z 114 | /|             impl PartialEq<$Int> for $Ty {
2020-04-06T23:01:54.6462951Z 115 | ||                 fn eq(&self, rhs: &$Int) -> bool {
2020-04-06T23:01:54.6463875Z 116 | ||                     self.0 == *rhs
2020-04-06T23:01:54.6464849Z 117 | ||                 }
2020-04-06T23:01:54.6465606Z 118 | ||             }
2020-04-06T23:01:54.6466304Z     | ||_____________^
2020-04-06T23:01:54.6467444Z 130 |  |     }
2020-04-06T23:01:54.6468110Z 131 |  | }
2020-04-06T23:01:54.6469006Z     |  |_- in this expansion of `nonzero_integers!`
2020-04-06T23:01:54.6469782Z 132 | 
2020-04-06T23:01:54.6469782Z 132 | 
2020-04-06T23:01:54.6470437Z 133 | /  nonzero_integers! {
2020-04-06T23:01:54.6471532Z 134 | |      #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU8(u8);
2020-04-06T23:01:54.6472494Z 135 | |      #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU16(u16);
2020-04-06T23:01:54.6473480Z 136 | |      #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU32(u32);
2020-04-06T23:01:54.6474613Z ...   |
2020-04-06T23:01:54.6475358Z 145 | |      #[stable(feature = "signed_nonzero", since = "1.34.0")] NonZeroIsize(isize);
2020-04-06T23:01:54.6477074Z     | |__- in this macro invocation
2020-04-06T23:01:54.6477378Z 
2020-04-06T23:01:54.6482818Z error: implementation has missing stability attribute
2020-04-06T23:01:54.6483501Z    --> src/libcore/num/mod.rs:120:13
2020-04-06T23:01:54.6483501Z    --> src/libcore/num/mod.rs:120:13
2020-04-06T23:01:54.6483951Z     |
2020-04-06T23:01:54.6484726Z 45  |  / macro_rules! nonzero_integers {
2020-04-06T23:01:54.6485586Z 46  |  |     ( $( #[$stability: meta] $Ty: ident($Int: ty); )+ ) => {
2020-04-06T23:01:54.6487107Z 48  |  |             doc_comment! {
2020-04-06T23:01:54.6487683Z ...    |
2020-04-06T23:01:54.6487683Z ...    |
2020-04-06T23:01:54.6488372Z 120 | /|             impl PartialOrd<$Int> for $Ty {
2020-04-06T23:01:54.6489377Z 121 | ||                 fn partial_cmp(&self, rhs: &$Int) -> Option<Ordering> {
2020-04-06T23:01:54.6490895Z 122 | ||                     Some(self.0.cmp(rhs))
2020-04-06T23:01:54.6491916Z 123 | ||                 }
2020-04-06T23:01:54.6492705Z 124 | ||             }
2020-04-06T23:01:54.6493426Z     | ||_____________^
2020-04-06T23:01:54.6494597Z 130 |  |     }
2020-04-06T23:01:54.6495410Z 131 |  | }
2020-04-06T23:01:54.6496149Z     |  |_- in this expansion of `nonzero_integers!`
2020-04-06T23:01:54.6496720Z 132 | 
2020-04-06T23:01:54.6496720Z 132 | 
2020-04-06T23:01:54.6497327Z 133 | /  nonzero_integers! {
2020-04-06T23:01:54.6498121Z 134 | |      #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU8(u8);
2020-04-06T23:01:54.6498989Z 135 | |      #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU16(u16);
2020-04-06T23:01:54.6499878Z 136 | |      #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU32(u32);
2020-04-06T23:01:54.6500491Z ...   |
2020-04-06T23:01:54.6501414Z 145 | |      #[stable(feature = "signed_nonzero", since = "1.34.0")] NonZeroIsize(isize);
2020-04-06T23:01:54.6503096Z     | |__- in this macro invocation
2020-04-06T23:01:54.6503401Z 
2020-04-06T23:01:55.4507308Z error: aborting due to 2 previous errors
2020-04-06T23:01:55.4512846Z 
---
2020-04-06T23:01:55.4807665Z expected success, got: exit code: 101
2020-04-06T23:01:55.4814815Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-04-06T23:01:55.4815152Z Build completed unsuccessfully in 0:02:03
2020-04-06T23:01:55.4869382Z == clock drift check ==
2020-04-06T23:01:55.5267260Z   local time: Mon Apr  6 23:01:55 UTC 2020
2020-04-06T23:01:55.8118326Z   network time: Mon, 06 Apr 2020 23:01:55 GMT
2020-04-06T23:01:56.4777761Z 
2020-04-06T23:01:56.4777761Z 
2020-04-06T23:01:56.4844633Z ##[error]Bash exited with code '1'.
2020-04-06T23:01:56.4858371Z ##[section]Finishing: Run build
2020-04-06T23:01:56.4917562Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70855/merge to s
2020-04-06T23:01:56.4922903Z Task         : Get sources
2020-04-06T23:01:56.4923211Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-06T23:01:56.4923694Z Version      : 1.0.0
2020-04-06T23:01:56.4923899Z Author       : Microsoft
2020-04-06T23:01:56.4923899Z Author       : Microsoft
2020-04-06T23:01:56.4924228Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-06T23:01:56.4924775Z ==============================================================================
2020-04-06T23:01:56.8009543Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-06T23:01:56.8049584Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70855/merge to s
2020-04-06T23:01:56.8135163Z Cleaning up task key
2020-04-06T23:01:56.8136987Z Start cleaning up orphan processes.
2020-04-06T23:01:56.8316376Z Terminate orphan process: pid (3456) (python)
2020-04-06T23:01:56.8457637Z ##[section]Finishing: Finalize Job
