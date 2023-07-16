plain
##[debug]Evaluating job defaults
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v2'
##[debug]Download 'https://api.github.com/repos/actions/checkout/tarball/5a4ac9002d0be2fb38bd78e4b4dbde5606d7042f' to '/home/runner/work/_actions/_temp_f1546009-49e5-4d74-88f0-3707a744473a/a70b31de-6dfe-49c9-897c-8e9f09119fac.tar.gz'
##[debug]Unwrap 'actions-checkout-5a4ac90' to '/home/runner/work/_actions/actions/checkout/v2'
##[debug]Archive '/home/runner/work/_actions/_temp_f1546009-49e5-4d74-88f0-3707a744473a/a70b31de-6dfe-49c9-897c-8e9f09119fac.tar.gz' has been unzipped into '/home/runner/work/_actions/actions/checkout/v2'.
Download action repository 'rust-lang/simpleinfra@master'
##[debug]Download 'https://api.github.com/repos/rust-lang/simpleinfra/tarball/caeabfd7dcd0342420d12a285e4a50b0a3ae2cc8' to '/home/runner/work/_actions/_temp_b2a0a2fb-e406-4c30-b766-9e7f68e73516/e702b4f1-4660-49cf-be72-20c2a78e698c.tar.gz'
##[debug]Unwrap 'rust-lang-simpleinfra-caeabfd' to '/home/runner/work/_actions/rust-lang/simpleinfra/master'
##[debug]Archive '/home/runner/work/_actions/_temp_b2a0a2fb-e406-4c30-b766-9e7f68e73516/e702b4f1-4660-49cf-be72-20c2a78e698c.tar.gz' has been unzipped into '/home/runner/work/_actions/rust-lang/simpleinfra/master'.
##[debug]action.yml for action: '/home/runner/work/_actions/rust-lang/simpleinfra/master/github-actions/cancel-outdated-builds/action.yml'.
##[debug]Set step 'run1' display name to: 'disable git crlf conversion'
##[debug]Set step 'actionscheckout' display name to: 'checkout the source code'
##[debug]Set step 'run2' display name to: 'configure the PR in which the error message will be posted'
---
::endgroup::
##[endgroup]
::group::Fetching the repository
##[group]Fetching the repository
[command]/usr/bin/git -c protocol.version=2 fetch --no-tags --prune --progress --no-recurse-submodules --depth=2 origin +1cd838451943835da5228ec71c7bae5b8268efdd:refs/remotes/origin/try
---
   Compiling memchr v2.3.3
   Compiling lazy_static v1.4.0
   Compiling libc v0.2.79
   Compiling serde_derive v1.0.115
   Compiling log v0.4.11 (https://github.com/Aaron1011/log?branch=fix/semi#ed7a622a)
   Compiling ryu v1.0.5
   Compiling regex-syntax v0.6.18
   Compiling cfg-if v0.1.10
   Compiling cfg-if v1.0.0
---
   Compiling serde v1.0.115
   Compiling cfg-if v0.1.10
   Compiling ppv-lite86 v0.2.8
   Compiling siphasher v0.3.3
   Compiling log v0.4.11 (https://github.com/Aaron1011/log?branch=fix/semi#ed7a622a)
   Compiling cfg-if v1.0.0
   Compiling itoa v0.4.6
   Compiling mac v0.1.1
   Compiling precomputed-hash v0.1.1
---
   Compiling syn v1.0.38
   Compiling cc v1.0.60
   Compiling scopeguard v1.1.0
   Compiling smallvec v1.4.2
   Compiling log v0.4.11 (https://github.com/Aaron1011/log?branch=fix/semi#ed7a622a)
   Compiling instant v0.1.6
   Compiling typenum v1.12.0
   Compiling version_check v0.9.1
   Compiling hashbrown v0.9.0
---
   Compiling syn v1.0.38
   Compiling cc v1.0.60
   Compiling scopeguard v1.1.0
   Compiling smallvec v1.4.2
   Compiling log v0.4.11 (https://github.com/Aaron1011/log?branch=fix/semi#ed7a622a)
   Compiling instant v0.1.6
   Compiling typenum v1.12.0
   Compiling version_check v0.9.1
   Compiling getrandom v0.1.14
---
Documenting stage2 std (x86_64-unknown-linux-gnu)
Uplifting stage1 std (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
Copying stage2 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
 Documenting core v0.0.0 (/checkout/library/core)
warning: unresolved link to `2:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:2065:44
     |
2065 | /// Rounding is done according to the imm8[2:0] parameter, which can be one of:
     |                                            ^^^ no item named `2:0` in scope
     |
     = note: `#[warn(broken_intra_doc_links)]` on by default
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
warning: unresolved link to `2:0`
warning: unresolved link to `2:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:2094:44
     |
2094 | /// Rounding is done according to the imm8[2:0] parameter, which can be one of:
     |                                            ^^^ no item named `2:0` in scope
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
warning: unresolved link to `2:0`
warning: unresolved link to `2:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:2123:44
     |
2123 | /// Rounding is done according to the imm8[2:0] parameter, which can be one of:
     |                                            ^^^ no item named `2:0` in scope
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
warning: unresolved link to `2:0`
warning: unresolved link to `2:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:2152:44
     |
2152 | /// Rounding is done according to the imm8[2:0] parameter, which can be one of:
     |                                            ^^^ no item named `2:0` in scope
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
warning: unresolved link to `2:0`
warning: unresolved link to `2:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:2181:44
     |
2181 | /// Rounding is done according to the imm8[2:0] parameter, which can be one of:
     |                                            ^^^ no item named `2:0` in scope
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
warning: unresolved link to `2:0`
warning: unresolved link to `2:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:2215:44
     |
2215 | /// Rounding is done according to the imm8[2:0] parameter, which can be one of:
     |                                            ^^^ no item named `2:0` in scope
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
warning: unresolved link to `2:0`
warning: unresolved link to `2:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:5512:44
     |
5512 | /// Rounding is done according to the imm8[2:0] parameter, which can be one of:
     |                                            ^^^ no item named `2:0` in scope
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
warning: unresolved link to `2:0`
warning: unresolved link to `2:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:5542:44
     |
5542 | /// Rounding is done according to the imm8[2:0] parameter, which can be one of:
     |                                            ^^^ no item named `2:0` in scope
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
warning: unresolved link to `2:0`
warning: unresolved link to `2:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:5572:44
     |
5572 | /// Rounding is done according to the imm8[2:0] parameter, which can be one of:
     |                                            ^^^ no item named `2:0` in scope
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
warning: unresolved link to `2:0`
warning: unresolved link to `2:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:5607:44
     |
5607 | /// Rounding is done according to the imm8[2:0] parameter, which can be one of:
     |                                            ^^^ no item named `2:0` in scope
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
warning: unresolved link to `2:0`
warning: unresolved link to `2:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:5637:44
     |
5637 | /// Rounding is done according to the imm8[2:0] parameter, which can be one of:
     |                                            ^^^ no item named `2:0` in scope
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
warning: unresolved link to `2:0`
warning: unresolved link to `2:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:5667:44
     |
5667 | /// Rounding is done according to the imm8[2:0] parameter, which can be one of:
     |                                            ^^^ no item named `2:0` in scope
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
warning: unresolved link to `2:0`
warning: unresolved link to `2:0`
     --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:19448:44
      |
19448 | /// Rounding is done according to the imm8[2:0] parameter, which can be one of:
      |                                            ^^^ no item named `2:0` in scope
      |
      = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
warning: unresolved link to `2:0`
warning: unresolved link to `2:0`
     --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:19478:44
      |
19478 | /// Rounding is done according to the imm8[2:0] parameter, which can be one of:
      |                                            ^^^ no item named `2:0` in scope
      |
      = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
warning: unresolved link to `2:0`
warning: unresolved link to `2:0`
     --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:19514:44
      |
19514 | /// Rounding is done according to the imm8[2:0] parameter, which can be one of:
      |                                            ^^^ no item named `2:0` in scope
      |
      = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
warning: unresolved link to `2:0`
warning: unresolved link to `2:0`
     --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:19544:44
      |
19544 | /// Rounding is done according to the imm8[2:0] parameter, which can be one of:
      |                                            ^^^ no item named `2:0` in scope
      |
      = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
warning: unresolved link to `2:0`
warning: unresolved link to `2:0`
     --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:19574:44
      |
19574 | /// Rounding is done according to the imm8[2:0] parameter, which can be one of:
      |                                            ^^^ no item named `2:0` in scope
      |
      = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
warning: unresolved link to `2:0`
warning: unresolved link to `2:0`
     --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:19610:44
      |
19610 | /// Rounding is done according to the imm8[2:0] parameter, which can be one of:
      |                                            ^^^ no item named `2:0` in scope
      |
      = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
warning: unresolved link to `2:0`
warning: unresolved link to `2:0`
     --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:21403:44
      |
21403 | /// Rounding is done according to the imm8[2:0] parameter, which can be one of:
      |                                            ^^^ no item named `2:0` in scope
      |
      = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
warning: unresolved link to `2:0`
warning: unresolved link to `2:0`
     --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:21434:44
      |
21434 | /// Rounding is done according to the imm8[2:0] parameter, which can be one of:
      |                                            ^^^ no item named `2:0` in scope
      |
      = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
warning: unresolved link to `2:0`
warning: unresolved link to `2:0`
     --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:21465:44
      |
21465 | /// Rounding is done according to the imm8[2:0] parameter, which can be one of:
      |                                            ^^^ no item named `2:0` in scope
      |
      = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
warning: unresolved link to `2:0`
warning: unresolved link to `2:0`
     --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:21502:44
      |
21502 | /// Rounding is done according to the imm8[2:0] parameter, which can be one of:
      |                                            ^^^ no item named `2:0` in scope
      |
      = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
warning: unresolved link to `2:0`
warning: unresolved link to `2:0`
     --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:21533:44
      |
21533 | /// Rounding is done according to the imm8[2:0] parameter, which can be one of:
      |                                            ^^^ no item named `2:0` in scope
      |
      = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
warning: unresolved link to `2:0`
warning: unresolved link to `2:0`
     --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:21564:44
      |
21564 | /// Rounding is done according to the imm8[2:0] parameter, which can be one of:
      |                                            ^^^ no item named `2:0` in scope
      |
      = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
warning: unresolved link to `2:0`
warning: unresolved link to `2:0`
    --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:2065:44
     |
2065 | /// Rounding is done according to the imm8[2:0] parameter, which can be one of:
     |                                            ^^^ no item named `2:0` in scope
     |
     = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
warning: 25 warnings emitted

    Finished release [optimized + debuginfo] target(s) in 13.26s
   Compiling cc v1.0.60
---
    Checking smallvec v1.4.2
   Compiling unicode-xid v0.2.1
    Checking scopeguard v1.1.0
   Compiling cc v1.0.60
   Compiling log v0.4.11 (https://github.com/Aaron1011/log?branch=fix/semi#ed7a622a)
    Checking hashbrown v0.9.0
    Checking instant v0.1.6
   Compiling typenum v1.12.0
   Compiling getrandom v0.1.14
