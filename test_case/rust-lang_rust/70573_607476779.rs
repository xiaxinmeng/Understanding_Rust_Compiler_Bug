plain
2020-04-01T20:16:16.8238072Z ========================== Starting Command Output ===========================
2020-04-01T20:16:16.8241036Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/8c5c089b-aece-410b-bdbe-eb253ab7b8a5.sh
2020-04-01T20:16:16.8241368Z 
2020-04-01T20:16:16.8245949Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-01T20:16:16.8265715Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70573/merge to s
2020-04-01T20:16:16.8268179Z Task         : Get sources
2020-04-01T20:16:16.8268398Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-01T20:16:16.8268612Z Version      : 1.0.0
2020-04-01T20:16:16.8268775Z Author       : Microsoft
---
2020-04-01T20:16:17.8373002Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-01T20:16:17.8377731Z ##[command]git config gc.auto 0
2020-04-01T20:16:17.8381579Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-01T20:16:17.8385138Z ##[command]git config --get-all http.proxy
2020-04-01T20:16:17.8390261Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70573/merge:refs/remotes/pull/70573/merge
---
2020-04-01T20:18:22.9591721Z Looks like docker image is the same as before, not uploading
2020-04-01T20:18:30.4974712Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-01T20:18:30.5217997Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-01T20:18:30.5233199Z == clock drift check ==
2020-04-01T20:18:30.5233616Z   local time: Wed Apr  1 20:18:30 UTC 2020
2020-04-01T20:18:30.6168086Z   network time: Wed, 01 Apr 2020 20:18:30 GMT
2020-04-01T20:18:30.6210888Z Starting sccache server...
2020-04-01T20:18:30.6839842Z configure: processing command line
2020-04-01T20:18:30.6842195Z configure: 
2020-04-01T20:18:30.6843617Z configure: rust.dist-src        := False
---
2020-04-01T20:22:21.5045456Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-01T20:22:22.5412371Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-01T20:22:23.6756228Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-01T20:22:23.9100845Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-01T20:22:30.8381144Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-01T20:22:31.9793804Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-01T20:22:35.1554624Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-01T20:22:38.1774746Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-01T20:22:45.6453926Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-01T20:35:42.8729082Z     Finished release [optimized] target(s) in 15m 09s
2020-04-01T20:35:42.9118476Z Copying stage0 rustc from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
2020-04-01T20:35:42.9143644Z Assembling stage1 compiler (x86_64-unknown-linux-gnu)
2020-04-01T20:35:42.9155455Z Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-01T20:35:43.2797629Z error: process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc -vV` (signal: 4, SIGILL: illegal instruction)
2020-04-01T20:35:43.2843484Z --- stdout
2020-04-01T20:35:43.2844171Z rustc 1.44.0-nightly (e2d30a08f 2020-04-01)
2020-04-01T20:35:43.2844645Z --- stderr
2020-04-01T20:35:43.2844645Z --- stderr
2020-04-01T20:35:43.2845347Z thread 'main' panicked at 'start drain index (is 0) should be <= end drain index (is 44)', <::core::macros::panic macros>:5:50
2020-04-01T20:35:43.2846128Z 
2020-04-01T20:35:43.2846364Z error: internal compiler error: unexpected panic
2020-04-01T20:35:43.2846562Z 
2020-04-01T20:35:43.2846562Z 
2020-04-01T20:35:43.2847108Z thread 'main' panicked at 'already borrowed: BorrowMutError', /checkout/src/libcore/cell.rs:878:9
2020-04-01T20:35:43.2847473Z stack backtrace:
2020-04-01T20:35:43.2848175Z    0:     0x7feffbb7e844 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h3449fa2911d0e00b
2020-04-01T20:35:43.2848896Z    1:     0x7feffbbb8f6c - core::fmt::write::h9783e269500109c4
2020-04-01T20:35:43.2849518Z    2:     0x7feffbb703b3 - std::io::Write::write_fmt::h5f88762f892c2bb1
2020-04-01T20:35:43.2850212Z    3:     0x7feffbb83665 - std::panicking::default_hook::{{closure}}::h495780587ab3c9c8
2020-04-01T20:35:43.2850879Z    4:     0x7feffbb833a2 - std::panicking::default_hook::h9c5f3eeaba1fe53e
2020-04-01T20:35:43.2851521Z    5:     0x7feffc19b52c - rustc_driver::report_ice::hd311707a60c517ef
2020-04-01T20:35:43.2852181Z    6:     0x7feffbb83e55 - std::panicking::rust_panic_with_hook::h5b3da46c48d6bea7
2020-04-01T20:35:43.2852818Z    7:     0x7feffbb8396b - rust_begin_unwind
2020-04-01T20:35:43.2853349Z    8:     0x7feffbbb5c01 - core::panicking::panic_fmt::hfcab76b0a642f537
2020-04-01T20:35:43.2853939Z    9:     0x7feffbbb5873 - core::result::unwrap_failed::h16a0a551966b2e8f
2020-04-01T20:35:43.2854605Z   10:     0x7feffbb6d802 - <std::io::stdio::StdoutLock as std::io::Write>::write::h751bc403c22ceeea
2020-04-01T20:35:43.2855257Z   11:     0x7feffbb6f960 - std::io::Write::write_all::h0bcc23ccbae0371c
2020-04-01T20:35:43.2855975Z   12:     0x7feffbb705f1 - <std::io::Write::write_fmt::Adaptor<T> as core::fmt::Write>::write_str::h7644e60d5ab9e34d
2020-04-01T20:35:43.2856619Z   13:     0x7feffbbb8f6c - core::fmt::write::h9783e269500109c4
2020-04-01T20:35:43.2857277Z   14:     0x7feffbb6d377 - <std::io::stdio::Stdout as std::io::Write>::write_fmt::hdbf5f9d097b1a80d
2020-04-01T20:35:43.2857908Z   15:     0x7feffbb6ed91 - std::io::stdio::_print::h0ded95bb5063c8ba
2020-04-01T20:35:43.2858476Z   16:     0x7feffc198954 - rustc_driver::version::h13d193f2bbd0e171
2020-04-01T20:35:43.2859069Z   17:     0x7feffc19afaa - rustc_driver::handle_options::hca3218ef71495153
2020-04-01T20:35:43.2859652Z   18:     0x7feffc19b904 - rustc_driver::report_ice::hd311707a60c517ef
2020-04-01T20:35:43.2860260Z   19:     0x7feffbb83e55 - std::panicking::rust_panic_with_hook::h5b3da46c48d6bea7
2020-04-01T20:35:43.2860788Z   20:     0x7feffbb8396b - rust_begin_unwind
2020-04-01T20:35:43.2861308Z   21:     0x7feffbbb5c01 - core::panicking::panic_fmt::hfcab76b0a642f537
2020-04-01T20:35:43.2861966Z   22:     0x7feffbb5f563 - alloc::vec::Vec<T>::start_lte_end_drain_assert_failed::hba8c4d5d4e770964
2020-04-01T20:35:43.2862664Z   23:     0x7feffbb6a237 - std::io::buffered::BufWriter<W>::flush_buf::h50d339e4a1963de1
2020-04-01T20:35:43.2863362Z   24:     0x7feffbb6d6b8 - <std::io::stdio::StdoutLock as std::io::Write>::write::h751bc403c22ceeea
2020-04-01T20:35:43.2863998Z   25:     0x7feffbb6f960 - std::io::Write::write_all::h0bcc23ccbae0371c
2020-04-01T20:35:43.2864723Z   26:     0x7feffbb705f1 - <std::io::Write::write_fmt::Adaptor<T> as core::fmt::Write>::write_str::h7644e60d5ab9e34d
2020-04-01T20:35:43.2865509Z   27:     0x7feffbbb8fcf - core::fmt::write::h9783e269500109c4
2020-04-01T20:35:43.2869197Z   28:     0x7feffbb6d377 - <std::io::stdio::Stdout as std::io::Write>::write_fmt::hdbf5f9d097b1a80d
2020-04-01T20:35:43.2869849Z   29:     0x7feffbb6ed91 - std::io::stdio::_print::h0ded95bb5063c8ba
2020-04-01T20:35:43.2870725Z   30:     0x7feffc198954 - rustc_driver::version::h13d193f2bbd0e171
2020-04-01T20:35:43.2871372Z   31:     0x7feffc19afaa - rustc_driver::handle_options::hca3218ef71495153
2020-04-01T20:35:43.2872009Z   32:     0x7feffc1947e8 - rustc_driver::run_compiler::h79cf86f923d25a63
2020-04-01T20:35:43.2873979Z   33:     0x7feffc1a7b57 - <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once::h37bdf725e36929bd
2020-04-01T20:35:43.2875360Z   34:     0x7feffc1ed28c - std::panicking::try::do_call::h9c746f923b5e06d1
2020-04-01T20:35:43.2876043Z   35:     0x7feffbb83784 - std::panicking::try::do_try::hcf2e66025b546175
2020-04-01T20:35:43.2876660Z   36:     0x7feffc1ed153 - std::panicking::try::h004f60c8a50ed872
2020-04-01T20:35:43.2877326Z   37:     0x7feffc19b436 - rustc_driver::catch_fatal_errors::h2b3485e66eb95cbc
2020-04-01T20:35:43.2877946Z   38:     0x7feffc19c2b1 - rustc_driver::main::h1c0cfc2216bb9075
2020-04-01T20:35:43.2878578Z   39:     0x55d37b8218f3 - std::rt::lang_start::{{closure}}::h77bdb512518e089c
2020-04-01T20:35:43.2879253Z   40:     0x7feffbb83823 - std::panicking::try::do_call::h566d0db02fb821e5
2020-04-01T20:35:43.2879889Z   41:     0x7feffbb83784 - std::panicking::try::do_try::hcf2e66025b546175
2020-04-01T20:35:43.2880529Z   42:     0x7feffbb842bd - std::rt::lang_start_internal::hb6231994c0db5af5
2020-04-01T20:35:43.2881049Z   43:     0x55d37b8218e2 - main
2020-04-01T20:35:43.2881484Z   44:     0x7feffb741b97 - __libc_start_main
2020-04-01T20:35:43.2881911Z   45:     0x55d37b8217ca - _start
2020-04-01T20:35:43.2882344Z   46:                0x0 - <unknown>
2020-04-01T20:35:43.2882954Z error: internal compiler error: unexpected panic
2020-04-01T20:35:43.2883154Z 
2020-04-01T20:35:43.2883383Z thread panicked while processing panic. aborting.
2020-04-01T20:35:43.2883570Z 
2020-04-01T20:35:43.2883570Z 
2020-04-01T20:35:43.2884909Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-04-01T20:35:43.2890468Z expected success, got: exit code: 101
2020-04-01T20:35:43.2891228Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-04-01T20:35:43.2891643Z Build completed unsuccessfully in 0:15:55
2020-04-01T20:35:43.2891896Z == clock drift check ==
2020-04-01T20:35:43.2900011Z   local time: Wed Apr  1 20:35:43 UTC 2020
2020-04-01T20:35:43.3617477Z   network time: Wed, 01 Apr 2020 20:35:43 GMT
2020-04-01T20:35:44.6809516Z 
2020-04-01T20:35:44.6809516Z 
2020-04-01T20:35:44.6867156Z ##[error]Bash exited with code '1'.
2020-04-01T20:35:44.6879505Z ##[section]Finishing: Run build
2020-04-01T20:35:44.6924567Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70573/merge to s
2020-04-01T20:35:44.6928835Z Task         : Get sources
2020-04-01T20:35:44.6929117Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-01T20:35:44.6929381Z Version      : 1.0.0
2020-04-01T20:35:44.6929582Z Author       : Microsoft
2020-04-01T20:35:44.6929582Z Author       : Microsoft
2020-04-01T20:35:44.6929873Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-01T20:35:44.6930207Z ==============================================================================
2020-04-01T20:35:44.9494019Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-01T20:35:44.9536760Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70573/merge to s
2020-04-01T20:35:44.9599342Z Cleaning up task key
2020-04-01T20:35:44.9600367Z Start cleaning up orphan processes.
2020-04-01T20:35:44.9739214Z Terminate orphan process: pid (3341) (python)
2020-04-01T20:35:44.9850748Z ##[section]Finishing: Finalize Job
