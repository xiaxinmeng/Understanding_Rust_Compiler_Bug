plain
2020-02-11T12:40:12.1762407Z    Compiling rustbook v0.1.0 (/checkout/src/tools/rustbook)
2020-02-11T12:40:18.2947592Z [RUSTC-TIMING] rustbook test:false 6.112
2020-02-11T12:40:18.3014574Z     Finished release [optimized] target(s) in 12m 36s
2020-02-11T12:40:18.3241692Z [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "rustbook", path: "src/tools/rustbook", mode: ToolBootstrap, is_optional_tool: false, source_type: InTree, extra_features: ["linkcheck"] } -- 756.091
2020-02-11T12:42:05.3394697Z Unsuccessful server response for link `https://ieeexplore.ieee.org/document/6650903`
2020-02-11T12:42:05.3423893Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-11T12:42:05.3424597Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2020-02-11T12:42:05.3611596Z Building stage2 tool clippy-driver (x86_64-unknown-linux-gnu)
2020-02-11T12:42:05.6455708Z    Compiling proc-macro2 v0.4.30
---
2020-02-11T13:06:28.5210495Z This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
2020-02-11T13:06:28.5210552Z 
2020-02-11T13:06:28.5210990Z We detected that this PR updated 'clippy-driver', but its tests failed.
2020-02-11T13:06:28.5211096Z 
2020-02-11T13:06:28.5211374Z If you do intend to update 'clippy-driver', please check the error messages above and
2020-02-11T13:06:28.5211451Z commit another update.
2020-02-11T13:06:28.5211518Z 
2020-02-11T13:06:28.5211755Z If you do NOT intend to update 'clippy-driver', please ensure you did not accidentally
2020-02-11T13:06:28.5212018Z change the submodule at 'src/tools/clippy'. You may ask your reviewer for the
2020-02-11T13:06:28.5212089Z proper steps.
2020-02-11T13:06:28.5220993Z Build completed unsuccessfully in 0:00:01
2020-02-11T13:06:28.5271661Z == clock drift check ==
2020-02-11T13:06:28.5289122Z   local time: Tue Feb 11 13:06:28 UTC 2020
2020-02-11T13:06:28.5787128Z   network time: Tue, 11 Feb 2020 13:06:28 GMT
2020-02-11T13:06:28.5787128Z   network time: Tue, 11 Feb 2020 13:06:28 GMT
2020-02-11T13:06:28.5791224Z == end clock drift check ==
2020-02-11T13:06:29.0780028Z 
2020-02-11T13:06:29.0876574Z ##[error]Bash exited with code '1'.
2020-02-11T13:06:29.0921697Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-02-11T13:06:29.0923530Z ==============================================================================
2020-02-11T13:06:29.0923774Z Task         : Get sources
2020-02-11T13:06:29.0923845Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
