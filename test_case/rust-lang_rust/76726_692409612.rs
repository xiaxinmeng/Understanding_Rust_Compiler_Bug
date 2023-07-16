plain
Initialized empty Git repository in /home/runner/work/rust/rust/.git/
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/76726/merge:refs/remotes/pull/76726/merge
---
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
   Compiling test v0.0.0 (/checkout/library/test)
    Finished release [optimized] target(s) in 1m 09s
Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
 Documenting core v0.0.0 (/checkout/library/core)
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:2196:48
     |
2196 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = note: `-D broken-intra-doc-links` implied by `-D warnings`
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:2220:48
     |
2220 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:2250:48
     |
2250 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:2280:48
     |
2280 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:2304:48
     |
2304 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:2334:48
     |
2334 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:2364:48
     |
2364 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:2388:48
     |
2388 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:2418:48
     |
2418 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:2448:48
     |
2448 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:2472:48
     |
2472 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:2502:48
     |
2502 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:2532:48
     |
2532 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:2556:48
     |
2556 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:2586:48
     |
2586 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:2616:48
     |
2616 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:2640:48
     |
2640 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:2670:48
     |
2670 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:2700:48
     |
2700 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:2724:48
     |
2724 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:2754:48
     |
2754 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:2784:48
     |
2784 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:2808:48
     |
2808 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:2838:48
     |
2838 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:2868:48
     |
2868 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:2892:48
     |
2892 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:2921:48
     |
2921 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:2946:48
     |
2946 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:2970:48
     |
2970 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:2999:48
     |
2999 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:3024:48
     |
3024 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:3048:48
     |
3048 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:3078:48
     |
3078 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:3109:48
     |
3109 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:3139:48
     |
3139 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:3163:48
     |
3163 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:3193:48
     |
3193 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:3224:48
     |
3224 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:3254:48
     |
3254 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:3280:48
     |
3280 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:3312:48
     |
3312 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:3344:48
     |
3344 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:3376:48
     |
3376 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:3402:48
     |
3402 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:3434:48
     |
3434 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:3466:48
     |
3466 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:3498:48
     |
3498 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:3522:48
     |
3522 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:3552:48
     |
3552 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:3583:48
     |
3583 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:3613:48
     |
3613 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:3642:48
     |
3642 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:3672:48
     |
3672 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:3703:48
     |
3703 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:3733:48
     |
3733 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:3759:48
     |
3759 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:3791:48
     |
3791 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:3823:48
     |
3823 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:3855:48
     |
3855 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:3886:48
     |
3886 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:3918:48
     |
3918 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `3:0`
error: unresolved link to `3:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:3950:48
     |
3950 | /// Rounding is done according to the rounding[3:0] parameter, which can be one of:
     |                                                ^^^ unresolved link
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
