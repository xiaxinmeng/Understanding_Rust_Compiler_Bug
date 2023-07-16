plain
2020-01-18T10:42:53.0102637Z ========================== Starting Command Output ===========================
2020-01-18T10:42:53.0108007Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/73b43233-8d91-4b14-9d3f-51be264ac635.sh
2020-01-18T10:42:53.0108137Z 
2020-01-18T10:42:53.0120998Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-18T10:42:53.0132801Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68338/merge to s
2020-01-18T10:42:53.0135684Z Task         : Get sources
2020-01-18T10:42:53.0135722Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-18T10:42:53.0135758Z Version      : 1.0.0
2020-01-18T10:42:53.0135794Z Author       : Microsoft
---
2020-01-18T10:42:54.0532701Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-18T10:42:54.0546218Z ##[command]git config gc.auto 0
2020-01-18T10:42:54.0549062Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-18T10:42:54.0551473Z ##[command]git config --get-all http.proxy
2020-01-18T10:42:54.0558749Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68338/merge:refs/remotes/pull/68338/merge
---
2020-01-18T10:54:09.7526803Z configure: build.locked-deps    := True
2020-01-18T10:54:09.7527511Z configure: llvm.ccache          := sccache
2020-01-18T10:54:09.7527951Z configure: build.cargo-native-static := True
2020-01-18T10:54:09.7528953Z configure: dist.missing-tools   := True
2020-01-18T10:54:09.7529405Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-01-18T10:54:09.7529672Z configure: writing `config.toml` in current directory
2020-01-18T10:54:09.7529723Z configure: 
2020-01-18T10:54:09.7530113Z configure: run `python /checkout/x.py --help`
2020-01-18T10:54:09.7531172Z configure: 
---
2020-01-18T10:56:14.9723417Z     Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2020-01-18T10:56:16.0869849Z error[E0428]: the name `MAIN_SEP` is defined multiple times
2020-01-18T10:56:16.0871164Z   --> src/libstd/sys/windows/path.rs:97:1
2020-01-18T10:56:16.0872112Z    |
2020-01-18T10:56:16.0872993Z 94 | pub const MAIN_SEP: char = '/';
2020-01-18T10:56:16.0873807Z    | ------------------------------- previous definition of the value `MAIN_SEP` here
2020-01-18T10:56:16.0874430Z ...
2020-01-18T10:56:16.0875087Z 97 | pub const MAIN_SEP: char = '\\';
2020-01-18T10:56:16.0875804Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `MAIN_SEP` redefined here
2020-01-18T10:56:16.0877398Z    = note: `MAIN_SEP` must be defined only once in the value namespace of this module
2020-01-18T10:56:16.0877741Z 
2020-01-18T10:56:18.5712310Z error: aborting due to previous error
2020-01-18T10:56:18.5713398Z 
---
2020-01-18T10:56:18.5936195Z   local time: Sat Jan 18 10:56:18 UTC 2020
2020-01-18T10:56:18.8753022Z   network time: Sat, 18 Jan 2020 10:56:18 GMT
2020-01-18T10:56:18.8764263Z == end clock drift check ==
2020-01-18T10:56:23.8798409Z 
2020-01-18T10:56:23.8917343Z ##[error]Bash exited with code '1'.
2020-01-18T10:56:23.8930275Z ##[section]Finishing: Run build
2020-01-18T10:56:23.8946514Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68338/merge to s
2020-01-18T10:56:23.8948571Z Task         : Get sources
2020-01-18T10:56:23.8948618Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-18T10:56:23.8948703Z Version      : 1.0.0
2020-01-18T10:56:23.8948744Z Author       : Microsoft
2020-01-18T10:56:23.8948744Z Author       : Microsoft
2020-01-18T10:56:23.8948788Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-18T10:56:23.8948854Z ==============================================================================
2020-01-18T10:56:24.3577817Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-18T10:56:24.3621111Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68338/merge to s
2020-01-18T10:56:24.3758854Z Cleaning up task key
2020-01-18T10:56:24.3759777Z Start cleaning up orphan processes.
2020-01-18T10:56:24.3905873Z Terminate orphan process: pid (3562) (python)
2020-01-18T10:56:24.4164397Z ##[section]Finishing: Finalize Job
