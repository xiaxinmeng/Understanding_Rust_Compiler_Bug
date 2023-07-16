plain
2020-03-09T09:02:28.4920543Z 
2020-03-09T09:02:42.6614472Z [RUSTC-TIMING] std test:false 14.302
2020-03-09T09:02:42.6635156Z    Compiling rustc-std-workspace-std v1.99.0 (/checkout/src/tools/rustc-std-workspace-std)
2020-03-09T09:02:42.6638172Z    Compiling term v0.0.0 (/checkout/src/libterm)
2020-03-09T09:02:42.6787483Z error: extern location for std does not exist: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/mips64-unknown-linux-muslabi64/release/deps/libstd-bd95e1e545976f16.so
2020-03-09T09:02:42.6788263Z 
2020-03-09T09:02:42.6847280Z error: extern location for std does not exist: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/mips64-unknown-linux-muslabi64/release/deps/libstd-bd95e1e545976f16.so
2020-03-09T09:02:42.6895687Z error: aborting due to previous error
2020-03-09T09:02:42.6896418Z 
2020-03-09T09:02:42.6919228Z [RUSTC-TIMING] rustc_std_workspace_std test:false 0.024
2020-03-09T09:02:42.6920004Z error: could not compile `rustc-std-workspace-std`.
---
2020-03-09T09:02:42.8353610Z   local time: Mon Mar  9 09:02:42 UTC 2020
2020-03-09T09:02:43.3735691Z   network time: Mon, 09 Mar 2020 09:02:43 GMT
2020-03-09T09:02:43.3737256Z == end clock drift check ==
2020-03-09T09:02:45.0232556Z 
2020-03-09T09:02:45.0293610Z ##[error]Bash exited with code '1'.
2020-03-09T09:02:45.0350599Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-03-09T09:02:45.0355131Z ==============================================================================
2020-03-09T09:02:45.0355488Z Task         : Get sources
2020-03-09T09:02:45.0355867Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
