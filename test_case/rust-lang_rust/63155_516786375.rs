plain
2019-07-31T09:48:21.4225980Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-31T09:48:21.4435447Z ##[command]git config gc.auto 0
2019-07-31T09:48:21.4507864Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-31T09:48:21.4556233Z ##[command]git config --get-all http.proxy
2019-07-31T09:48:21.4688528Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63155/merge:refs/remotes/pull/63155/merge
---
2019-07-31T09:48:58.2591647Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-31T09:48:58.2591679Z 
2019-07-31T09:48:58.2591892Z   git checkout -b <new-branch-name>
2019-07-31T09:48:58.2591922Z 
2019-07-31T09:48:58.2591969Z HEAD is now at ebf2dbd3d Merge 6e4d02369ab30872bf4fa86ed2d2e6f897d0cbd8 into 4a18848e05b0957474fdb5be162502742b5eb9fd
2019-07-31T09:48:58.2747180Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-31T09:48:58.2749639Z ==============================================================================
2019-07-31T09:48:58.2749688Z Task         : Bash
2019-07-31T09:48:58.2749726Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-31T09:55:14.5442351Z    Compiling serde_json v1.0.40
2019-07-31T09:55:18.6824883Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-07-31T09:55:27.0768694Z     Finished release [optimized] target(s) in 1m 27s
2019-07-31T09:55:27.0838234Z tidy check
2019-07-31T09:55:28.2153702Z tidy error: /checkout/src/librustc_target/spec/aarch64_uwp_windows_msvc.rs:14: line longer than 100 chars
2019-07-31T09:55:28.2154765Z tidy error: /checkout/src/librustc_target/spec/x86_64_uwp_windows_msvc.rs:12: line longer than 100 chars
2019-07-31T09:55:28.2154929Z tidy error: /checkout/src/librustc_target/spec/x86_64_uwp_windows_msvc.rs: missing trailing newline
2019-07-31T09:55:28.2155079Z tidy error: /checkout/src/librustc_target/spec/windows_uwp_msvc_base.rs: missing trailing newline
2019-07-31T09:55:28.2155220Z tidy error: /checkout/src/librustc_target/spec/i686_uwp_windows_msvc.rs:12: line longer than 100 chars
2019-07-31T09:55:28.2155370Z tidy error: /checkout/src/librustc_target/spec/i686_uwp_windows_msvc.rs: missing trailing newline
2019-07-31T09:55:28.8696978Z some tidy checks failed
2019-07-31T09:55:28.8701784Z 
2019-07-31T09:55:28.8701784Z 
2019-07-31T09:55:28.8702641Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-07-31T09:55:28.8702807Z 
2019-07-31T09:55:28.8702834Z 
2019-07-31T09:55:28.8710538Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-31T09:55:28.8710665Z Build completed unsuccessfully in 0:01:30
2019-07-31T09:55:28.8710665Z Build completed unsuccessfully in 0:01:30
2019-07-31T09:55:30.3786674Z ##[error]Bash exited with code '1'.
2019-07-31T09:55:30.3819321Z ##[section]Starting: Checkout
2019-07-31T09:55:30.3821325Z ==============================================================================
2019-07-31T09:55:30.3821381Z Task         : Get sources
2019-07-31T09:55:30.3821449Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
