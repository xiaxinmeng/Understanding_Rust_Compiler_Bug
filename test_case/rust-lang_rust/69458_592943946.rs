plain
2020-02-29T11:54:41.0297103Z ========================== Starting Command Output ===========================
2020-02-29T11:54:41.0299556Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/d9b6d399-9205-4dba-a479-e6fe5730d964.sh
2020-02-29T11:54:41.0299814Z 
2020-02-29T11:54:41.0303314Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-29T11:54:41.0322179Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69458/merge to s
2020-02-29T11:54:41.0326065Z Task         : Get sources
2020-02-29T11:54:41.0326334Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-29T11:54:41.0326593Z Version      : 1.0.0
2020-02-29T11:54:41.0326768Z Author       : Microsoft
---
2020-02-29T11:54:44.3080044Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-29T11:54:44.3089590Z ##[command]git config gc.auto 0
2020-02-29T11:54:44.3097458Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-29T11:54:44.3104142Z ##[command]git config --get-all http.proxy
2020-02-29T11:54:44.3112552Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69458/merge:refs/remotes/pull/69458/merge
---
2020-02-29T12:05:19.6399420Z     Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
2020-02-29T12:05:20.1646437Z error[E0412]: cannot find type `Path` in this scope
2020-02-29T12:05:20.1647876Z    --> src/librustdoc/test.rs:210:35
2020-02-29T12:05:20.1648799Z     |
2020-02-29T12:05:20.1649848Z 210 |     visited_tests: &Mutex<HashMap<Path, usize>>,
2020-02-29T12:05:20.1652410Z     |
2020-02-29T12:05:20.1653353Z help: possible candidates are found in other modules, you can import them into scope
2020-02-29T12:05:20.1654272Z     |
2020-02-29T12:05:20.1655490Z 1   | use crate::clean::types::Path;
---
2020-02-29T12:05:20.1660897Z 1   | use syntax::ast::Path;
2020-02-29T12:05:20.1661790Z     |
2020-02-29T12:05:20.1662622Z help: you might be missing a type parameter
2020-02-29T12:05:20.1663423Z     |
2020-02-29T12:05:20.1665154Z 194 | fn run_test<Path>(
2020-02-29T12:05:20.1666626Z 
2020-02-29T12:05:20.1666626Z 
2020-02-29T12:05:22.5324872Z error[E0282]: type annotations needed for `std::sync::Mutex<std::collections::HashMap<K, V>>`
2020-02-29T12:05:22.5326443Z     |
2020-02-29T12:05:22.5327217Z 723 |         let test_number = Mutex::new(HashMap::new());
2020-02-29T12:05:22.5328456Z     |             -----------              ^^^^^^^^^^^^ cannot infer type for type parameter `K`
2020-02-29T12:05:22.5329395Z     |             |
2020-02-29T12:05:22.5329395Z     |             |
2020-02-29T12:05:22.5330889Z     |             consider giving `test_number` the explicit type `std::sync::Mutex<std::collections::HashMap<K, V>>`, where the type parameter `K` is specified
2020-02-29T12:05:22.6088034Z error: aborting due to 2 previous errors
2020-02-29T12:05:22.6093551Z 
2020-02-29T12:05:22.6106545Z Some errors have detailed explanations: E0282, E0412.
2020-02-29T12:05:22.6107273Z For more information about an error, try `rustc --explain E0282`.
---
2020-02-29T12:05:22.6313097Z   local time: Sat Feb 29 12:05:22 UTC 2020
2020-02-29T12:05:22.7872908Z   network time: Sat, 29 Feb 2020 12:05:22 GMT
2020-02-29T12:05:22.7875187Z == end clock drift check ==
2020-02-29T12:05:23.3938226Z 
2020-02-29T12:05:23.4009270Z ##[error]Bash exited with code '1'.
2020-02-29T12:05:23.4021183Z ##[section]Finishing: Run build
2020-02-29T12:05:23.4070333Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69458/merge to s
2020-02-29T12:05:23.4074801Z Task         : Get sources
2020-02-29T12:05:23.4075101Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-29T12:05:23.4075535Z Version      : 1.0.0
2020-02-29T12:05:23.4075752Z Author       : Microsoft
2020-02-29T12:05:23.4075752Z Author       : Microsoft
2020-02-29T12:05:23.4076079Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-29T12:05:23.4076433Z ==============================================================================
2020-02-29T12:05:23.7663477Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-29T12:05:23.7714449Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69458/merge to s
2020-02-29T12:05:23.7814591Z Cleaning up task key
2020-02-29T12:05:23.7816166Z Start cleaning up orphan processes.
2020-02-29T12:05:23.8019496Z Terminate orphan process: pid (3901) (python)
2020-02-29T12:05:23.8354898Z ##[section]Finishing: Finalize Job
