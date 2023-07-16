plain
2019-12-18T17:22:17.0393797Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-18T17:22:17.0625096Z ##[command]git config gc.auto 0
2019-12-18T17:22:17.0691707Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-18T17:22:17.0751934Z ##[command]git config --get-all http.proxy
2019-12-18T17:22:17.0909329Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67402/merge:refs/remotes/pull/67402/merge
---
2019-12-18T17:33:12.4334702Z configure: build.locked-deps    := True
2019-12-18T17:33:12.4334750Z configure: llvm.ccache          := sccache
2019-12-18T17:33:12.4334955Z configure: build.cargo-native-static := True
2019-12-18T17:33:12.4335176Z configure: dist.missing-tools   := True
2019-12-18T17:33:12.4335612Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2019-12-18T17:33:12.4335734Z configure: writing `config.toml` in current directory
2019-12-18T17:33:12.4335783Z configure: 
2019-12-18T17:33:12.4336038Z configure: run `python /checkout/x.py --help`
2019-12-18T17:33:12.4336090Z configure: 
---
2019-12-18T17:42:12.6375955Z   local time: Wed Dec 18 17:42:12 UTC 2019
2019-12-18T17:42:12.7873900Z   network time: Wed, 18 Dec 2019 17:42:12 GMT
2019-12-18T17:42:12.7877490Z == end clock drift check ==
2019-12-18T17:42:14.2555208Z 
2019-12-18T17:42:14.2663769Z ##[error]Bash exited with code '1'.
2019-12-18T17:42:14.2705622Z ##[section]Starting: Checkout
2019-12-18T17:42:14.2708141Z ==============================================================================
2019-12-18T17:42:14.2708201Z Task         : Get sources
2019-12-18T17:42:14.2708250Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
