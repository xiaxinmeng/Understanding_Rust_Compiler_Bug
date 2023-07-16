plain
2019-08-26T00:03:26.6088551Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-26T00:03:26.6292091Z ##[command]git config gc.auto 0
2019-08-26T00:03:26.6355363Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-26T00:03:26.6407770Z ##[command]git config --get-all http.proxy
2019-08-26T00:03:26.6533914Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63825/merge:refs/remotes/pull/63825/merge
---
2019-08-26T00:04:01.6262190Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-26T00:04:01.6262489Z 
2019-08-26T00:04:01.6262951Z   git checkout -b <new-branch-name>
2019-08-26T00:04:01.6263261Z 
2019-08-26T00:04:01.6263521Z HEAD is now at ce9430cbf Merge 1c6b98647b9e7cf44d2bbea5497e7ec1bae329f9 into 521d78407471cb78e9bbf47160f6aa23047ac499
2019-08-26T00:04:01.6429693Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-26T00:04:01.6432562Z ==============================================================================
2019-08-26T00:04:01.6432617Z Task         : Bash
2019-08-26T00:04:01.6432658Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-26T00:13:38.1510567Z configure: build.locked-deps    := True
2019-08-26T00:13:38.1510635Z configure: llvm.ccache          := sccache
2019-08-26T00:13:38.1510893Z configure: build.cargo-native-static := True
2019-08-26T00:13:38.1511129Z configure: dist.missing-tools   := True
2019-08-26T00:13:38.1511409Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2019-08-26T00:13:38.1511534Z configure: writing `config.toml` in current directory
2019-08-26T00:13:38.1511582Z configure: 
2019-08-26T00:13:38.1511820Z configure: run `python /checkout/x.py --help`
2019-08-26T00:13:38.1511888Z configure: 
---
2019-08-26T00:22:15.1772440Z    Compiling serde v1.0.99
2019-08-26T00:22:24.1612047Z    Compiling serde_json v1.0.40
2019-08-26T00:22:25.9333716Z    Compiling rustfix v0.4.6
2019-08-26T00:22:29.6875956Z    Compiling compiletest v0.0.0 (/checkout/src/tools/compiletest)
2019-08-26T00:22:29.7226465Z error: expected `;`, found keyword `use`
2019-08-26T00:22:29.7227087Z   |
2019-08-26T00:22:29.7227087Z   |
2019-08-26T00:22:29.7227372Z 4 | use crate::common::{expected_output_path, UI_EXTENSIONS, UI_FIXED, UI_STDERR, UI_STDOUT}
2019-08-26T00:22:29.7227728Z   |                                                                                         - expected `;`
2019-08-26T00:22:29.7227990Z 5 | use crate::{UI_RUN_STDERR, UI_RUN_STDOUT};
2019-08-26T00:22:29.7228333Z 
2019-08-26T00:22:29.7229184Z error: aborting due to previous error
2019-08-26T00:22:29.7229229Z 
2019-08-26T00:22:29.7289733Z error: Could not compile `compiletest`.
---
2019-08-26T00:22:29.7365784Z == clock drift check ==
2019-08-26T00:22:29.7381807Z   local time: Mon Aug 26 00:22:29 UTC 2019
2019-08-26T00:22:30.0165878Z   network time: Mon, 26 Aug 2019 00:22:30 GMT
2019-08-26T00:22:30.0170539Z == end clock drift check ==
2019-08-26T00:22:31.4914643Z ##[error]Bash exited with code '1'.
2019-08-26T00:22:31.4957212Z ##[section]Starting: Checkout
2019-08-26T00:22:31.4958893Z ==============================================================================
2019-08-26T00:22:31.4958943Z Task         : Get sources
2019-08-26T00:22:31.4958986Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
