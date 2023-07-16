plain
2019-08-02T17:07:31.8515014Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-02T17:07:31.8722332Z ##[command]git config gc.auto 0
2019-08-02T17:07:32.5605307Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-02T17:07:32.5612832Z ##[command]git config --get-all http.proxy
2019-08-02T17:07:32.5616719Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63162/merge:refs/remotes/pull/63162/merge
---
2019-08-02T17:08:06.7061830Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-02T17:08:06.7063698Z 
2019-08-02T17:08:06.7065291Z   git checkout -b <new-branch-name>
2019-08-02T17:08:06.7066681Z 
2019-08-02T17:08:06.7068120Z HEAD is now at fb45f85e2 Merge 77028562fc02b3dd2222adbaec94daa9feba1f2f into 1df512fcaeaf17639c5d28a3045814d6f7a7db97
2019-08-02T17:08:06.7197092Z ##[section]Finishing: Checkout
2019-08-02T17:08:06.7204803Z ##[section]Starting: Decide whether to run this job
2019-08-02T17:08:06.7207764Z Task         : Bash
2019-08-02T17:08:06.7207809Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-08-02T17:08:06.7208015Z Version      : 3.151.3
2019-08-02T17:08:06.7208080Z Author       : Microsoft Corporation
2019-08-02T17:08:06.7208080Z Author       : Microsoft Corporation
2019-08-02T17:08:06.7208129Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-08-02T17:08:06.7208179Z ==============================================================================
2019-08-02T17:08:06.8694040Z Generating script.
2019-08-02T17:08:06.8757987Z ========================== Starting Command Output ===========================
2019-08-02T17:08:06.8760645Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/0e55f6ab-d227-456c-aa18-eed2e83ddd36.sh
2019-08-02T17:08:07.2700778Z Executing the job since submodules are updated
2019-08-02T17:08:07.2809642Z ##[section]Finishing: Decide whether to run this job
2019-08-02T17:08:07.2826009Z ==============================================================================
2019-08-02T17:08:07.2826136Z Task         : Bash
2019-08-02T17:08:07.2827673Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-08-02T17:08:07.2827719Z Version      : 3.151.3
---
2019-08-02T17:11:25.0923215Z  ---> da247f97cae5
2019-08-02T17:11:25.0965896Z Successfully built da247f97cae5
2019-08-02T17:11:25.1785026Z Successfully tagged rust-ci:latest
2019-08-02T17:11:25.2446350Z Built container sha256:da247f97cae5eae5b83c5c729161a428910c2334b8e4b7c36a44141f6436df9e
2019-08-02T17:11:25.2462831Z Uploading finished image to https://rust-lang-ci-sccache2.s3.amazonaws.com/docker/0a94e800dbdbef90fbb9cf381d041a9386910340a5cdd758b34797988dea18bff1c9fc0562ea46a76a5a5ba9a8bbf4046ce3ddf0fb0b70448e54334a8af215fe
2019-08-02T17:12:06.0676108Z upload failed: - to s3://rust-lang-ci-sccache2/docker/0a94e800dbdbef90fbb9cf381d041a9386910340a5cdd758b34797988dea18bff1c9fc0562ea46a76a5a5ba9a8bbf4046ce3ddf0fb0b70448e54334a8af215fe Unable to locate credentials
2019-08-02T17:12:07.0522142Z [CI_JOB_NAME=LinuxTools]
2019-08-02T17:12:07.0572725Z Starting sccache server...
2019-08-02T17:12:07.1538739Z configure: processing command line
2019-08-02T17:12:07.1539523Z configure: 
---
2019-08-02T19:00:40.7248694Z 
2019-08-02T19:00:40.7249344Z stderr ----
2019-08-02T19:00:40.7249526Z Error: there are broken links
2019-08-02T19:00:40.7249955Z  Caused By: "https://github.com/rust-lang/compiler-team/blob/master/experts/MAP.md" returned 404 Not Found
2019-08-02T19:00:40.7250850Z  Caused By: "***/tree/master/src/test/run-pass/" returned 404 Not Found
2019-08-02T19:00:40.7251596Z  Caused By: There was an error while fetching "http://aturon.github.io/blog/2017/04/24/negative-chalk/", http://aturon.github.io/blog/2017/04/24/negative-chalk/: timed out
2019-08-02T19:00:40.7252530Z  Caused By: There was an error while fetching "http://rust-lang.github.io/rfcs/1211-mir.html", http://rust-lang.github.io/rfcs/1211-mir.html: timed out
2019-08-02T19:00:40.7253842Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/rustc/mir/enum.Place.html" returned 404 Not Found
2019-08-02T19:00:40.7254282Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/rustc/mir/enum.Place.html" returned 404 Not Found
2019-08-02T19:00:40.7254689Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/rustc/mir/enum.Place.html" returned 404 Not Found
2019-08-02T19:00:40.7255819Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/rustc/mir/enum.Place.html" returned 404 Not Found
2019-08-02T19:00:40.7255819Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/rustc/mir/enum.Place.html" returned 404 Not Found
2019-08-02T19:00:40.7256369Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/rustc/mir/enum.Place.html" returned 404 Not Found
2019-08-02T19:00:40.7256834Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/rustc/mir/enum.Place.html" returned 404 Not Found
2019-08-02T19:00:40.7257323Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/rustc/mir/enum.Place.html" returned 404 Not Found
2019-08-02T19:00:40.7257797Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/rustc/mir/enum.Place.html" returned 404 Not Found
2019-08-02T19:00:40.7258265Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/rustc/mir/enum.Place.html" returned 404 Not Found
2019-08-02T19:00:40.7258799Z  Caused By: There was an error while fetching "http://rust-lang.github.io/rfcs/2094-nll.html", http://rust-lang.github.io/rfcs/2094-nll.html: timed out
2019-08-02T19:00:40.7259387Z 
2019-08-02T19:00:40.7259707Z 
2019-08-02T19:00:40.7260096Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-02T19:00:40.7260535Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2019-08-02T19:27:37.1806931Z      |
2019-08-02T19:27:37.1813521Z 1269 | #[bench]
2019-08-02T19:27:37.1819166Z      |   ^^^^^
2019-08-02T19:27:37.1824766Z      |
2019-08-02T19:27:37.1830453Z      = note: for more information, see ***/issues/50297
2019-08-02T19:27:37.1896485Z 
2019-08-02T19:27:37.1896815Z error[E0658]: use of unstable library feature 'test': `bench` is a part of custom test frameworks which are unstable
2019-08-02T19:27:37.1899223Z     --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_data_structures-542.0.0/bit_set.rs:1284:3
2019-08-02T19:27:37.1899738Z      |
2019-08-02T19:27:37.1899738Z      |
2019-08-02T19:27:37.1900058Z 1284 | #[bench]
2019-08-02T19:27:37.1900310Z      |   ^^^^^
2019-08-02T19:27:37.1900542Z      |
2019-08-02T19:27:37.1900918Z      = note: for more information, see ***/issues/50297
2019-08-02T19:27:37.1901316Z 
2019-08-02T19:27:37.1901630Z error[E0658]: use of unstable library feature 'test': `bench` is a part of custom test frameworks which are unstable
2019-08-02T19:27:37.1901963Z     --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_data_structures-542.0.0/bit_set.rs:1302:3
2019-08-02T19:27:37.1902206Z      |
2019-08-02T19:27:37.1902206Z      |
2019-08-02T19:27:37.1902791Z 1302 | #[bench]
2019-08-02T19:27:37.1903022Z      |   ^^^^^
2019-08-02T19:27:37.1903233Z      |
2019-08-02T19:27:37.1903536Z      = note: for more information, see ***/issues/50297
2019-08-02T19:27:37.1903882Z 
2019-08-02T19:27:37.1904489Z error[E0658]: use of unstable library feature 'test': `bench` is a part of custom test frameworks which are unstable
2019-08-02T19:27:37.1904951Z     --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_data_structures-542.0.0/bit_set.rs:1320:3
2019-08-02T19:27:37.1905141Z      |
2019-08-02T19:27:37.1905141Z      |
2019-08-02T19:27:37.1905354Z 1320 | #[bench]
2019-08-02T19:27:37.1905575Z      |   ^^^^^
2019-08-02T19:27:37.1905752Z      |
2019-08-02T19:27:37.1906024Z      = note: for more information, see ***/issues/50297
2019-08-02T19:27:37.1906333Z 
2019-08-02T19:27:37.1906598Z error[E0658]: use of unstable library feature 'test': `bench` is a part of custom test frameworks which are unstable
2019-08-02T19:27:37.1906893Z     --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_data_structures-542.0.0/bit_set.rs:1335:3
2019-08-02T19:27:37.1907081Z      |
2019-08-02T19:27:37.1907081Z      |
2019-08-02T19:27:37.1907293Z 1335 | #[bench]
2019-08-02T19:27:37.1907517Z      |   ^^^^^
2019-08-02T19:27:37.1907691Z      |
2019-08-02T19:27:37.1907984Z      = note: for more information, see ***/issues/50297
2019-08-02T19:27:37.1908366Z 
2019-08-02T19:27:39.0078505Z error: aborting due to 5 previous errors
2019-08-02T19:27:39.0082895Z 
2019-08-02T19:27:39.0088117Z For more information about this error, try `rustc --explain E0658`.
---
2019-08-02T19:27:49.5261446Z      |
2019-08-02T19:27:49.5261740Z 1269 | #[bench]
2019-08-02T19:27:49.5261993Z      |   ^^^^^
2019-08-02T19:27:49.5262209Z      |
2019-08-02T19:27:49.5262958Z      = note: for more information, see ***/issues/50297
2019-08-02T19:27:49.5263302Z 
2019-08-02T19:27:49.5263615Z error[E0658]: use of unstable library feature 'test': `bench` is a part of custom test frameworks which are unstable
2019-08-02T19:27:49.5263937Z     --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_data_structures-542.0.0/bit_set.rs:1284:3
2019-08-02T19:27:49.5264143Z      |
2019-08-02T19:27:49.5264143Z      |
2019-08-02T19:27:49.5264388Z 1284 | #[bench]
2019-08-02T19:27:49.5264613Z      |   ^^^^^
2019-08-02T19:27:49.5264806Z      |
2019-08-02T19:27:49.5265141Z      = note: for more information, see ***/issues/50297
2019-08-02T19:27:49.5265460Z 
2019-08-02T19:27:49.5265822Z error[E0658]: use of unstable library feature 'test': `bench` is a part of custom test frameworks which are unstable
2019-08-02T19:27:49.5266320Z     --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_data_structures-542.0.0/bit_set.rs:1302:3
2019-08-02T19:27:49.5266551Z      |
2019-08-02T19:27:49.5266551Z      |
2019-08-02T19:27:49.5266787Z 1302 | #[bench]
2019-08-02T19:27:49.5267023Z      |   ^^^^^
2019-08-02T19:27:49.5267238Z      |
2019-08-02T19:27:49.5267831Z      = note: for more information, see ***/issues/50297
2019-08-02T19:27:49.5268178Z 
2019-08-02T19:27:49.5268476Z error[E0658]: use of unstable library feature 'test': `bench` is a part of custom test frameworks which are unstable
2019-08-02T19:27:49.5268896Z     --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_data_structures-542.0.0/bit_set.rs:1320:3
2019-08-02T19:27:49.5269167Z      |
2019-08-02T19:27:49.5269167Z      |
2019-08-02T19:27:49.5269406Z 1320 | #[bench]
2019-08-02T19:27:49.5270373Z      |   ^^^^^
2019-08-02T19:27:49.5270611Z      |
2019-08-02T19:27:49.5270949Z      = note: for more information, see ***/issues/50297
2019-08-02T19:27:49.5271329Z 
2019-08-02T19:27:49.5271640Z error[E0658]: use of unstable library feature 'test': `bench` is a part of custom test frameworks which are unstable
2019-08-02T19:27:49.5271993Z     --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_data_structures-542.0.0/bit_set.rs:1335:3
2019-08-02T19:27:49.5272213Z      |
2019-08-02T19:27:49.5272213Z      |
2019-08-02T19:27:49.5272455Z 1335 | #[bench]
2019-08-02T19:27:49.5272712Z      |   ^^^^^
2019-08-02T19:27:49.5273591Z      |
2019-08-02T19:27:49.5274161Z      = note: for more information, see ***/issues/50297
2019-08-02T19:27:49.5274872Z 
2019-08-02T19:27:51.2720543Z error: aborting due to 5 previous errors
2019-08-02T19:27:51.2721375Z 
2019-08-02T19:27:51.2721983Z For more information about this error, try `rustc --explain E0658`.
---
2019-08-02T19:28:11.3027478Z Verifying status of clippy-driver...
2019-08-02T19:28:11.3038183Z Verifying status of miri...
2019-08-02T19:28:11.3049777Z This PR updated 'src/tools/miri', verifying if status is 'test-pass'...
2019-08-02T19:28:11.3062054Z 
2019-08-02T19:28:11.3069697Z ⚠️ We detected that this PR updated 'miri', but its tests failed.
2019-08-02T19:28:11.3069804Z 
2019-08-02T19:28:11.3070355Z If you do intend to update 'miri', please check the error messages above and
2019-08-02T19:28:11.3070410Z commit another update.
2019-08-02T19:28:11.3070437Z 
2019-08-02T19:28:11.3070673Z If you do NOT intend to update 'miri', please ensure you did not accidentally
2019-08-02T19:28:11.3071325Z change the submodule at 'src/tools/miri'. You may ask your reviewer for the
2019-08-02T19:28:11.3071382Z proper steps.
2019-08-02T19:28:12.2583702Z ##[error]Bash exited with code '3'.
2019-08-02T19:28:12.2652694Z ##[section]Starting: Checkout
2019-08-02T19:28:12.2654825Z ==============================================================================
2019-08-02T19:28:12.2654893Z Task         : Get sources
2019-08-02T19:28:12.2654936Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
