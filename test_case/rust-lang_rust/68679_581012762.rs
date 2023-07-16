plain
2020-02-01T09:20:16.4718726Z ========================== Starting Command Output ===========================
2020-02-01T09:20:16.4720242Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/a1e3cd31-960e-48ae-af1c-366ece855376.sh
2020-02-01T09:20:16.4720321Z 
2020-02-01T09:20:16.4722914Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-01T09:20:16.4729228Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68679/merge to s
2020-02-01T09:20:16.4730741Z Task         : Get sources
2020-02-01T09:20:16.4730775Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-01T09:20:16.4730852Z Version      : 1.0.0
2020-02-01T09:20:16.4730884Z Author       : Microsoft
---
2020-02-01T09:20:17.4744823Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-01T09:20:17.4758245Z ##[command]git config gc.auto 0
2020-02-01T09:20:17.4767092Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-01T09:20:17.4771364Z ##[command]git config --get-all http.proxy
2020-02-01T09:20:17.4787084Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68679/merge:refs/remotes/pull/68679/merge
---
2020-02-01T09:27:58.9280926Z     Checking rustc_ty v0.0.0 (/checkout/src/librustc_ty)
2020-02-01T09:27:59.0427831Z error[E0425]: cannot find value `upvar_ty` in this scope
2020-02-01T09:27:59.0428151Z    --> src/librustc_ty/needs_drop.rs:114:40
2020-02-01T09:27:59.0428343Z     |
2020-02-01T09:27:59.0428637Z 114 | ...                   queue_type(upvar_ty);
2020-02-01T09:27:59.0428996Z 
2020-02-01T09:27:59.0448213Z error[E0425]: cannot find value `upvar_ty` in this scope
2020-02-01T09:27:59.0448490Z    --> src/librustc_ty/needs_drop.rs:126:40
2020-02-01T09:27:59.0448697Z     |
2020-02-01T09:27:59.0448697Z     |
2020-02-01T09:27:59.0448973Z 126 | ...                   queue_type(upvar_ty);
2020-02-01T09:27:59.0449310Z 
2020-02-01T09:27:59.1237928Z error[E0057]: this function takes 2 parameters but 1 parameter was supplied
2020-02-01T09:27:59.1238240Z   --> src/librustc_ty/needs_drop.rs:97:29
2020-02-01T09:27:59.1238441Z    |
2020-02-01T09:27:59.1238441Z    |
2020-02-01T09:27:59.1238663Z 97 | ...                   queue_type(upvar_ty);
2020-02-01T09:27:59.1239580Z 
2020-02-01T09:27:59.1277785Z error[E0057]: this function takes 2 parameters but 1 parameter was supplied
2020-02-01T09:27:59.1278097Z    --> src/librustc_ty/needs_drop.rs:114:29
2020-02-01T09:27:59.1278315Z     |
2020-02-01T09:27:59.1278315Z     |
2020-02-01T09:27:59.1278551Z 114 | ...                   queue_type(upvar_ty);
2020-02-01T09:27:59.1278886Z 
2020-02-01T09:27:59.1307333Z error[E0057]: this function takes 2 parameters but 1 parameter was supplied
2020-02-01T09:27:59.1307617Z    --> src/librustc_ty/needs_drop.rs:126:29
2020-02-01T09:27:59.1307850Z     |
2020-02-01T09:27:59.1307850Z     |
2020-02-01T09:27:59.1308143Z 126 | ...                   queue_type(upvar_ty);
2020-02-01T09:27:59.1308512Z 
2020-02-01T09:27:59.1973336Z error: aborting due to 5 previous errors
2020-02-01T09:27:59.1973437Z 
2020-02-01T09:27:59.1977275Z Some errors have detailed explanations: E0057, E0425.
2020-02-01T09:27:59.1977275Z Some errors have detailed explanations: E0057, E0425.
2020-02-01T09:27:59.1984633Z For more information about an error, try `rustc --explain E0057`.
2020-02-01T09:27:59.2025162Z error: could not compile `rustc_ty`.
2020-02-01T09:27:59.2025468Z warning: build failed, waiting for other jobs to finish...
2020-02-01T09:28:31.0642726Z error: build failed
2020-02-01T09:28:31.0665517Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-02-01T09:28:31.0677928Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-02-01T09:28:31.0678201Z Build completed unsuccessfully in 0:05:59
2020-02-01T09:28:31.0727905Z == clock drift check ==
2020-02-01T09:28:31.0741716Z   local time: Sat Feb  1 09:28:31 UTC 2020
2020-02-01T09:28:31.0741716Z   local time: Sat Feb  1 09:28:31 UTC 2020
2020-02-01T09:28:31.3552758Z   network time: Sat, 01 Feb 2020 09:28:31 GMT
2020-02-01T09:28:31.3557640Z == end clock drift check ==
2020-02-01T09:28:31.6794050Z 
2020-02-01T09:28:31.6891507Z ##[error]Bash exited with code '1'.
2020-02-01T09:28:31.6906524Z ##[section]Finishing: Run build
2020-02-01T09:28:31.6922332Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68679/merge to s
2020-02-01T09:28:31.6924697Z Task         : Get sources
2020-02-01T09:28:31.6924745Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-01T09:28:31.6924810Z Version      : 1.0.0
2020-02-01T09:28:31.6924854Z Author       : Microsoft
2020-02-01T09:28:31.6924854Z Author       : Microsoft
2020-02-01T09:28:31.6924902Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-01T09:28:31.6924951Z ==============================================================================
2020-02-01T09:28:32.0942560Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-01T09:28:32.0987310Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68679/merge to s
2020-02-01T09:28:32.1140083Z Cleaning up task key
2020-02-01T09:28:32.1140861Z Start cleaning up orphan processes.
2020-02-01T09:28:32.1244075Z Terminate orphan process: pid (6008) (python)
2020-02-01T09:28:32.1478700Z ##[section]Finishing: Finalize Job
