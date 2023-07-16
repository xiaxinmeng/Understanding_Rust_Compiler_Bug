plain
2020-03-22T16:30:37.5943979Z ========================== Starting Command Output ===========================
2020-03-22T16:30:37.5949436Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/c26fa20a-1899-441c-9d46-a8c3bf4cc28e.sh
2020-03-22T16:30:37.5949902Z 
2020-03-22T16:30:37.5954392Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-22T16:30:37.5971341Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70277/merge to s
2020-03-22T16:30:37.5974558Z Task         : Get sources
2020-03-22T16:30:37.5974812Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-22T16:30:37.5975056Z Version      : 1.0.0
2020-03-22T16:30:37.5975227Z Author       : Microsoft
---
2020-03-22T16:30:38.5855375Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-22T16:30:38.5861016Z ##[command]git config gc.auto 0
2020-03-22T16:30:38.5864928Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-22T16:30:38.5869049Z ##[command]git config --get-all http.proxy
2020-03-22T16:30:38.5875710Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70277/merge:refs/remotes/pull/70277/merge
---
2020-03-22T16:40:31.8525969Z configure: build.locked-deps    := True
2020-03-22T16:40:31.8526475Z configure: llvm.ccache          := sccache
2020-03-22T16:40:31.8527174Z configure: build.cargo-native-static := True
2020-03-22T16:40:31.8527926Z configure: dist.missing-tools   := True
2020-03-22T16:40:31.8528804Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-03-22T16:40:31.8529733Z configure: writing `config.toml` in current directory
2020-03-22T16:40:31.8530118Z configure: 
2020-03-22T16:40:31.8530757Z configure: run `python /checkout/x.py --help`
2020-03-22T16:40:31.8531179Z configure: 
---
2020-03-22T16:48:43.1189917Z skip untracked path cpu-usage.csv during rustfmt invocations
2020-03-22T16:48:43.1190811Z skip untracked path src/doc/book/ during rustfmt invocations
2020-03-22T16:48:43.1193903Z skip untracked path src/doc/rust-by-example/ during rustfmt invocations
2020-03-22T16:48:43.1197363Z skip untracked path src/llvm-project/ during rustfmt invocations
2020-03-22T16:48:43.3529746Z Diff in /checkout/src/librustc_trait_selection/opaque_types.rs at line 823:
2020-03-22T16:48:43.3530363Z              // The regions that we expect from borrow checking.
2020-03-22T16:48:43.3530957Z              ty::ReEarlyBound(_) | ty::ReFree(_) | ty::ReEmpty(ty::UniverseIndex::ROOT) => {}
2020-03-22T16:48:43.3532189Z -            ty::ReEmpty(_)
2020-03-22T16:48:43.3532921Z -            | ty::RePlaceholder(_)
2020-03-22T16:48:43.3533815Z -            | ty::ReVar(_)
2020-03-22T16:48:43.3534515Z -            | ty::ReScope(_) => {
2020-03-22T16:48:43.3534515Z -            | ty::ReScope(_) => {
2020-03-22T16:48:43.3535152Z +            ty::ReEmpty(_) | ty::RePlaceholder(_) | ty::ReVar(_) | ty::ReScope(_) => {
2020-03-22T16:48:43.3535768Z                  // All of the regions in the type should either have been
2020-03-22T16:48:43.3536343Z                  // erased by writeback, or mapped back to named regions by
2020-03-22T16:48:43.3536818Z                  // borrow checking.
2020-03-22T16:48:43.3549704Z Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustc_trait_selection/opaque_types.rs"` failed.
2020-03-22T16:48:43.3560285Z If you're running `tidy`, try again with `--bless` flag. Or, you just want to format code, run `./x.py fmt` instead.
2020-03-22T16:48:43.3568093Z Build completed unsuccessfully in 0:00:37
2020-03-22T16:48:43.3614540Z == clock drift check ==
2020-03-22T16:48:43.3626742Z   local time: Sun Mar 22 16:48:43 UTC 2020
2020-03-22T16:48:43.9089008Z   network time: Sun, 22 Mar 2020 16:48:43 GMT
2020-03-22T16:48:43.9089008Z   network time: Sun, 22 Mar 2020 16:48:43 GMT
2020-03-22T16:48:43.9089380Z == end clock drift check ==
2020-03-22T16:48:45.5804348Z 
2020-03-22T16:48:45.5896919Z ##[error]Bash exited with code '1'.
2020-03-22T16:48:45.5913079Z ##[section]Finishing: Run build
2020-03-22T16:48:45.5969124Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70277/merge to s
2020-03-22T16:48:45.5974941Z Task         : Get sources
2020-03-22T16:48:45.5975334Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-22T16:48:45.5975717Z Version      : 1.0.0
2020-03-22T16:48:45.5976154Z Author       : Microsoft
2020-03-22T16:48:45.5976154Z Author       : Microsoft
2020-03-22T16:48:45.5976561Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-22T16:48:45.5977047Z ==============================================================================
2020-03-22T16:48:45.9897996Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-22T16:48:45.9952734Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70277/merge to s
2020-03-22T16:48:46.0096177Z Cleaning up task key
2020-03-22T16:48:46.0098034Z Start cleaning up orphan processes.
2020-03-22T16:48:46.0315747Z Terminate orphan process: pid (3457) (python)
2020-03-22T16:48:46.0478588Z ##[section]Finishing: Finalize Job
