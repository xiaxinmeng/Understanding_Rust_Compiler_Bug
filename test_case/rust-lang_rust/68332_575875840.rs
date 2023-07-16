plain
2020-01-18T07:40:19.8463947Z ========================== Starting Command Output ===========================
2020-01-18T07:40:19.8465436Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/1f7d7eab-dd5b-4302-bd1b-bdd2c8d49b21.sh
2020-01-18T07:40:19.8465468Z 
2020-01-18T07:40:19.8471990Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-18T07:40:19.8478506Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68332/merge to s
2020-01-18T07:40:19.8480987Z Task         : Get sources
2020-01-18T07:40:19.8481041Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-18T07:40:19.8481073Z Version      : 1.0.0
2020-01-18T07:40:19.8481105Z Author       : Microsoft
---
2020-01-18T07:40:20.9115678Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-18T07:40:20.9127743Z ##[command]git config gc.auto 0
2020-01-18T07:40:20.9130396Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-18T07:40:20.9132340Z ##[command]git config --get-all http.proxy
2020-01-18T07:40:20.9139706Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68332/merge:refs/remotes/pull/68332/merge
---
2020-01-18T07:44:28.2761713Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-18T07:44:28.2778499Z Checking std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-18T07:44:28.5452656Z    Compiling cc v1.0.49
2020-01-18T07:44:28.5456914Z     Checking core v0.0.0 (/checkout/src/libcore)
2020-01-18T07:44:28.8815579Z error: `$m:ty` is followed by `with`, which is not allowed for `ty` fragments
2020-01-18T07:44:28.8816371Z    --> src/libcore/ops/function.rs:291:46
2020-01-18T07:44:28.8817183Z     |
2020-01-18T07:44:28.8817507Z 291 |     (impl $imp:ident, $method:ident in $m:ty with $z:ty) => {
2020-01-18T07:44:28.8817887Z     |                                              ^^^^ not allowed after `ty` fragments
2020-01-18T07:44:28.8818150Z     |
2020-01-18T07:44:28.8818480Z     = note: allowed there are: `{`, `[`, `=>`, `,`, `>`, `=`, `:`, `;`, `|`, `as` or `where`
2020-01-18T07:44:36.7014018Z    Compiling libc v0.2.66
2020-01-18T07:44:37.6564662Z    Compiling autocfg v0.1.6
2020-01-18T07:44:39.2157112Z error: aborting due to previous error
2020-01-18T07:44:39.2157252Z 
---
2020-01-18T07:44:39.5705366Z   local time: Sat Jan 18 07:44:39 UTC 2020
2020-01-18T07:44:39.6639520Z   network time: Sat, 18 Jan 2020 07:44:39 GMT
2020-01-18T07:44:39.6639605Z == end clock drift check ==
2020-01-18T07:44:45.0683068Z 
2020-01-18T07:44:45.0794385Z ##[error]Bash exited with code '1'.
2020-01-18T07:44:45.0807964Z ##[section]Finishing: Run build
2020-01-18T07:44:45.0822896Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68332/merge to s
2020-01-18T07:44:45.0824667Z Task         : Get sources
2020-01-18T07:44:45.0824734Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-18T07:44:45.0824775Z Version      : 1.0.0
2020-01-18T07:44:45.0824812Z Author       : Microsoft
2020-01-18T07:44:45.0824812Z Author       : Microsoft
2020-01-18T07:44:45.0824873Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-18T07:44:45.0824917Z ==============================================================================
2020-01-18T07:44:45.4993363Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-18T07:44:45.5039491Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68332/merge to s
2020-01-18T07:44:45.5153976Z Cleaning up task key
2020-01-18T07:44:45.5154900Z Start cleaning up orphan processes.
2020-01-18T07:44:45.5273303Z Terminate orphan process: pid (3985) (python)
2020-01-18T07:44:45.5517829Z ##[section]Finishing: Finalize Job
