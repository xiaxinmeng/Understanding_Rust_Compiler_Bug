plain
2020-02-19T16:06:14.4258135Z ========================== Starting Command Output ===========================
2020-02-19T16:06:14.4295156Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/f2672492-244e-4e7c-89ba-7c05bab6e5a0.sh
2020-02-19T16:06:14.4295260Z 
2020-02-19T16:06:14.4297979Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-19T16:06:14.4303973Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69201/merge to s
2020-02-19T16:06:14.4305653Z Task         : Get sources
2020-02-19T16:06:14.4305688Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-19T16:06:14.4305723Z Version      : 1.0.0
2020-02-19T16:06:14.4305758Z Author       : Microsoft
---
2020-02-19T16:06:15.2833440Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-19T16:06:15.2936260Z ##[command]git config gc.auto 0
2020-02-19T16:06:15.3000888Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-19T16:06:15.3072873Z ##[command]git config --get-all http.proxy
2020-02-19T16:06:15.3205112Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69201/merge:refs/remotes/pull/69201/merge
---
2020-02-19T16:11:49.1750071Z    Compiling serde_json v1.0.40
2020-02-19T16:11:50.2881613Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2020-02-19T16:11:59.3565872Z     Finished release [optimized] target(s) in 1m 21s
2020-02-19T16:11:59.3652037Z tidy check
2020-02-19T16:11:59.9046588Z tidy error: /checkout/src/test/ui/if-attrs/gate-whole-expr.rs:4: tab character
2020-02-19T16:11:59.9047073Z tidy error: /checkout/src/test/ui/if-attrs/gate-whole-expr.rs:6: tab character
2020-02-19T16:11:59.9047394Z tidy error: /checkout/src/test/ui/if-attrs/gate-whole-expr.rs:7: tab character
2020-02-19T16:11:59.9047703Z tidy error: /checkout/src/test/ui/if-attrs/gate-whole-expr.rs:8: tab character
2020-02-19T16:11:59.9048022Z tidy error: /checkout/src/test/ui/if-attrs/gate-whole-expr.rs:9: tab character
2020-02-19T16:11:59.9048346Z tidy error: /checkout/src/test/ui/if-attrs/gate-whole-expr.rs:10: tab character
2020-02-19T16:11:59.9048655Z tidy error: /checkout/src/test/ui/if-attrs/gate-whole-expr.rs:11: tab character
2020-02-19T16:11:59.9048976Z tidy error: /checkout/src/test/ui/if-attrs/gate-whole-expr.rs:12: tab character
2020-02-19T16:11:59.9049284Z tidy error: /checkout/src/test/ui/if-attrs/gate-whole-expr.rs:13: tab character
2020-02-19T16:11:59.9050064Z tidy error: /checkout/src/test/ui/if-attrs/gate-whole-expr.rs:14: tab character
2020-02-19T16:12:01.8422230Z some tidy checks failed
2020-02-19T16:12:01.8422372Z Found 487 error codes
2020-02-19T16:12:01.8422437Z Found 0 error codes with no tests
2020-02-19T16:12:01.8422520Z Done!
2020-02-19T16:12:01.8422520Z Done!
2020-02-19T16:12:01.8422557Z 
2020-02-19T16:12:01.8422590Z 
2020-02-19T16:12:01.8423604Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-02-19T16:12:01.8423741Z 
2020-02-19T16:12:01.8423772Z 
2020-02-19T16:12:01.8423848Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-02-19T16:12:01.8423913Z Build completed unsuccessfully in 0:01:31
2020-02-19T16:12:01.8423913Z Build completed unsuccessfully in 0:01:31
2020-02-19T16:12:01.8467495Z == clock drift check ==
2020-02-19T16:12:01.8488385Z   local time: Wed Feb 19 16:12:01 UTC 2020
2020-02-19T16:12:02.0045624Z   network time: Wed, 19 Feb 2020 16:12:02 GMT
2020-02-19T16:12:02.0051506Z == end clock drift check ==
2020-02-19T16:12:02.7155115Z 
2020-02-19T16:12:02.7245306Z ##[error]Bash exited with code '1'.
2020-02-19T16:12:02.7259893Z ##[section]Finishing: Run build
2020-02-19T16:12:02.7273691Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69201/merge to s
2020-02-19T16:12:02.7275407Z Task         : Get sources
2020-02-19T16:12:02.7275476Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-19T16:12:02.7275526Z Version      : 1.0.0
2020-02-19T16:12:02.7275569Z Author       : Microsoft
2020-02-19T16:12:02.7275569Z Author       : Microsoft
2020-02-19T16:12:02.7275633Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-19T16:12:02.7275687Z ==============================================================================
2020-02-19T16:12:03.1035820Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-19T16:12:03.1075712Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69201/merge to s
2020-02-19T16:12:03.1180888Z Cleaning up task key
2020-02-19T16:12:03.1181809Z Start cleaning up orphan processes.
2020-02-19T16:12:03.1285956Z Terminate orphan process: pid (4010) (python)
2020-02-19T16:12:03.1461964Z ##[section]Finishing: Finalize Job
