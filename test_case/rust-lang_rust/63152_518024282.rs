plain
2019-08-04T17:43:30.6257705Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-04T17:43:30.6445838Z ##[command]git config gc.auto 0
2019-08-04T17:43:30.6523212Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-04T17:43:30.6597272Z ##[command]git config --get-all http.proxy
2019-08-04T17:43:30.6748592Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63152/merge:refs/remotes/pull/63152/merge
---
2019-08-04T17:44:04.9902561Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-04T17:44:04.9902594Z 
2019-08-04T17:44:04.9902814Z   git checkout -b <new-branch-name>
2019-08-04T17:44:04.9902845Z 
2019-08-04T17:44:04.9902917Z HEAD is now at b45650c9e Merge 387dcff796406eb55624c61e9f14a3b5c27ad5ff into 460072ebeed5a2463109894592ac172b47cdfb74
2019-08-04T17:44:05.0065921Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-04T17:44:05.0068359Z ==============================================================================
2019-08-04T17:44:05.0068410Z Task         : Bash
2019-08-04T17:44:05.0068451Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-04T17:53:19.9825902Z     Checking tempfile v3.0.5
2019-08-04T17:53:21.7873903Z error[E0308]: mismatched types
2019-08-04T17:53:21.7874314Z    --> src/librustc_codegen_llvm/context.rs:871:21
2019-08-04T17:53:21.7874531Z     |
2019-08-04T17:53:21.7874784Z 870 |                 match span {
2019-08-04T17:53:21.7875131Z     |                       ---- this match expression has type `syntax_pos::Span`
2019-08-04T17:53:21.7875445Z 871 |                     Some(span) => self.sess().span_fatal(span, &e.to_string()),
2019-08-04T17:53:21.7876270Z     |
2019-08-04T17:53:21.7876648Z     = note: expected type `syntax_pos::Span`
2019-08-04T17:53:21.7876948Z                found type `std::option::Option<_>`
2019-08-04T17:53:21.7876985Z 
2019-08-04T17:53:21.7876985Z 
2019-08-04T17:53:21.7877206Z error[E0308]: mismatched types
2019-08-04T17:53:21.7877475Z    --> src/librustc_codegen_llvm/context.rs:872:21
2019-08-04T17:53:21.7877679Z     |
2019-08-04T17:53:21.7877959Z 872 |                     None => self.sess().fatal(&e.to_string()),
2019-08-04T17:53:21.7878541Z     |
2019-08-04T17:53:21.7879060Z     = note: expected type `syntax_pos::Span`
2019-08-04T17:53:21.7879367Z                found type `std::option::Option<_>`
2019-08-04T17:53:21.7879403Z 
---
2019-08-04T17:53:22.8833816Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--manifest-path" "/checkout/src/librustc_codegen_llvm/Cargo.toml" "--message-format" "json"
2019-08-04T17:53:22.8834225Z expected success, got: exit code: 101
2019-08-04T17:53:22.8852679Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-08-04T17:53:22.8853745Z Build completed unsuccessfully in 0:06:17
2019-08-04T17:53:24.5086613Z ##[error]Bash exited with code '1'.
2019-08-04T17:53:24.5142611Z ##[section]Starting: Checkout
2019-08-04T17:53:24.5144320Z ==============================================================================
2019-08-04T17:53:24.5144387Z Task         : Get sources
2019-08-04T17:53:24.5144431Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
