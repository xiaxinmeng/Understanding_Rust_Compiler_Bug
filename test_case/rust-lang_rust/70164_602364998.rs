plain
2020-03-23T02:08:25.9731921Z ========================== Starting Command Output ===========================
2020-03-23T02:08:25.9736963Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/09b95faf-ad63-4c22-b173-8f2cbd023ab4.sh
2020-03-23T02:08:25.9737437Z 
2020-03-23T02:08:25.9741852Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-23T02:08:25.9760890Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70164/merge to s
2020-03-23T02:08:25.9764083Z Task         : Get sources
2020-03-23T02:08:25.9764380Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-23T02:08:25.9764682Z Version      : 1.0.0
2020-03-23T02:08:25.9764880Z Author       : Microsoft
---
2020-03-23T02:08:26.9720948Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-23T02:08:26.9727484Z ##[command]git config gc.auto 0
2020-03-23T02:08:26.9731815Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-23T02:08:26.9736507Z ##[command]git config --get-all http.proxy
2020-03-23T02:08:26.9743691Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70164/merge:refs/remotes/pull/70164/merge
---
2020-03-23T02:19:04.7647286Z     Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
2020-03-23T02:19:06.6180994Z error[E0308]: mismatched types
2020-03-23T02:19:06.6181889Z    --> src/librustdoc/clean/auto_trait.rs:320:9
2020-03-23T02:19:06.6182480Z     |
2020-03-23T02:19:06.6183180Z 316 |       ) -> FxHashSet<GenericParamDef> {
2020-03-23T02:19:06.6184775Z     |            -------------------------- expected `std::collections::HashSet<clean::types::GenericParamDef, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>` because of return type
2020-03-23T02:19:06.6185847Z ...
2020-03-23T02:19:06.6186858Z 320 | /         regions.into_iter().flat_map(|r| {
2020-03-23T02:19:06.6187824Z 321 | |             match r {
2020-03-23T02:19:06.6188895Z 322 | |                 // We only care about late bound regions, as we need to add them
2020-03-23T02:19:06.6189950Z 323 | |                 // to the 'for<>' section
2020-03-23T02:19:06.6191692Z 332 | |             }
2020-03-23T02:19:06.6192524Z 333 | |         })
2020-03-23T02:19:06.6193747Z     | |__________^ expected struct `std::collections::HashSet`, found struct `std::iter::FlatMap`
2020-03-23T02:19:06.6194639Z     |
2020-03-23T02:19:06.6194639Z     |
2020-03-23T02:19:06.6195765Z     = note: expected struct `std::collections::HashSet<clean::types::GenericParamDef, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>`
2020-03-23T02:19:06.6197460Z                found struct `std::iter::FlatMap<std::collections::hash_set::IntoIter<&rustc::ty::RegionKind>, std::option::Option<clean::types::GenericParamDef>, [closure@src/librustdoc/clean/auto_trait.rs:320:38: 333:10]>`
2020-03-23T02:19:08.2384200Z error: aborting due to previous error
2020-03-23T02:19:08.2384501Z 
2020-03-23T02:19:08.2385023Z For more information about this error, try `rustc --explain E0308`.
2020-03-23T02:19:08.2518914Z error: could not compile `rustdoc`.
---
2020-03-23T02:19:08.2635417Z   local time: Mon Mar 23 02:19:08 UTC 2020
2020-03-23T02:19:08.5483854Z   network time: Mon, 23 Mar 2020 02:19:08 GMT
2020-03-23T02:19:08.5484305Z == end clock drift check ==
2020-03-23T02:19:09.0763891Z 
2020-03-23T02:19:09.0846474Z ##[error]Bash exited with code '1'.
2020-03-23T02:19:09.0859761Z ##[section]Finishing: Run build
2020-03-23T02:19:09.0907576Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70164/merge to s
2020-03-23T02:19:09.0912554Z Task         : Get sources
2020-03-23T02:19:09.0912916Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-23T02:19:09.0913246Z Version      : 1.0.0
2020-03-23T02:19:09.0913462Z Author       : Microsoft
2020-03-23T02:19:09.0913462Z Author       : Microsoft
2020-03-23T02:19:09.0913829Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-23T02:19:09.0914234Z ==============================================================================
2020-03-23T02:19:09.4424433Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-23T02:19:09.4470041Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70164/merge to s
2020-03-23T02:19:09.4557285Z Cleaning up task key
2020-03-23T02:19:09.4563611Z Start cleaning up orphan processes.
2020-03-23T02:19:09.4784733Z Terminate orphan process: pid (3711) (python)
2020-03-23T02:19:09.5181518Z ##[section]Finishing: Finalize Job
