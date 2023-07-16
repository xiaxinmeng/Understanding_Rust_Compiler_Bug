plain
2019-10-06T00:34:54.1805174Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-06T00:34:54.2006760Z ##[command]git config gc.auto 0
2019-10-06T00:34:54.2090056Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-06T00:34:54.2137451Z ##[command]git config --get-all http.proxy
2019-10-06T00:34:54.2275969Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65146/merge:refs/remotes/pull/65146/merge
---
2019-10-06T00:44:57.4168824Z configure: build.locked-deps    := True
2019-10-06T00:44:57.4168873Z configure: llvm.ccache          := sccache
2019-10-06T00:44:57.4169122Z configure: build.cargo-native-static := True
2019-10-06T00:44:57.4169346Z configure: dist.missing-tools   := True
2019-10-06T00:44:57.4169607Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2019-10-06T00:44:57.4169742Z configure: writing `config.toml` in current directory
2019-10-06T00:44:57.4169789Z configure: 
2019-10-06T00:44:57.4170070Z configure: run `python /checkout/x.py --help`
2019-10-06T00:44:57.4170120Z configure: 
---
2019-10-06T00:49:01.7063060Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-10-06T00:49:03.0770053Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-10-06T00:49:15.8445622Z     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-10-06T00:49:24.9989295Z error[E0308]: mismatched types
2019-10-06T00:49:24.9990876Z     --> src/librustc/dep_graph/graph.rs:1095:57
2019-10-06T00:49:24.9991394Z      |
2019-10-06T00:49:24.9991902Z 1095 | ...                   let source = graph.data[source].node;
2019-10-06T00:49:24.9992942Z      |                                               |
2019-10-06T00:49:24.9992942Z      |                                               |
2019-10-06T00:49:24.9993497Z      |                                               expected reference, found struct `dep_graph::graph::DepNodeIndex`
2019-10-06T00:49:24.9994047Z      |                                               help: consider borrowing here: `&source`
2019-10-06T00:49:24.9995089Z      = note: expected type `&_`
2019-10-06T00:49:24.9995089Z      = note: expected type `&_`
2019-10-06T00:49:24.9995590Z                 found type `dep_graph::graph::DepNodeIndex`
2019-10-06T00:49:40.3652616Z error: aborting due to previous error
2019-10-06T00:49:40.3653746Z 
2019-10-06T00:49:40.3655356Z For more information about this error, try `rustc --explain E0308`.
2019-10-06T00:49:40.5573602Z error: could not compile `rustc`.
---
2019-10-06T00:49:40.5688336Z == clock drift check ==
2019-10-06T00:49:40.5721221Z   local time: Sun Oct  6 00:49:40 UTC 2019
2019-10-06T00:49:40.7214187Z   network time: Sun, 06 Oct 2019 00:49:40 GMT
2019-10-06T00:49:40.7214315Z == end clock drift check ==
2019-10-06T00:49:42.0062871Z ##[error]Bash exited with code '1'.
2019-10-06T00:49:42.0109674Z ##[section]Starting: Checkout
2019-10-06T00:49:42.0111316Z ==============================================================================
2019-10-06T00:49:42.0111363Z Task         : Get sources
2019-10-06T00:49:42.0111402Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
