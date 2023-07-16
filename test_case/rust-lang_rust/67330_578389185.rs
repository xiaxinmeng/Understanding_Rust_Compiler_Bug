plain
2020-01-25T08:48:25.3252412Z ========================== Starting Command Output ===========================
2020-01-25T08:48:25.3254226Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/22e2f8ba-5c29-4274-a65d-cc8c7f04dec0.sh
2020-01-25T08:48:25.3254268Z 
2020-01-25T08:48:25.3257510Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-25T08:48:25.3264997Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67330/merge to s
2020-01-25T08:48:25.3267052Z Task         : Get sources
2020-01-25T08:48:25.3267089Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-25T08:48:25.3267127Z Version      : 1.0.0
2020-01-25T08:48:25.3267222Z Author       : Microsoft
---
2020-01-25T08:48:26.3021119Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-25T08:48:26.3039284Z ##[command]git config gc.auto 0
2020-01-25T08:48:26.3043047Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-25T08:48:26.3045845Z ##[command]git config --get-all http.proxy
2020-01-25T08:48:26.3080163Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67330/merge:refs/remotes/pull/67330/merge
---
2020-01-25T08:52:24.4991968Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-25T08:52:24.5005168Z Checking std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-25T08:52:24.7679276Z    Compiling cc v1.0.50
2020-01-25T08:52:24.7679664Z     Checking core v0.0.0 (/checkout/src/libcore)
2020-01-25T08:52:24.8875478Z error: expected one of `.`, `?`, `{`, or an operator, found `;`
2020-01-25T08:52:24.8877419Z      |
2020-01-25T08:52:24.8877419Z      |
2020-01-25T08:52:24.8878297Z 3789 |         let idx = match idx_opt.map(|idx| idx + 1).unwrap_or(self.v.len());
2020-01-25T08:52:24.8878810Z      |                   -----                                                   ^ expected one of `.`, `?`, `{`, or an operator
2020-01-25T08:52:24.8879381Z      |                   while parsing this match expression
2020-01-25T08:52:24.8879668Z      |                   help: try removing this `match`
2020-01-25T08:52:24.8879707Z 
2020-01-25T08:52:24.8879707Z 
2020-01-25T08:52:24.8917826Z error: expected one of `.`, `?`, `{`, or an operator, found `;`
2020-01-25T08:52:24.8918499Z      |
2020-01-25T08:52:24.8918499Z      |
2020-01-25T08:52:24.8918863Z 3814 |         let idx = match idx_opt.map(|idx| idx + 1).unwrap_or(self.v.len());
2020-01-25T08:52:24.8919374Z      |                   -----                                                   ^ expected one of `.`, `?`, `{`, or an operator
2020-01-25T08:52:24.8920062Z      |                   while parsing this match expression
2020-01-25T08:52:24.8920430Z      |                   help: try removing this `match`
2020-01-25T08:52:24.8920678Z 
2020-01-25T08:52:24.8920678Z 
2020-01-25T08:52:24.8956874Z error: expected one of `.`, `?`, `{`, or an operator, found `;`
2020-01-25T08:52:24.8957488Z      |
2020-01-25T08:52:24.8957488Z      |
2020-01-25T08:52:24.8957868Z 3978 |         let idx = match idx_opt.map(|idx| idx + 1).unwrap_or(self.v.len());
2020-01-25T08:52:24.8958595Z      |                   -----                                                   ^ expected one of `.`, `?`, `{`, or an operator
2020-01-25T08:52:24.8959304Z      |                   while parsing this match expression
2020-01-25T08:52:24.8959652Z      |                   help: try removing this `match`
2020-01-25T08:52:24.8959720Z 
2020-01-25T08:52:24.8959720Z 
2020-01-25T08:52:24.9036950Z error: expected one of `.`, `?`, `{`, or an operator, found `;`
2020-01-25T08:52:24.9037585Z      |
2020-01-25T08:52:24.9037585Z      |
2020-01-25T08:52:24.9037982Z 4012 |         let idx = match idx_opt.map(|idx| idx + 1).unwrap_or(self.v.len());
2020-01-25T08:52:24.9038472Z      |                   -----                                                   ^ expected one of `.`, `?`, `{`, or an operator
2020-01-25T08:52:24.9039180Z      |                   while parsing this match expression
2020-01-25T08:52:24.9039546Z      |                   help: try removing this `match`
2020-01-25T08:52:24.9039595Z 
2020-01-25T08:52:31.9618002Z    Compiling libc v0.2.66
---
2020-01-25T08:52:34.4959806Z   local time: Sat Jan 25 08:52:34 UTC 2020
2020-01-25T08:52:34.7608517Z   network time: Sat, 25 Jan 2020 08:52:34 GMT
2020-01-25T08:52:34.7609217Z == end clock drift check ==
2020-01-25T08:52:42.4763566Z 
2020-01-25T08:52:42.4891528Z ##[error]Bash exited with code '1'.
2020-01-25T08:52:42.4903515Z ##[section]Finishing: Run build
2020-01-25T08:52:42.4919615Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67330/merge to s
2020-01-25T08:52:42.4921678Z Task         : Get sources
2020-01-25T08:52:42.4921722Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-25T08:52:42.4921788Z Version      : 1.0.0
2020-01-25T08:52:42.4921825Z Author       : Microsoft
2020-01-25T08:52:42.4921825Z Author       : Microsoft
2020-01-25T08:52:42.4922044Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-25T08:52:42.4922114Z ==============================================================================
2020-01-25T08:52:42.8720965Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-25T08:52:42.8791582Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/67330/merge to s
2020-01-25T08:52:42.8923479Z Cleaning up task key
2020-01-25T08:52:42.8924391Z Start cleaning up orphan processes.
2020-01-25T08:52:42.9086224Z Terminate orphan process: pid (4838) (python)
2020-01-25T08:52:42.9300421Z ##[section]Finishing: Finalize Job
