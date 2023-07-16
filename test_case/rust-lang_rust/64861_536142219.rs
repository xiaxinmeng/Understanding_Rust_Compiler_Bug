plain
2019-09-28T01:42:09.4200348Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-28T01:42:09.4371077Z ##[command]git config gc.auto 0
2019-09-28T01:42:09.4449989Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-28T01:42:09.4501303Z ##[command]git config --get-all http.proxy
2019-09-28T01:42:10.0172727Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64861/merge:refs/remotes/pull/64861/merge
---
2019-09-28T01:52:23.0899110Z configure: build.locked-deps    := True
2019-09-28T01:52:23.0899180Z configure: llvm.ccache          := sccache
2019-09-28T01:52:23.0899403Z configure: build.cargo-native-static := True
2019-09-28T01:52:23.0899652Z configure: dist.missing-tools   := True
2019-09-28T01:52:23.0899949Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2019-09-28T01:52:23.0900057Z configure: writing `config.toml` in current directory
2019-09-28T01:52:23.0900122Z configure: 
2019-09-28T01:52:23.0900376Z configure: run `python /checkout/x.py --help`
2019-09-28T01:52:23.0900425Z configure: 
---
2019-09-28T01:54:41.9225365Z     Checking backtrace v0.3.37
2019-09-28T01:54:44.0131786Z     Checking rustc-std-workspace-alloc v1.99.0 (/checkout/src/tools/rustc-std-workspace-alloc)
2019-09-28T01:54:44.0164990Z     Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2019-09-28T01:54:44.0573254Z     Checking hashbrown v0.5.0
2019-09-28T01:54:49.1371625Z error[E0599]: no method named `should_be_line_buffered` found for type `sys::windows::stdio::Stdout` in the current scope
2019-09-28T01:54:49.1372785Z    |
2019-09-28T01:54:49.1372785Z    |
2019-09-28T01:54:49.1373056Z 89 |         self.0.should_be_line_buffered()
2019-09-28T01:54:49.1373645Z    |         |      |
2019-09-28T01:54:49.1373957Z    |         |      this is an associated function, not a method
2019-09-28T01:54:49.1373957Z    |         |      this is an associated function, not a method
2019-09-28T01:54:49.1374872Z    |         help: use associated function syntax instead: `sys::windows::stdio::Stdout::should_be_line_buffered`
2019-09-28T01:54:49.1375353Z   ::: src/libstd/sys/windows/stdio.rs:17:1
2019-09-28T01:54:49.1375588Z    |
2019-09-28T01:54:49.1376018Z 17 | pub struct Stdout;
2019-09-28T01:54:49.1376018Z 17 | pub struct Stdout;
2019-09-28T01:54:49.1376361Z    | ------------------ method `should_be_line_buffered` not found for this
2019-09-28T01:54:49.1378004Z    |
2019-09-28T01:54:49.1379004Z    = note: found the following associated functions; to be used as methods, functions must have a `self` parameter
2019-09-28T01:54:49.1379321Z note: the candidate is defined in an impl for the type `sys::windows::stdio::Stdout`
2019-09-28T01:54:49.1380122Z    |
2019-09-28T01:54:49.1380122Z    |
2019-09-28T01:54:49.1380379Z 249|     pub fn should_be_line_buffered() -> bool {
2019-09-28T01:54:49.1380737Z 
2019-09-28T01:54:50.1286748Z error: aborting due to previous error
2019-09-28T01:54:50.1287697Z 
2019-09-28T01:54:50.1288821Z For more information about this error, try `rustc --explain E0599`.
---
2019-09-28T01:54:50.1793221Z == clock drift check ==
2019-09-28T01:54:50.1848753Z   local time: Sat Sep 28 01:54:50 UTC 2019
2019-09-28T01:54:50.3339875Z   network time: Sat, 28 Sep 2019 01:54:50 GMT
2019-09-28T01:54:50.3340267Z == end clock drift check ==
2019-09-28T01:54:54.8557548Z ##[error]Bash exited with code '1'.
2019-09-28T01:54:54.8600628Z ##[section]Starting: Checkout
2019-09-28T01:54:54.8603267Z ==============================================================================
2019-09-28T01:54:54.8603316Z Task         : Get sources
2019-09-28T01:54:54.8603379Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
