plain
2020-03-17T19:48:02.7723503Z ========================== Starting Command Output ===========================
2020-03-17T19:48:02.7725981Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/060dd11d-aab5-47c1-8a64-6adc593d0acc.sh
2020-03-17T19:48:02.7726332Z 
2020-03-17T19:48:02.7730717Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-17T19:48:02.7750374Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70081/merge to s
2020-03-17T19:48:02.7754320Z Task         : Get sources
2020-03-17T19:48:02.7754625Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-17T19:48:02.7754944Z Version      : 1.0.0
2020-03-17T19:48:02.7755144Z Author       : Microsoft
---
2020-03-17T19:48:03.7661394Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-17T19:48:03.7668452Z ##[command]git config gc.auto 0
2020-03-17T19:48:03.7673427Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-17T19:48:03.7678048Z ##[command]git config --get-all http.proxy
2020-03-17T19:48:03.7685444Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70081/merge:refs/remotes/pull/70081/merge
---
2020-03-17T20:33:00.7891940Z    Compiling rustc_resolve v0.0.0 (/checkout/src/librustc_resolve)
2020-03-17T20:33:01.2365716Z error: unnecessary parentheses around method argument
2020-03-17T20:33:01.2366438Z     --> src/librustc_resolve/lib.rs:2806:28
2020-03-17T20:33:01.2366989Z      |
2020-03-17T20:33:01.2367812Z 2806 |                     .chain({ path_str.split("::").skip(1).map(Ident::from_str) })
2020-03-17T20:33:01.2369846Z      |
2020-03-17T20:33:01.2370679Z      = note: `-D unused-parens` implied by `-D warnings`
2020-03-17T20:33:01.2376264Z 
2020-03-17T20:33:02.7906779Z error: aborting due to previous error
---
2020-03-17T20:35:51.3591733Z   local time: Tue Mar 17 20:35:51 UTC 2020
2020-03-17T20:35:51.6487082Z   network time: Tue, 17 Mar 2020 20:35:51 GMT
2020-03-17T20:35:51.6494301Z == end clock drift check ==
2020-03-17T20:35:52.5492295Z 
2020-03-17T20:35:52.5560131Z ##[error]Bash exited with code '1'.
2020-03-17T20:35:52.5575660Z ##[section]Finishing: Run build
2020-03-17T20:35:52.5623306Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70081/merge to s
2020-03-17T20:35:52.5628315Z Task         : Get sources
2020-03-17T20:35:52.5628664Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-17T20:35:52.5628984Z Version      : 1.0.0
2020-03-17T20:35:52.5629225Z Author       : Microsoft
2020-03-17T20:35:52.5629225Z Author       : Microsoft
2020-03-17T20:35:52.5629581Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-17T20:35:52.5629992Z ==============================================================================
2020-03-17T20:35:52.9036567Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-17T20:35:52.9082465Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70081/merge to s
2020-03-17T20:35:52.9168798Z Cleaning up task key
2020-03-17T20:35:52.9170046Z Start cleaning up orphan processes.
2020-03-17T20:35:52.9374765Z Terminate orphan process: pid (3687) (python)
2020-03-17T20:35:52.9540009Z ##[section]Finishing: Finalize Job
