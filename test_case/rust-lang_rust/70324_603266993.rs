plain
2020-03-24T14:20:22.5746953Z ========================== Starting Command Output ===========================
2020-03-24T14:20:22.5752655Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/816a5193-ecf1-4dcc-ba10-09e4a9aeda1f.sh
2020-03-24T14:20:22.5753173Z 
2020-03-24T14:20:22.5758066Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-24T14:20:22.5780154Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70324/merge to s
2020-03-24T14:20:22.5784007Z Task         : Get sources
2020-03-24T14:20:22.5784333Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-24T14:20:22.5784667Z Version      : 1.0.0
2020-03-24T14:20:22.5784886Z Author       : Microsoft
---
2020-03-24T14:20:23.7317355Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-24T14:20:23.7329132Z ##[command]git config gc.auto 0
2020-03-24T14:20:23.7372322Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-24T14:20:23.7377921Z ##[command]git config --get-all http.proxy
2020-03-24T14:20:23.7389360Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70324/merge:refs/remotes/pull/70324/merge
2020-03-24T14:20:24.7170978Z fatal: couldn't find remote ref refs/pull/70324/merge
2020-03-24T14:20:24.7868585Z ##[warning]Git fetch failed with exit code 128, back off 3.531 seconds before retry.
2020-03-24T14:20:27.5344676Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70324/merge:refs/remotes/pull/70324/merge
2020-03-24T14:20:28.1671503Z fatal: couldn't find remote ref refs/pull/70324/merge
2020-03-24T14:20:28.2365632Z ##[warning]Git fetch failed with exit code 128, back off 6.155 seconds before retry.
2020-03-24T14:20:34.3290906Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70324/merge:refs/remotes/pull/70324/merge
2020-03-24T14:20:34.9298282Z fatal: couldn't find remote ref refs/pull/70324/merge
2020-03-24T14:20:34.9768859Z ##[error]Git fetch failed with exit code: 128
2020-03-24T14:20:34.9783606Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70324/merge to s
2020-03-24T14:20:35.0216542Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70324/merge to s
2020-03-24T14:20:35.0222384Z Task         : Get sources
2020-03-24T14:20:35.0222717Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-24T14:20:35.0223051Z Version      : 1.0.0
2020-03-24T14:20:35.0223270Z Author       : Microsoft
2020-03-24T14:20:35.0223270Z Author       : Microsoft
2020-03-24T14:20:35.0223624Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-24T14:20:35.0224022Z ==============================================================================
2020-03-24T14:20:35.3690945Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70324/merge to s
2020-03-24T14:20:35.3802772Z Cleaning up task key
2020-03-24T14:20:35.3805159Z Start cleaning up orphan processes.
2020-03-24T14:20:35.3985764Z ##[section]Finishing: Finalize Job
2020-03-24T14:20:35.4031405Z ##[section]Finishing: Linux x86_64-gnu-tools
