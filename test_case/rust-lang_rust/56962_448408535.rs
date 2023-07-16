plain
travis_time:end:0007891a:start=1545171189780722977,finish=1545171244851088695,duration=55070365718
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:01:29] .................................................................................................... 500/5184
[01:01:33] ..............................i..................................................................... 600/5184
[01:01:36] .................................................................................................... 700/5184
[01:01:42] .................................................................................................... 800/5184
[01:01:46] .....i....F.FF.F.....i.............................................................................. 900/5184
[01:01:50] .............................iiiii.................................................................. 1000/5184
[01:01:55] .................................................................................................... 1200/5184
[01:01:57] .................................................................................................... 1300/5184
[01:02:00] .................................................................................................... 1400/5184
[01:02:02] .................................................................................................... 1500/5184
---
[01:03:17] ........................................ii.......................................................... 3600/5184
[01:03:19] ..........................................................i......................................... 3700/5184
[01:03:20] .................................................................................................... 3800/5184
[01:03:21] ..............i..................................................................................... 3900/5184
[01:03:24] ..................................................................................................FF 4000/5184
[01:03:27] .FFFF.F.FFFFFFF.F..FF..FFFFFFFFFFFFFF..FFFFFF.FFFFF...FFF.F....F.................................... 4100/5184
[01:03:34] .................................................................................................... 4300/5184
[01:03:34] .................................................................................................... 4300/5184
[01:03:37] ...................................................i.......................................F........ 4400/5184
[01:03:43] ........................................................F...F....................................... 4500/5184
[01:03:49] .................................................................................................... 4700/5184
[01:03:53] .................................................................................................... 4800/5184
[01:03:57] .................................................................................................... 4900/5184
[01:04:00] .................................................................................................... 5000/5184
[01:04:00] .................................................................................................... 5000/5184
[01:04:04] .......................i............................................................
[01:04:04] failures:
[01:04:04] 
[01:04:04] ---- [ui] ui/custom-derive/derive-in-mod.rs stdout ----
[01:04:04] 
[01:04:04] error: auxiliary build of "/checkout/src/test/ui/custom-derive/auxiliary/plugin.rs" failed to compile: 
[01:04:04] status: exit code: 101
[01:04:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/custom-derive/auxiliary/plugin.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/custom-derive/derive-in-mod/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/custom-derive/derive-in-mod/auxiliary"
[01:04:04] ------------------------------------------
[01:04:04] 
[01:04:04] ------------------------------------------
[01:04:04] stderr:
[01:04:04] stderr:
[01:04:04] ------------------------------------------
[01:04:04] {"message":"unused variable: `input`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/custom-derive/auxiliary/plugin.rs","byte_start":728,"byte_end":733,"line_start":26,"line_end":26,"column_start":19,"column_end":24,"is_primary":true,"text":[{"text":"pub fn derive_bar(input: TokenStream) -> TokenStream {","highlight_start":19,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_variables)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using `_input` instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/custom-derive/auxiliary/plugin.rs","byte_start":728,"byte_end":733,"line_start":26,"line_end":26,"column_start":19,"column_end":24,"is_primary":true,"text":[{"text":"pub fn derive_bar(input: TokenStream) -> TokenStream {","highlight_start":19,"highlight_end":24}],"label":null,"suggested_replacement":"_input","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `input`\n  --> /checkout/src/test/ui/custom-derive/auxiliary/plugin.rs:26:19\n   |\nLL | pub fn derive_bar(input: TokenStream) -> TokenStream {\n   |                   ^^^^^ help: consider using `_input` instead\n   |\n   = note: #[warn(unused_variables)] on by default\n\n"}
[01:04:04] {"message":"unused variable: `input`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/custom-derive/auxiliary/plugin.rs","byte_start":863,"byte_end":868,"line_start":31,"line_end":31,"column_start":20,"column_end":25,"is_primary":true,"text":[{"text":"pub fn with_helper(input: TokenStream) -> TokenStream {","highlight_start":20,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `_input` instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/custom-derive/auxiliary/plugin.rs","byte_start":863,"byte_end":868,"line_start":31,"line_end":31,"column_start":20,"column_end":25,"is_primary":true,"text":[{"text":"pub fn with_helper(input: TokenStream) -> TokenStream {","highlight_start":20,"highlight_end":25}],"label":null,"suggested_replacement":"_input","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `input`\n  --> /checkout/src/test/ui/custom-derive/auxiliary/plugin.rs:31:20\n   |\nLL | pub fn with_helper(input: TokenStream) -> TokenStream {\n   |                    ^^^^^ help: consider using `_input` instead\n\n"}
[01:04:04] {"message":"src/librustc_mir/monomorphize/collector.rs:757: Cannot create local mono-item for DefId(10/0:523 ~ proc_macro[c6f4]::bridge[0]::client[0]::{{impl}}[13]::get[0])","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src/librustc_mir/monomorphize/collector.rs:757: Cannot create local mono-item for DefId(10/0:523 ~ proc_macro[c6f4]::bridge[0]::client[0]::{{impl}}[13]::get[0])\n\n"}
[01:04:04] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:04:04] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:04:04] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:04:04] note: the compiler unexpectedly panicked. this is a bug.
[01:04:04] 
[01:04:04] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:04:04] 
[01:04:04] 
[01:04:04] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[01:04:04] 
[01:04:04] note: compiler flags: -Z ui-testing -Z unstable-options -C rpath
[01:04:04] 
[01:04:04] ------------------------------------------
[01:04:04] 
[01:04:04] thread '[ui] ui/custom-derive/derive-in-mod.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[01:04:04] thread '[ui] ui/custom-derive/derive-in-mod.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[01:04:04] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:04:04] 
[01:04:04] ---- [ui] ui/custom-derive/helper-attr-blocked-by-import-ambig.rs stdout ----
[01:04:04] 
[01:04:04] error: auxiliary build of "/checkout/src/test/ui/custom-derive/auxiliary/plugin.rs" failed to compile: 
[01:04:04] status: exit code: 101
[01:04:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/custom-derive/auxiliary/plugin.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/custom-derive/helper-attr-blocked-by-import-ambig/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/custom-derive/helper-attr-blocked-by-import-ambig/auxiliary"
[01:04:04] ------------------------------------------
[01:04:04] 
[01:04:04] ------------------------------------------
[01:04:04] stderr:
[01:04:04] stderr:
[01:04:04] ------------------------------------------
[01:04:04] {"message":"unused variable: `input`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/custom-derive/auxiliary/plugin.rs","byte_start":728,"byte_end":733,"line_start":26,"line_end":26,"column_start":19,"column_end":24,"is_primary":true,"text":[{"text":"pub fn derive_bar(input: TokenStream) -> TokenStream {","highlight_start":19,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_variables)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using `_input` instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/custom-derive/auxiliary/plugin.rs","byte_start":728,"byte_end":733,"line_start":26,"line_end":26,"column_start":19,"column_end":24,"is_primary":true,"text":[{"text":"pub fn derive_bar(input: TokenStream) -> TokenStream {","highlight_start":19,"highlight_end":24}],"label":null,"suggested_replacement":"_input","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `input`\n  --> /checkout/src/test/ui/custom-derive/auxiliary/plugin.rs:26:19\n   |\nLL | pub fn derive_bar(input: TokenStream) -> TokenStream {\n   |                   ^^^^^ help: consider using `_input` instead\n   |\n   = note: #[warn(unused_variables)] on by default\n\n"}
[01:04:04] {"message":"unused variable: `input`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/custom-derive/auxiliary/plugin.rs","byte_start":863,"byte_end":868,"line_start":31,"line_end":31,"column_start":20,"column_end":25,"is_primary":true,"text":[{"text":"pub fn with_helper(input: TokenStream) -> TokenStream {","highlight_start":20,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `_input` instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/custom-derive/auxiliary/plugin.rs","byte_start":863,"byte_end":868,"line_start":31,"line_end":31,"column_start":20,"column_end":25,"is_primary":true,"text":[{"text":"pub fn with_helper(input: TokenStream) -> TokenStream {","highlight_start":20,"highlight_end":25}],"label":null,"suggested_replacement":"_input","suggestion_applicability":"i] ui/custom-derive/helper-attr-blocked-by-import-ambig.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[01:04:04] ---- [ui] ui/custom-derive/helper-attr-blocked-by-import.rs stdout ----
[01:04:04] 
[01:04:04] 
[01:04:04] error: auxiliary build of "/checkout/src/test/ui/custom-derive/auxiliary/plugin.rs" failed to compile: 
[01:04:04] status: exit code: 101
[01:04:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/custom-derive/auxiliary/plugin.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/custom-derive/helper-attr-blocked-by-import/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/custom-derive/helper-attr-blocked-by-import/auxiliary"
[01:04:04] ------------------------------------------
[01:04:04] 
[01:04:04] ------------------------------------------
[01:04:04] stderr:
[01:04:04] stderr:
[01:04:04] ------------------------------------------
[01:04:04] {"message":"unused variable: `input`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/custom-derive/auxiliary/plugin.rs","byte_start":728,"byte_end":733,"line_start":26,"line_end":26,"column_start":19,"column_end":24,"is_primary":true,"text":[{"text":"pub fn derive_bar(input: TokenStream) -> TokenStream {","highlight_start":19,"highlight_end":24}],"label":null,"suggested_replacement":nuame":"/checkout/src/test/ui/custom-derive/auxiliary/plugin.rs","byte_start":863,"byte_end":868,"line_start":31,"line_end":31,"column_start":20,"column_end":25,"is_primary":true,"text":[{"text":"pub fn with_helper(input: TokenStream) -> TokenStream {","highlight_start":20,"highlight_end":25}],"label":null,"suggested_replacement":"_input","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `input`\n  --> /checkout/src/test/ui/custom-derive/auxiliary/plugin.rs:31:20\n   |\nLL | pub fn with_helper(input: TokenStream) -> TokenStream {\n   |                    ^^^^^ help: consider using `_input` instead\n\n"}
[01:04:04] {"message":"src/librustc_mir/monomorphize/collector.rs:757: Cannot create local mono-item for DefId(10/0:523 ~ proc_macro[c6f4]::bridge[0]::client[0]::{{impl}}[13]::get[0])","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src/librustc_mir/monomorphize/collector.rs:757: Cannot create local mono-item for DefId(10/0:523 ~ proc_macro[c6f4]::bridge[0]::client[0]::{{impl}}[13]::get[0])\n\n"}
[01:04:04] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:04:04] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:04:04] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:04:04] note: the compiler unexpectedly panicked. this is a bug.
[01:04:04] 
[01:04:04] 
[01:04:04] note: we would appreciate a bug reporxt":[{"text":"pub fn with_helper(input: TokenStream) -> TokenStream {","highlight_start":20,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `_input` instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/custom-derive/auxiliary/plugin.rs","byte_start":863,"byte_end":868,"line_start":31,"line_end":31,"column_start":20,"column_end":25,"is_primary":true,"text":[{"text":"pub fn with_helper(input: TokenStream) -> TokenStream {","highlight_start":20,"highlight_end":25}],"label":null,"suggested_replacement":"_input","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `input`\n  --> /checkout/src/test/ui/custom-derive/auxiliary/plugin.rs:31:20\n   |\nLL | pub fn with_helper(input: TokenStream) -> TokenStream {\n   |                    ^^^^^ help: consider using `_input` instead\n\n"}
[01:04:04] {"message":"src/librustc_mir/monomorphize/collector.rs:757: Cannot create local mono-item for DefId(10/0:523 ~ proc_macro[c6f4]::bridge[0]::client[0]::{{impl}}[13]::get[0])","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src/librustc_mir/monomorphize/collector.rs:757: Cannot create local mono-item for DefId(10/0:523 ~ proc_macro[c6f4]::bridge[0]::client[0]::{{impl}}[13]::get[0])\n\n"}
[01:04:04] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:04:04] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[instead\n   |\n   = note: #[warn(unused_variables)] on by default\n\n"}
[01:04:04] {"message":"unused variable: `input`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/proc-macro/auxiliary/builtin-attrs.rs","byte_start":967,"byte_end":972,"line_start":35,"line_end":35,"column_start":30,"column_end":35,"is_primary":true,"text":[{"text":"pub fn bench(_: TokenStream, input: TokenStream) -> TokenStream {","highlight_start":30,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `_input` instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/proc-macro/auxiliary/builtin-attrs.rs","byte_start":967,"byte_end":972,"line_start":35,"line_end":35,"column_start":30,"column_end":35,"is_primary":true,"text":[{"text":"pub fn bench(_: TokenStream, input: TokenStream) -> TokenStream {","highlight_start":30,"highlight_end":35}],"label":null,"suggested_replacement":"_input","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `input`\n  --> /checkout/src/test/ui/proc-macro/auxiliary/builtin-attrs.rs:35:30\n   |\nLL | pub fn bench(_: TokenStream, input: TokenStream) -> TokenStream {\n   |                              ^^^^^ help: consider using `_input` instead\n\n"}
[01:04:04] {"message":"src/librustc_mir/monomorphize/collector.rs:757: Cannot create local mono-item for DefId(10/0:523 ~ proc_macro[c6f4]::bridge[0]::client[0]::{{impl}}[13]::get[0])","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src/librustc_mir/monomorphize/collector.rs:757: Cannot create local mono-item for DefId(10/0:523 ~ proc_macro[c6f4]::bridge[0]::client[0]::{{impl}}[13]::get[0])\n\n"}
[01:04:04] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:04:04] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:04:04] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:04:04] note: the compiler unexpectedly panicked. this is a bug.
[01:04:04] 
[01:04:04] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:04:04] 
[01:04:04] 
[01:04:04] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[01:04:04] 
[01:04:04] note: compiler flags: -Z ui-testing -Z unstable-options -C rpath
[01:04:04] 
[01:04:04] ------------------------------------------
[01:04:04] 
[01:04:04] thread '[ui] ui/proc-macro/ambiguous-builtin-attrs-test.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[01:04:04] thread '[ui] ui/proc-macro/ambiguous-builtin-attrs-test.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[01:04:04] 
[01:04:04] ---- [ui] ui/proc-macro/ambiguous-builtin-attrs.rs stdout ----
[01:04:04] 
[01:04:04] error: auxiliary build of "/checkout/src/test/ui/proc-macro/auxiliary/builtin-attrs.rs" failed to compile: 
[01:04:04] status: exit code: 101
[01:04:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/auxiliary/builtin-attrs.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/ambiguous-builtin-attrs/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/ambiguous-builtin-attrs/auxiliary"
[01:04:04] ------------------------------------------
[01:04:04] 
[01:04:04] ------------------------------------------
[01:04:04] stderr:
[01:04:04] stderr:
[01:04:04] ------------------------------------------
[01:04:04] {"message":"unused variable: `input`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/proc-macro/auxiliary/builtin-attrs.rs","byte_start":838,"byte_end":843,"line_start":30,"line_end":30,"column_start":29,"column_end":34,"is_primary":true,"text":[{"text":"pub fn test(_: TokenStream, input: TokenStream) -> TokenStream {","highlight_start":29,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(unused_variables)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using `_input` instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/proc-macro/auxiliary/builtin-attrs.rs","byte_start":838,"byte_end":843,"line_start":30,"line_end":30,"column_start":29,"column_end":34,"is_primary":true,"text":[{"text":"pub fn test(_: TokenStream, input: TokenStream) -> TokenStream {","highlight_start":29,"highlight_end":34}],"label":null,"suggested_replacement":"_input","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `input`\n  --> /checkout/src/test/ui/proc-macro/auxiliary/builtin-attrs.rs:30:29\n   |\nLL | pub fn test(_: TokenStream, input: TokenStream) -> TokenStream {\n   |                             ^^^^^ help: consider using `_input` instead\n   |\n   = note: #[warn(unused_variables)] on by default\n\n"}
[01:04:04] {"message":"unused variable: `input`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/proc-macro/auxiliary/builtin-attrs.rs","byte_start":967,"byte_end":972,"line_start":35,"line_end":35,"column_start":30,"column_end":35,"is_primary":true,"text":[{"text":"pub fn bench(_: TokenStream, input: TokenStream) -> TokenStream {","highlight_start":30,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using `_input` instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/proc-macro/auxiliary/builtin-attrs.rs","byte_start":967,"byte_end":972,"line_start":35,"line_end":35,"column_start":30,"column_end":35,"is_primary":true,"text":[{"text":"pub fn bench(_: TokenStream, input: TokenStream) -> TokenStream {","highlight_start":30,"highlight_end":35}],"label":null,"suggested_replacement":"_input","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `input`\n  --> /checkout/src/test/ui/proc-macro/auxiliary/builtin-attrs.rs:35:30\n   |\nLL | pub fn bench(_: TokenStream, input: TokenStream) -> TokenStream {\n   |                              ^^^^^ help: consider using `_input` instead\n\n"}
[01:04:04] {"message":"src/librustc_mir/monomorphize/collector.rs:757: Cannot create local mono-item for DefId(10/0:523 ~ proc_macro[c6f4]::bridge[0]::client[0]::{{impl}}[13]::get[0])","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src/librustc_mir/monomorphize/collector.rs:757: Cannot create local mono-item for DefId(10/0:523 ~ proc_macro[c6f4]::bridge[0]::client[0]::{{impl}}[13]::get[0])\n\n"}
[01:04:04] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:04:04] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:04:04] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:04:04] note: the compiler unexpectedly panicked. this is a bug.
[01:04:04] 
[01:04:04] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:04:04] 
[01:04:04] 
[01:04:04] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[01:04:04] 
[01:04:04] note: compiler flags: -Z ui-testing -Z unstable-options -C rpath
[01:04:04] 
[01:04:04] ------------------------------------------
[01:04:04] 
[01:04:04] thread '[ui] ui/proc-macro/ambiguous-builtin-attrs.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[01:04:04] thread '[ui] ui/proc-macro/ambiguous-builtin-attrs.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[01:04:04] 
[01:04:04] ---- [ui] ui/proc-macro/attribute-order-restricted.rs stdout ----
[01:04:04] 
[01:04:04] error: auxiliary build of "/checkout/src/test/ui/proc-macro/auxiliary/attr_proc_macro.rs" failed to compile: 
[01:04:04] status: exit code: 101
[01:04:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/auxiliary/attr_proc_macro.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/attribute-order-restricted/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/attribute-order-restricted/auxiliary"
[01:04:04] ------------------------------------------
[01:04:04] 
[01:04:04] ------------------------------------------
[01:04:04] stderr:
[01:04:04] stderr:
[01:04:04] ------------------------------------------
[01:04:04] {"message":"src/librustc_mir/monomorphize/collector.rs:757: Cannot create local mono-item for DefId(10/0:523 ~ proc_macro[c6f4]::bridge[0]::client[0]::{{impl}}[13]::get[0])","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src/librustc_mir/monomorphize/collector.rs:757: Cannot create local mono-item for DefId(10/0:523 ~ proc_macro[c6f4]::bridge[0]::client[0]::{{impl}}[13]::get[0])\n\n"}
[01:04:04] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:04:04] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:04:04] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:04:04] note: the compiler unexpectedly panicked. this is a bug.
[01:04:04] 
[01:04:04] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:04:04] 
[01:04:04] 
[01:04:04] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[01:04:04] 
[01:04:04] note: compiler flags: -Z ui-testing -Z unstable-options -C rpath
[01:04:04] 
[01:04:04] ------------------------------------------
[01:04:04] 
[01:04:04] thread '[ui] ui/proc-macro/attribute-order-restricted.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[01:04:04] thread '[ui] ui/proc-macro/attribute-order-restricted.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[01:04:04] 
[01:04:04] ---- [ui] ui/proc-macro/attr-invalid-exprs.rs stdout ----
[01:04:04] 
[01:04:04] error: auxiliary build of "/checkout/src/test/ui/proc-macro/auxiliary/attr-stmt-expr.rs" failed to compile: 
[01:04:04] status: exit code: 101
[01:04:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/auxiliary/attr-stmt-expr.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/attr-invalid-exprs/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/attr-invalid-exprs/auxt.rs:3252:9
[01:04:04] ---- [ui] ui/proc-macro/attr-stmt-expr.rs stdout ----
[01:04:04] 
[01:04:04] 
[01:04:04] error: auxiliary build of "/checkout/src/test/ui/proc-macro/auxiliary/attr-stmt-expr.rs" failed to compile: 
[01:04:04] status: exit code: 101
[01:04:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/auxiliary/attr-stmt-expr.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/attr-stmt-expr/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/attr-stmt-expr/auxiliary"
[01:04:04] ------------------------------------------
[01:04:04] 
[01:04:04] ------------------------------------------
[01:04:04] stderr:
[01:04:04] stderr:
[01:04:04] ------------------------------------------
[01:04:04] {"message":"src/librustc_mir/monomorphize/collector.rs:757: Cannot create local mono-item for DefId(10/0:523 ~ proc_macro[c6f4]::bridge[0]::client[0]::{{impl}}[13]::get[0])","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src/librustc_mir/monomorphize/collector.rs:757: Cannot create local mono-item for DefId(10/0:523 ~ proc_macro[c6f4]::bridge[0]::client[0]::{{impl}}[13]::get[0])\n\n"}
[01:04:04] note: Run with `RUST_BACKTRACE=1` environment variable to display -------------------------------------
[01:04:04] 
[01:04:04] ------------------------------------------
[01:04:04] stderr:
[01:04:04] stderr:
[01:04:04] ------------------------------------------
[01:04:04] {"message":"src/librustc_mir/monomorphize/collector.rs:757: Cannot create local mono-item for DefId(10/0:523 ~ proc_macro[c6f4]::bridge[0]::client[0]::{{impl}}[13]::get[0])","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src/librustc_mir/monomorphize/collector.rs:757: Cannot create local mono-item for DefId(10/0:523 ~ proc_macro[c6f4]::bridge[0]::client[0]::{{impl}}[13]::get[0])\n\n"}
[01:04:04] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:04:04] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:04:04] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:04:04] note: the compiler unexpectedly panicked. this is a bug.
[01:04:04] 
[01:04:04] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:04:04] 
[01:04:04] 
[01:04:04] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[01:04:04] 
[01:04:04] note: compiler flags: -Z ui-testing -Z unstable-options -C rpath
[01:04:04] 
[01:04:04] ------------------------------------------
[01:04:04] 
[01:04:04] thread '[ui] ui/proc-macro/attribute-with-error.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[01:04:04] thread '[ui] ui/proc-macro/attribute-with-error.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[01:04:04] 
[01:04:04] ---- [],"children":[],"rendered":null},{"message":"consider using `_body` instead","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/proc-macro/auxiliary/attribute-spans-preserved.rs","byte_start":1012,"byte_end":1016,"line_start":32,"line_end":32,"column_start":9,"column_end":13,"is_primary":true,"text":[{"text":"    let body = tokens.next().unwrap();","highlight_start":9,"highlight_end":13}],"label":null,"suggested_replacement":"_body","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `body`\n  --> /checkout/src/test/ui/proc-macro/auxiliary/attribute-spans-preserved.rs:32:9\n   |\nLL |     let body = tokens.next().unwrap();\n   |         ^^^^ help: consider using `_body` instead\n   |\n   = note: #[warn(unused_variables)] on by default\n\n"}
[01:04:04] {"message":"src/librustc_mir/monomorphize/collector.rs:757: Cannot create local mono-item for DefId(10/0:523 ~ proc_macro[c6f4]::bridge[0]::client[0]::{{impl}}[13]::get[0])","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src/librustc_mir/monomorphize/collector.rs:757: Cannot create local mono-item for DefId(10/0:523 ~ proc_macro[c6f4]::bridge[0]::client[0]::{{impl}}[13]::get[0])\n\n"}
[01:04:04] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:04:04] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:04:04] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to prev----------------------------------------
[01:04:04] {"message":"src/librustc_mir/monomorphize/collector.rs:757: Cannot create local mono-item for DefId(10/0:523 ~ proc_macro[c6f4]::bridge[0]::client[0]::{{impl}}[13]::get[0])","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src/librustc_mir/monomorphize/collector.rs:757: Cannot create local mono-item for DefId(10/0:523 ~ proc_macro[c6f4]::bridge[0]::client[0]::{{impl}}[13]::get[0])\n\n"}
[01:04:04] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:04:04] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:04:04] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:04:04] note: the compiler unexpectedly panicked. this is a bug.
[01:04:04] 
[01:04:04] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:04:04] 
[01:04:04] 
[01:04:04] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[01:04:04] 
[01:04:04] note: compiler flags: -Z ui-testing -Z unstable-options -C rpath
[01:04:04] 
[01:04:04] ------------------------------------------
[01:04:04] 
[01:04:04] thread '[ui] ui/proc-macro/derive-helper-shadowed.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[01:04:04] thread '[ui] ui/proc-macro/derive-helper-shadowed.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[01:04:04] 
[01:04:04] ---- [ui] ui/proc-macro/derive-bad.rs stdout ----
[01:04:04] 
[01:04:04] error: auxiliary build of "/checkout/src/test/ui/proc-macro/auxiliary] note: the compiler unexpectedly panicked. this is a bug.
[01:04:04] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:04:04] 
[01:04:04] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[01:04:04] 
[01:04:04] 
[01:04:04] note: compiler flags: -Z ui-testing -Z unstable-options -C rpath
[01:04:04] 
[01:04:04] ------------------------------------------
[01:04:04] 
[01:04:04] thread '[ui] ui/proc-macro/derive-bad.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[01:04:04] thread '[ui] ui/proc-macro/derive-bad.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[01:04:04] 
[01:04:04] ---- [ui] ui/proc-macro/attributes-included.rs stdout ----
[01:04:04] 
[01:04:04] error: auxiliary build of "/checkout/src/test/ui/proc-macro/auxiliary/attributes-included.rs" failed to compile: 
[01:04:04] status: exit code: 101
[01:04:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/auxiliary/attributes-included.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/attributes-included/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/attributes-included/auxiliary"
[01:04:04] ------------------------------------------
[01:04:04] 
[01:04:04] ------------------------------------------
[01:04:04] stderr:
[01:04:04] stderr:
[01:04:04] ------------------------------------------
[01:04:04] {"message":"src/l":282,"byte_end":287,"line_start":15,"line_end":15,"column_start":15,"column_end":20,"is_primary":true,"text":[{"text":"pub fn derive(input: TokenStream) -> TokenStream {","highlight_start":15,"highlight_end":20}],"label":null,"suggested_replacement":"_input","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: unused variable: `input`\n  --> /checkout/src/test/ui/proc-macro/auxiliary/derive-helper-shadowing.rs:15:15\n   |\nLL | pub fn derive(input: TokenStream) -> TokenStream {\n   |               ^^^^^ help: consider using `_input` instead\n   |\n   = note: #[warn(unused_variables)] on by default\n\n"}
[01:04:04] {"message":"src/librustc_mir/monomorphize/collector.rs:757: Cannot create local mono-item for DefId(10/0:523 ~ proc_macro[c6f4]::bridge[0]::client[0]::{{impl}}[13]::get[0])","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src/librustc_mir/monomorphize/collector.rs:757: Cannot create local mono-item for DefId(10/0:523 ~ proc_macro[c6f4]::bridge[0]::client[0]::{{impl}}[13]::get[0])\n\n"}
[01:04:04] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:04:04] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:04:04] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:04:04] note: the compiler unexpectedly panicked. this is a bug.
[01:04:04] 
[01:04:04] 
[01:04:04] note: we would appreciate a bug report: https://githubr.rs:757: Cannot create local mono-item for DefId(10/0:523 ~ proc_macro[c6f4]::bridge[0]::client[0]::{{impl}}[13]::get[0])\n\n"}
[01:04:04] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:04:04] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:04:04] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:04:04] note: the compiler unexpectedly panicked. this is a bug.
[01:04:04] 
[01:04:04] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:04:04] 
[01:04:04] 
[01:04:04] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[01:04:04] 
[01:04:04] note: compiler flags: -Z ui-testing -Z unstable-options -C rpath
[01:04:04] 
[01:04:04] ------------------------------------------
[01:04:04] 
[01:04:04] thread '[ui] ui/proc-macro/derive-still-gated.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[01:04:04] thread '[ui] ui/proc-macro/derive-still-gated.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[01:04:04] 
[01:04:04] ---- [ui] ui/proc-macro/edition-imports-2018.rs stdout ----
[01:04:04] 
[01:04:04] error: auxiliary build of "/checkout/src/test/ui/proc-macro/auxiliary/edition-imports-2015.rs" failed to compile: 
[01:04:04] status: exit code: 101
[01:04:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/auxiliary/edition-imports-2015.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/edition-imports-2018/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2015" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/edition-imports-2018/auxiliary"
[01:04:04] ------------------------------------------
[01:04:04] 
[01:04:04] ------------------------------------------
[01:04:04] stderr:
[01:04:04] stderr:
[01:04:04] ------------------------------------------
[01:04:04] {"message":"src/librustc_mir/monomorphize/collector.rs:757: Cannot create local mono-item for DefId(10/0:523 ~ proc_macro[c6f4]::bridge[0]::client[0]::{{impl}}[13]::get[0])","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src/librustc_mir/monomorphize/collector.rs:757: Cannot create local mono-item for DefId(10/0:523 ~ proc_macro[c6f4]::bridge[0]::client[0]::{{impl}}[13]::get[0])\n\n"}
[01:04:04] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:04:04] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:04:04] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:04:04] note: the compiler unexpectedly panicked. this is a bug.
[01:04:04] 
[01:04:04] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:04:04] 
[01:04:04] 
[01:04:04] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[01:04:04] 
[01:04:04] note: compiler flags: -Z ui-testing -Z unstable-options -C rpath
[01:04:04] 
[01:04:04] ------------------------------------------
[01:04:04] 
[01:04:04] thread '[ui] ui/proc-macro/edition-imports-2018.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[01:04:04] thread '[ui] ui/proc-macro/edition-imports-2018.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[01:04:04] 
[01:04:04] ---- [ui] ui/proc-macro/expand-to-unstable-2.rs stdout ----
[01:04:04] 
[01:04:04] error: auxiliary build of "/checkout/src/test/ui/proc-macro/auxiliary/derive-unstable-2.rs" failed to compile: 
[01:04:04] status: exit code: 101
[01:04:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/auxiliary/derive-unstable-2.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/expand-to-unstable-2/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/expand-to-unstable-2/auxiliary"
[01:04:04] ------------------------------------------
[01:04:04] 
[01:04:04] ------------------------------------------
[01:04:04] stderr:
[01:04:04] stderr:
[01:04:04] ------------------------------------------
[01:04:04] {"message":"src/librustc_mir/monomorphize/collector.rs:757: Cannot create local mono-item for DefId(10/0:523 ~ proc_macro[c6f4]::bridge[0]::client[0]::{{impl}}[13]::get[0])","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src/librustc_mir/monomorphize/collector.rs:757: Cannot create local mono-item for DefId(10/0:523 ~ proc_macro[c6f4]::bridge[0]::client[0]::{{impl}}[13]::get[0])\n\n"}
[01:04:04] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:04:04] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:04:04] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:04:04] note: the compiler unexpectedly panicked. this is a bug.
[01:04:04] 
[01:04:04] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:04:04] 
[01:04:04] 
[01:04:04] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[01:04:04] 
[01:04:04] note: compiler flags: -Z ui-testing -Z unstable-options -C rpath
[01:04:04] 
[01:04:04] ------------------------------------------
[01:04:04] 
[01:04:04] thread '[ui] ui/proc-macro/expand-to-unstable-2.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[01:04:04] thread '[ui] ui/proc-macro/expand-to-unstable-2.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[01:04:04] 
[01:04:04] ---- [ui] ui/proc-macro/expand-to-unstable.rs stdout ----
[01:04:04] 
[01:04:04] error: auxiliary build of "/checkout/src/test/ui/proc-macro/auxiliary/derive-unstable.rs" failed to compile: 
[01:04:04] status: exit code: 101
[01:04:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/auxiliary/derive-unstable.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/expand-to-unstable/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/expand-to-unstable/auxiliary"
[01:04:04] ------------------------------------------
[01:04:04] 
[01:04:04] ------------------------------------------
[01:04:04] stderr:
[01:04:04] stderr:
[01:04:04] ------------------------------------------
[01:04:04] {"message":"src/librustc_mir/monomorphize/collector.rs:757: Cannot create local mono-item for DefId(10/0:523 ~ proc_macro[c6f4]::bridge[0]::client[0]::{{impl}}[13]::get[0])","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src/librustc_mir/monomorphize/collector.rs:757: Cannot create local mono-item for DefId(10/0:523 ~ proc_macro[c6f4]::bridge[0]::client[0]::{{impl}}[13]::get[0])\n\n"}
[01:04:04] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:04:04] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:04:04] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:04:04] note: the compiler unexpectedly panicked. this is a bug.
[01:04:04] 
[01:04:04] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:04:04] 
[01:04:04] 
[01:04:04] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[01:04:04] 
[01:04:04] note: compiler flags: -Z ui-testing -Z unstable-options -C rpath
[01:04:04] 
[01:04:04] ------------------------------------------
[01:04:04] 
[01:04:04] thread '[ui] ui/proc-macro/expand-to-unstable.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[01:04:04] thread '[ui] ui/proc-macro/expand-to-unstable.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[01:04:04] 
[01:04:04] ---- [ui] ui/proc-macro/import.rs stdout ----
[01:04:04] 
[01:04:04] error: auxiliary build of "/checkout/src/test/ui/proc-macro/auxiliary/derive-a.rs" failed to compile: 
[01:04:04] status: exit code: 101
[01:04:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/auxiliary/derive-a.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/import/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/import/auxiliary"
[01:04:04] ------------------------------------------
[01:04:04] 
[01:04:04] ------------------------------------------
[01:04:04] stderr:
[01:04:04] stderr:
[01:04:04] ------------------------------------------
[01:04:04] {"message":"unused variable: `input`","code":{"code":"unused_variables","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/proc-macro/auxiliary/derive-a.rs","byte_start":630,"byte_end":635,"line_start":21,"line_end":21,"column_start":17,"column_end":22,"is_primary":true,"text":[{"text":"pub fn derive_a(input: TokenStream) -> TokenStream {","highlight_start":17,"highlight_end":22}],"label"` environment variable to display a backtrace.
[01:04:04] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:04:04] note: the compiler unexpectedly panicked. this is a bug.
[01:04:04] 
[01:04:04] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:04:04] 
[01:04:04] 
[01:04:04] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[01:04:04] 
[01:04:04] note: compiler flags: -Z ui-testing -Z unstable-options -C rpath
[01:04:04] 
[01:04:04] ------------------------------------------
[01:04:04] 
[01:04:04] thread '[ui] ui/proc-macro/import.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[01:04:04] thread '[ui] ui/proc-macro/import.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[01:04:04] 
[01:04:04] ---- [ui] ui/proc-macro/generate-mod.rs stdout ----
[01:04:04] 
[01:04:04] error: auxiliary build of "/checkout/src/test/ui/proc-macro/auxiliary/generate-mod.rs" failed to compile: 
[01:04:04] status: exit code: 101
[01:04:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/auxiliary/generate-mod.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/generate-mod/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/generate-mod/auxiliary"
[01:04:04] -------------------lid-punct-ident-1.rs stdout ----
[01:04:04] 
[01:04:04] 
[01:04:04] error: auxiliary build of "/checkout/src/test/ui/proc-macro/auxiliary/invalid-punct-ident.rs" failed to compile: 
[01:04:04] status: exit code: 101
[01:04:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/auxiliary/invalid-punct-ident.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/invalid-punct-ident-1/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/invalid-punct-ident-1/auxiliary"
[01:04:04] ------------------------------------------
[01:04:04] 
[01:04:04] ------------------------------------------
[01:04:04] stderr:
[01:04:04] stderr:
[01:04:04] ------------------------------------------
[01:04:04] {"message":"src/librustc_mir/monomorphize/collector.rs:757: Cannot create local mono-item for DefId(10/0:523 ~ proc_macro[c6f4]::bridge[0]::client[0]::{{impl}}[13]::get[0])","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src/librustc_mir/monomorphize/collector.rs:757: Cannot create local mono-item for DefId(10/0:523 ~ proc_macro[c6f4]::bridge[0]::client[0]::{{impl}}[13]::get[0])\n\n"}
[01:04:04] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:04:04] {"message-----------
[01:04:04] 
[01:04:04] ------------------------------------------
[01:04:04] ------------------------------------------
[01:04:04] stderr:
[01:04:04] ------------------------------------------
[01:04:04] {"message":"src/librustc_mir/monomorphize/collector.rs:757: Cannot create local mono-item for DefId(10/0:523 ~ proc_macro[c6f4]::bridge[0]::client[0]::{{impl}}[13]::get[0])","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src/librustc_mir/monomorphize/collector.rs:757: Cannot create local mono-item for DefId(10/0:523 ~ proc_macro[c6f4]::bridge[0]::client[0]::{{impl}}[13]::get[0])\n\n"}
[01:04:04] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:04:04] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:04:04] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:04:04] note: the compiler unexpectedly panicked. this is a bug.
[01:04:04] 
[01:04:04] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:04:04] 
[01:04:04] 
[01:04:04] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[01:04:04] 
[01:04:04] note: compiler flags: -Z ui-testing -Z unstable-options -C rpath
[01:04:04] 
[01:04:04] ------------------------------------------
[01:04:04] 
[01:04:04] thread '[ui] ui/proc-macro/invalid-punct-ident-2.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[01:04:04] thread '[ui] ui/proc-macro/invalid-punct-ident-2.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[01:04:04] 
[01:04:04] ---- [ui] ui/proc-macro/invalidaborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:04:04] note: the compiler unexpectedly panicked. this is a bug.
[01:04:04] 
[01:04:04] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:04:04] 
[01:04:04] 
[01:04:04] note: rustc 1.33.0-dev running on x86_64-unknown-linux-gnu
[01:04:04] 
[01:04:04] note: compiler flags: -Z ui-testing -Z unstable-options -C rpath
[01:04:04] 
[01:04:04] ------------------------------------------
[01:04:04] 
[01:04:04] thread '[ui] ui/proc-macro/invalid-punct-ident-3.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[01:04:04] thread '[ui] ui/proc-macro/invalid-punct-ident-3.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[01:04:04] 
[01:04:04] ---- [ui] ui/proc-macro/invalid-punct-ident-4.rs stdout ----
[01:04:04] 
[01:04:04] error: auxiliary build of "/checkout/src/test/ui/proc-macro/auxiliary/invalid-punct-ident.rs" failed to compile: 
[01:04:04] status: exit code: 101
[01:04:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/auxiliary/invalid-punct-ident.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/invalid-punct-ident-4/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/invalid-punct-ident-4/auxiliary"
[01:04:04] ----------------------------------/tools/compiletest/src/runtest.rs:3252:9
[01:04:04] 
[01:04:04] ---- [ui] ui/proc-macro/issue-38586.rs stdout ----
[01:04:04] 
[01:04:04] 
[01:04:04] error: auxiliary build of "/checkout/src/test/ui/proc-macro/auxiliary/issue_38586.rs" failed to compile: 
[01:04:04] status: exit code: 101
[01:04:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/auxiliary/issue_38586.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/issue-38586/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/issue-38586/auxiliary"
[01:04:04] ------------------------------------------
[01:04:04] 
[01:04:04] ------------------------------------------
[01:04:04] stderr:
[01:04:04] stderr:
[01:04:04] ------------------------------------------
[01:04:04] {"message":"src/librustc_mir/monomorphize/collector.rs:757: Cannot create local mono-item for DefId(10/0:523 ~ proc_macro[c6f4]::bridge[0]::client[0]::{{impl}}[13]::get[0])","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src/librustc_mir/monomorphize/collector.rs:757: Cannot create local mono-item for DefId(10/0:523 ~ proc_macro[c6f4]::bridge[0]::client[0]::{{impl}}[13]::get[0])\n\n"}
[01:04:04] note: Run with `RUST_BACKTRACE=1` environment variab
[01:04:04] 
[01:04:04] 
[01:04:04] error: auxiliary build of "/checkout/src/test/ui/proc-macro/auxiliary/issue-41211.rs" failed to compile: 
[01:04:04] status: exit code: 101
[01:04:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/auxiliary/issue-41211.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/issue-41211/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/issue-41211/auxiliary"
[01:04:04] ------------------------------------------
[01:04:04] 
[01:04:04] ------------------------------------------
[01:04:04] stderr:
[01:04:04] stderr:
[01:04:04] ------------------------------------------
[01:04:04] {"message":"src/librustc_mir/monomorphize/collector.rs:757: Cannot create local mono-item for DefId(10/0:523 ~ proc_macro[c6f4]::bridge[0]::client[0]::{{impl}}[13]::get[0])","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src/librustc_mir/monomorphize/collector.rs:757: Cannot create local mono-item for DefId(10/0:523 ~ proc_macro[c6f4]::bridge[0]::client[0]::{{impl}}[13]::get[0])\n\n"}
[01:04:04] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:04:04] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:04:04] {"message":"aborting due to previous error","code":null,"level":"error","spansed to compile: 
[01:04:04] status: exit code: 101
[01:04:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/auxiliary/issue-53481.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/issue-53481/auxiliary" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/issue-53481/auxiliary"
[01:04:04] ------------------------------------------
[01:04:04] 
[01:04:04] ------------------------------------------
[01:04:04] stderr:
[01:04:04] stderr:
[01:04:04] ------------------------------------------
---
[01:04:04] test result: FAILED. 5107 passed; 54 failed; 23 ignored; 0 measured; 0 filtered out
[01:04:04] 
[01:04:04] 
[01:04:04] 
[01:04:04] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvtstrap-tools/x86_64-unknown-linux-gnu
118652 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps
115344 ./src/llvm/test/CodeGen
111164 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
107420 ./src/tools/lldb
---
travis_time:end:08e9e50a:start=1545175100727303400,finish=1545175100731844488,duration=4541088
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1a826de4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set
