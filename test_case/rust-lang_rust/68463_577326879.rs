plain
2020-01-22T17:17:35.7002748Z ========================== Starting Command Output ===========================
2020-01-22T17:17:35.7005277Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/91335b33-c956-455e-879a-da3bbdf1713c.sh
2020-01-22T17:17:35.7005317Z 
2020-01-22T17:17:35.7008105Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-22T17:17:35.7014835Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68463/merge to s
2020-01-22T17:17:35.7016581Z Task         : Get sources
2020-01-22T17:17:35.7016614Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-22T17:17:35.7016646Z Version      : 1.0.0
2020-01-22T17:17:35.7016721Z Author       : Microsoft
---
2020-01-22T17:17:36.6957383Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-22T17:17:36.6968623Z ##[command]git config gc.auto 0
2020-01-22T17:17:36.6971851Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-22T17:17:36.6973888Z ##[command]git config --get-all http.proxy
2020-01-22T17:17:36.6981419Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68463/merge:refs/remotes/pull/68463/merge
---
2020-01-22T17:29:13.5854899Z configure: build.locked-deps    := True
2020-01-22T17:29:13.5854964Z configure: llvm.ccache          := sccache
2020-01-22T17:29:13.5855227Z configure: build.cargo-native-static := True
2020-01-22T17:29:13.5858566Z configure: dist.missing-tools   := True
2020-01-22T17:29:13.5859261Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-01-22T17:29:13.5859372Z configure: writing `config.toml` in current directory
2020-01-22T17:29:13.5859437Z configure: 
2020-01-22T17:29:13.5859666Z configure: run `python /checkout/x.py --help`
2020-01-22T17:29:13.5859714Z configure: 
---
2020-01-22T17:30:50.2486906Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
2020-01-22T17:30:50.2506593Z Checking std artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
2020-01-22T17:30:50.5451613Z    Compiling cc v1.0.49
2020-01-22T17:30:50.5451966Z     Checking core v0.0.0 (/checkout/src/libcore)
2020-01-22T17:30:53.6380321Z error: unused import: `_mm256_set_epi8`
2020-01-22T17:30:53.6380950Z      |
2020-01-22T17:30:53.6380950Z      |
2020-01-22T17:30:53.6381299Z 1539 |         _mm256_set1_epi8, _mm256_set_epi8, _mm256_setr_epi8, _mm256_setzero_si256,
2020-01-22T17:30:53.6381902Z      |
2020-01-22T17:30:53.6382195Z      = note: `-D unused-imports` implied by `-D warnings`
2020-01-22T17:30:53.6382234Z 
2020-01-22T17:30:56.7126371Z    Compiling libc v0.2.66
---
2020-01-22T17:31:01.7241229Z   local time: Wed Jan 22 17:31:01 UTC 2020
2020-01-22T17:31:01.8791910Z   network time: Wed, 22 Jan 2020 17:31:01 GMT
2020-01-22T17:31:01.8794574Z == end clock drift check ==
2020-01-22T17:31:07.9154899Z 
2020-01-22T17:31:07.9247374Z ##[error]Bash exited with code '1'.
2020-01-22T17:31:07.9259525Z ##[section]Finishing: Run build
2020-01-22T17:31:07.9280880Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68463/merge to s
2020-01-22T17:31:07.9282492Z Task         : Get sources
2020-01-22T17:31:07.9282552Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-22T17:31:07.9282593Z Version      : 1.0.0
2020-01-22T17:31:07.9282630Z Author       : Microsoft
2020-01-22T17:31:07.9282630Z Author       : Microsoft
2020-01-22T17:31:07.9282685Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-22T17:31:07.9282742Z ==============================================================================
2020-01-22T17:31:08.3709492Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-22T17:31:08.3749291Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68463/merge to s
2020-01-22T17:31:08.3876033Z Cleaning up task key
2020-01-22T17:31:08.3877133Z Start cleaning up orphan processes.
2020-01-22T17:31:08.3989879Z Terminate orphan process: pid (3448) (python)
2020-01-22T17:31:08.4739504Z ##[section]Finishing: Finalize Job
