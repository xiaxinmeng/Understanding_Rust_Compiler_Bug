plain
2019-10-06T12:27:49.6166899Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-06T12:27:49.6357883Z ##[command]git config gc.auto 0
2019-10-06T12:27:49.6440965Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-06T12:27:49.6491602Z ##[command]git config --get-all http.proxy
2019-10-06T12:27:49.6628021Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65161/merge:refs/remotes/pull/65161/merge
---
2019-10-06T12:38:00.1185956Z configure: build.locked-deps    := True
2019-10-06T12:38:00.1186006Z configure: llvm.ccache          := sccache
2019-10-06T12:38:00.1186236Z configure: build.cargo-native-static := True
2019-10-06T12:38:00.1186445Z configure: dist.missing-tools   := True
2019-10-06T12:38:00.1186703Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2019-10-06T12:38:00.1186830Z configure: writing `config.toml` in current directory
2019-10-06T12:38:00.1186877Z configure: 
2019-10-06T12:38:00.1189296Z configure: run `python /checkout/x.py --help`
2019-10-06T12:38:00.1189385Z configure: 
---
2019-10-06T12:42:14.9056038Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-10-06T12:42:16.2638892Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-10-06T12:42:28.9877747Z     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-10-06T12:42:37.9214802Z error[E0308]: mismatched types
2019-10-06T12:42:37.9216600Z     --> src/librustc/dep_graph/graph.rs:1093:57
2019-10-06T12:42:37.9217530Z      |
2019-10-06T12:42:37.9218083Z 1093 | ...                   let source = graph.data[source].node;
2019-10-06T12:42:37.9219128Z      |                                               |
2019-10-06T12:42:37.9219128Z      |                                               |
2019-10-06T12:42:37.9219664Z      |                                               expected reference, found struct `dep_graph::graph::DepNodeIndex`
2019-10-06T12:42:37.9220222Z      |                                               help: consider borrowing here: `&source`
2019-10-06T12:42:37.9221410Z      = note: expected type `&_`
2019-10-06T12:42:37.9221410Z      = note: expected type `&_`
2019-10-06T12:42:37.9222106Z                 found type `dep_graph::graph::DepNodeIndex`
2019-10-06T12:42:37.9222414Z 
2019-10-06T12:42:38.1320640Z error[E0609]: no field `node` on type `dep_graph::graph::DepNodeData`
2019-10-06T12:42:38.1322322Z     --> src/librustc/dep_graph/graph.rs:1093:65
2019-10-06T12:42:38.1323098Z      |
2019-10-06T12:42:38.1325543Z 1093 | ...                   let source = graph.data[source].node;
2019-10-06T12:42:38.1326954Z      |
2019-10-06T12:42:38.1326954Z      |
2019-10-06T12:42:38.1327722Z      = note: available fields are: `edges`, `fingerprint`
2019-10-06T12:42:52.3921647Z error: aborting due to 2 previous errors
2019-10-06T12:42:52.3922678Z 
2019-10-06T12:42:52.3923241Z Some errors have detailed explanations: E0308, E0609.
2019-10-06T12:42:52.3923745Z For more information about an error, try `rustc --explain E0308`.
---
2019-10-06T12:42:52.5834431Z == clock drift check ==
2019-10-06T12:42:52.5854302Z   local time: Sun Oct  6 12:42:52 UTC 2019
2019-10-06T12:42:52.7350869Z   network time: Sun, 06 Oct 2019 12:42:52 GMT
2019-10-06T12:42:52.7353138Z == end clock drift check ==
2019-10-06T12:42:53.5955017Z ##[error]Bash exited with code '1'.
2019-10-06T12:42:53.6019309Z ##[section]Starting: Checkout
2019-10-06T12:42:53.6020996Z ==============================================================================
2019-10-06T12:42:53.6021038Z Task         : Get sources
2019-10-06T12:42:53.6021075Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
