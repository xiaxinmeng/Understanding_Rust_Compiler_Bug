plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 24'
Agent machine name: 'fv-az578'
Current agent version: '2.169.0'
##[group]Operating System
16.04.6
LTS
LTS
##[endgroup]
##[group]Virtual Environment
Environment: ubuntu-16.04
Version: 20200517.1
Included Software: https://github.com/actions/virtual-environments/blob/ubuntu16/20200517.1/images/linux/Ubuntu1604-README.md
##[endgroup]
Agent running as: 'vsts'
Prepare build directory.
Set build variables.
Download all required tasks.
Download all required tasks.
Downloading task: Bash (3.163.2)
Checking job knob settings.
   Knob: AgentToolsDirectory = /opt/hostedtoolcache Source: ${AGENT_TOOLSDIRECTORY} 
   Knob: AgentPerflog = /home/vsts/perflog Source: ${VSTS_AGENT_PERFLOG} 
Start tracking orphan processes.
##[section]Finishing: Initialize job
##[section]Starting: Configure Job Name
==============================================================================
---
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/db8e33ff-7149-4579-96a1-e0c06a293777.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/72585/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/72585/merge:refs/remotes/pull/72585/merge
---
 ---> cb2676f08729
Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
 ---> Using cache
 ---> df25ce111862
Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
 ---> 599b9ac96b27
Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
 ---> Using cache
 ---> 091087e35a36
---
   Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
   Compiling chalk-rust-ir v0.10.0
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling chalk-solve v0.10.0
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
   Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
   Compiling chalk-rust-ir v0.10.0
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling chalk-solve v0.10.0
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
...........................................................................i........................ 1800/10223
.................................................................................................... 1900/10223
..............................................................................................i..i.. 2000/10223
.................................................................................................... 2100/10223
....................................................................................iiiii........... 2200/10223
.................................................................................................... 2400/10223
.................................................................................................... 2500/10223
.................................................................................................... 2600/10223
.................................................................................................... 2700/10223
---
...........i...............i........................................................................ 5200/10223
.................................................................................................... 5300/10223
..........................................................i......................................... 5400/10223
...................................................i................................................ 5500/10223
..............................................................ii.ii........i...i.................... 5600/10223
....i............................................................................................... 5800/10223
............i....................................................................................... 5900/10223
................................................................ii.................................. 6000/10223
...i................................................................................................ 6100/10223
...i................................................................................................ 6100/10223
.................................................................................................... 6200/10223
.................................................................................................... 6300/10223
.........................ii...i..ii...........i..................................................... 6400/10223
.................................................................................................... 6600/10223
.................................................................................................... 6700/10223
.................................................................................................... 6700/10223
..........................................................i..ii..................................... 6800/10223
.................................................................................................... 7000/10223
.................................................................................................... 7100/10223
............i....................................................................................... 7200/10223
.................................................................................................... 7300/10223
---
.................................................................................................... 8100/10223
.................................................................................................... 8200/10223
.................................................................................................... 8300/10223
.......................................i............................................................ 8400/10223
.............................................................................................iiiiii. 8500/10223
iiiiii.i............................................................................................ 8600/10223
.................................................................................................... 8800/10223
.................................................................................................... 8900/10223
.................................................................................................... 9000/10223
.................................................................................................... 9100/10223
---

---- [ui] ui/ast-json/ast-json-noexpand-output.rs stdout ----
diff of stdout:

- {"module":{"inner":{"lo":0,"hi":0},"items":[{"attrs":[],"id":0,"span":{"lo":0,"hi":0},"vis":{"node":"Inherited","span":{"lo":0,"hi":0}},"ident":{"name":"core","span":{"lo":0,"hi":0}},"kind":{"variant":"ExternCrate","fields":[null]},"tokens":{"_field0":[[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["extern",false]},"span":{"lo":0,"hi":0}}]},"NonJoint"],[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["crate",false]},"span":{"lo":0,"hi":0}}]},"NonJoint"],[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["core",false]},"span":{"lo":0,"hi":0}}]},"Joint"],[{"variant":"Token","fields":[{"kind":"Semi","span":{"lo":0,"hi":0}}]},"NonJoint"]]}}],"inline":true},"attrs":[{"kind":{"variant":"Normal","fields":[{"path":{"span":{"lo":0,"hi":0},"segments":[{"ident":{"name":"crate_type","span":{"lo":0,"hi":0}},"id":0,"args":null}]},"args":{"variant":"Eq","fields":[{"lo":0,"hi":0},{"_field0":[[{"variant":"Token","fields":[{"kind":{"variant":"Literal","fields":[{"kind":"Str","symbol":"lib","suffix":null}]},"span":{"lo":0,"hi":0}}]},"NonJoint"]]}]}}]},"id":null,"style":"Inner","span":{"lo":0,"hi":0}}],"span":{"lo":0,"hi":0},"proc_macros":[]}
+ {"module":{"inner":{"lo":0,"hi":0},"items":[{"attrs":[],"id":0,"span":{"lo":0,"hi":0},"vis":{"node":"Inherited","span":{"lo":0,"hi":0}},"ident":{"name":"core","span":{"lo":0,"hi":0}},"kind":{"variant":"ExternCrate","fields":[null]},"tokens":null}],"inline":true},"attrs":[{"kind":{"variant":"Normal","fields":[{"path":{"span":{"lo":0,"hi":0},"segments":[{"ident":{"name":"crate_type","span":{"lo":0,"hi":0}},"id":0,"args":null}]},"args":{"variant":"Eq","fields":[{"lo":0,"hi":0},{"_field0":[[{"variant":"Token","fields":[{"kind":{"variant":"Literal","fields":[{"kind":"Str","symbol":"lib","suffix":null}]},"span":{"lo":0,"hi":0}}]},"NonJoint"]]}]}}]},"id":null,"style":"Inner","span":{"lo":0,"hi":0}}],"span":{"lo":0,"hi":0},"proc_macros":[]}


The actual stdout differed from the expected stdout.
The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/ast-json/ast-json-noexpand-output/ast-json-noexpand-output.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args ast-json/ast-json-noexpand-output.rs`
error: 1 errors occurred comparing output.
error: 1 errors occurred comparing output.
failed to decode compiler output as json: line: {"module":{"inner":{"lo":39,"hi":259},"items":[{"attrs":[],"id":4294967040,"span":{"lo":241,"hi":259},"vis":{"node":"Inherited","span":{"lo":241,"hi":241}},"ident":{"name":"core","span":{"lo":254,"hi":258}},"kind":{"variant":"ExternCrate","fields":[null]},"tokens":null}],"inline":true},"attrs":[{"kind":{"variant":"Normal","fields":[{"path":{"span":{"lo":42,"hi":52},"segments":[{"ident":{"name":"crate_type","span":{"lo":42,"hi":52}},"id":4294967040,"args":null}]},"args":{"variant":"Eq","fields":[{"lo":53,"hi":54},{"_field0":[[{"variant":"Token","fields":[{"kind":{"variant":"Literal","fields":[{"kind":"Str","symbol":"lib","suffix":null}]},"span":{"lo":55,"hi":60}}]},"NonJoint"]]}]}}]},"id":null,"style":"Inner","span":{"lo":39,"hi":61}}],"span":{"lo":39,"hi":259},"proc_macros":[]}
output: {"module":{"inner":{"lo":39,"hi":259},"items":[{"attrs":[],"id":4294967040,"span":{"lo":241,"hi":259},"vis":{"node":"Inherited","span":{"lo":241,"hi":241}},"ident":{"name":"core","span":{"lo":254,"hi":258}},"kind":{"variant":"ExternCrate","fields":[null]},"tokens":null}],"inline":true},"attrs":[{"kind":{"variant":"Normal","fields":[{"path":{"span":{"lo":42,"hi":52},"segments":[{"ident":{"name":"crate_type","span":{"lo":42,"hi":52}},"id":4294967040,"args":null}]},"args":{"variant":"Eq","fields":[{"lo":53,"hi":54},{"_field0":[[{"variant":"Token","fields":[{"kind":{"variant":"Literal","fields":[{"kind":"Str","symbol":"lib","suffix":null}]},"span":{"lo":55,"hi":60}}]},"NonJoint"]]}]}}]},"id":null,"style":"Inner","span":{"lo":39,"hi":61}}],"span":{"lo":39,"hi":259},"proc_macros":[]}
thread '[ui] ui/ast-json/ast-json-noexpand-output.rs' panicked at 'explicit panic', src/tools/compiletest/src/json.rs:87:21

---- [ui] ui/ast-json/ast-json-output.rs stdout ----
diff of stdout:


- {"module":{"inner":{"lo":0,"hi":0},"items":[{"attrs":[{"kind":{"variant":"Normal","fields":[{"path":{"span":{"lo":0,"hi":0},"segments":[{"ident":{"name":"prelude_import","span":{"lo":0,"hi":0}},"id":0,"args":null}]},"args":"Empty"}]},"id":null,"style":"Outer","span":{"lo":0,"hi":0}}],"id":0,"span":{"lo":0,"hi":0},"vis":{"node":"Inherited","span":{"lo":0,"hi":0}},"ident":{"name":"","span":{"lo":0,"hi":0}},"kind":{"variant":"Use","fields":[{"prefix":{"span":{"lo":0,"hi":0},"segments":[{"ident":{"name":"{{root}}","span":{"lo":0,"hi":0}},"id":0,"args":null},{"ident":{"name":"std","span":{"lo":0,"hi":0}},"id":0,"args":null},{"ident":{"name":"prelude","span":{"lo":0,"hi":0}},"id":0,"args":null},{"ident":{"name":"v1","span":{"lo":0,"hi":0}},"id":0,"args":null}]},"kind":"Glob","span":{"lo":0,"hi":0}}]},"tokens":null},{"attrs":[{"kind":{"variant":"Normal","fields":[{"path":{"span":{"lo":0,"hi":0},"segments":[{"ident":{"name":"macro_use","span":{"lo":0,"hi":0}},"id":0,"args":null}]},"args":"Empty"}]},"id":null,"style":"Outer","span":{"lo":0,"hi":0}}],"id":0,"span":{"lo":0,"hi":0},"vis":{"node":"Inherited","span":{"lo":0,"hi":0}},"ident":{"name":"std","span":{"lo":0,"hi":0}},"kind":{"variant":"ExternCrate","fields":[null]},"tokens":null},{"attrs":[],"id":0,"span":{"lo":0,"hi":0},"vis":{"node":"Inherited","span":{"lo":0,"hi":0}},"ident":{"name":"core","span":{"lo":0,"hi":0}},"kind":{"variant":"ExternCrate","fields":[null]},"tokens":{"_field0":[[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["extern",false]},"span":{"lo":0,"hi":0}}]},"NonJoint"],[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["crate",false]},"span":{"lo":0,"hi":0}}]},"NonJoint"],[{"variant":"Token","fields":[{"kind":{"variant":"Ident","fields":["core",false]},"span":{"lo":0,"hi":0}}]},"Joint"],[{"variant":"Token","fields":[{"kind":"Semi","span":{"lo":0,"hi":0}}]},"NonJoint"]]}}],"inline":true},"attrs":[{"kind":{"variant":"Normal","fields":[{"path":{"span":{"lo":0,"hi":0},"segments":[{"ident":{"name":"crate_type","span":{"lo":0,"hi":0}},"id":0,"args":null}]},"args":{"variant":"Eq","fields":[{"lo":0,"hi":0},{"_field0":[[{"variant":"Token","fields":[{"kind":{"variant":"Literal","fields":[{"kind":"Str","symbol":"lib","suffix":null}]},"span":{"lo":0,"hi":0}}]},"NonJoint"]]}]}}]},"id":null,"style":"Inner","span":{"lo":0,"hi":0}}],"span":{"lo":0,"hi":0},"proc_macros":[]}
+ {"module":{"inner":{"lo":0,"hi":0},"items":[{"attrs":[{"kind":{"variant":"Normal","fields":[{"path":{"span":{"lo":0,"hi":0},"segments":[{"ident":{"name":"prelude_import","span":{"lo":0,"hi":0}},"id":0,"args":null}]},"args":"Empty"}]},"id":null,"style":"Outer","span":{"lo":0,"hi":0}}],"id":0,"span":{"lo":0,"hi":0},"vis":{"node":"Inherited","span":{"lo":0,"hi":0}},"ident":{"name":"","span":{"lo":0,"hi":0}},"kind":{"variant":"Use","fields":[{"prefix":{"span":{"lo":0,"hi":0},"segments":[{"ident":{"name":"{{root}}","span":{"lo":0,"hi":0}},"id":0,"args":null},{"ident":{"name":"std","span":{"lo":0,"hi":0}},"id":0,"args":null},{"ident":{"name":"prelude","span":{"lo":0,"hi":0}},"id":0,"args":null},{"ident":{"name":"v1","span":{"lo":0,"hi":0}},"id":0,"args":null}]},"kind":"Glob","span":{"lo":0,"hi":0}}]},"tokens":null},{"attrs":[{"kind":{"variant":"Normal","fields":[{"path":{"span":{"lo":0,"hi":0},"segments":[{"ident":{"name":"macro_use","span":{"lo":0,"hi":0}},"id":0,"args":null}]},"args":"Empty"}]},"id":null,"style":"Outer","span":{"lo":0,"hi":0}}],"id":0,"span":{"lo":0,"hi":0},"vis":{"node":"Inherited","span":{"lo":0,"hi":0}},"ident":{"name":"std","span":{"lo":0,"hi":0}},"kind":{"variant":"ExternCrate","fields":[null]},"tokens":null},{"attrs":[],"id":0,"span":{"lo":0,"hi":0},"vis":{"node":"Inherited","span":{"lo":0,"hi":0}},"ident":{"name":"core","span":{"lo":0,"hi":0}},"kind":{"variant":"ExternCrate","fields":[null]},"tokens":null}],"inline":true},"attrs":[{"kind":{"variant":"Normal","fields":[{"path":{"span":{"lo":0,"hi":0},"segments":[{"ident":{"name":"crate_type","span":{"lo":0,"hi":0}},"id":0,"args":null}]},"args":{"variant":"Eq","fields":[{"lo":0,"hi":0},{"_field0":[[{"variant":"Token","fields":[{"kind":{"variant":"Literal","fields":[{"kind":"Str","symbol":"lib","suffix":null}]},"span":{"lo":0,"hi":0}}]},"NonJoint"]]}]}}]},"id":null,"style":"Inner","span":{"lo":0,"hi":0}}],"span":{"lo":0,"hi":0},"proc_macros":[]}


The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/ast-json/ast-json-output/ast-json-output.stdout
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/ast-json/ast-json-output/ast-json-output.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args ast-json/ast-json-output.rs`
error: 1 errors occurred comparing output.
error: 1 errors occurred comparing output.
failed to decode compiler output as json: line: {"module":{"inner":{"lo":39,"hi":250},"items":[{"attrs":[{"kind":{"variant":"Normal","fields":[{"path":{"span":{"lo":0,"hi":0},"segments":[{"ident":{"name":"prelude_import","span":{"lo":0,"hi":0}},"id":4,"args":null}]},"args":"Empty"}]},"id":null,"style":"Outer","span":{"lo":0,"hi":0}}],"id":5,"span":{"lo":0,"hi":0},"vis":{"node":"Inherited","span":{"lo":0,"hi":0}},"ident":{"name":"","span":{"lo":0,"hi":0}},"kind":{"variant":"Use","fields":[{"prefix":{"span":{"lo":0,"hi":0},"segments":[{"ident":{"name":"{{root}}","span":{"lo":0,"hi":0}},"id":6,"args":null},{"ident":{"name":"std","span":{"lo":0,"hi":0}},"id":7,"args":null},{"ident":{"name":"prelude","span":{"lo":0,"hi":0}},"id":8,"args":null},{"ident":{"name":"v1","span":{"lo":0,"hi":0}},"id":9,"args":null}]},"kind":"Glob","span":{"lo":0,"hi":0}}]},"tokens":null},{"attrs":[{"kind":{"variant":"Normal","fields":[{"path":{"span":{"lo":0,"hi":0},"segments":[{"ident":{"name":"macro_use","span":{"lo":0,"hi":0}},"id":10,"args":null}]},"args":"Empty"}]},"id":null,"style":"Outer","span":{"lo":0,"hi":0}}],"id":11,"span":{"lo":0,"hi":0},"vis":{"node":"Inherited","span":{"lo":0,"hi":0}},"ident":{"name":"std","span":{"lo":0,"hi":0}},"kind":{"variant":"ExternCrate","fields":[null]},"tokens":null},{"attrs":[],"id":12,"span":{"lo":232,"hi":250},"vis":{"node":"Inherited","span":{"lo":232,"hi":232}},"ident":{"name":"core","span":{"lo":245,"hi":249}},"kind":{"variant":"ExternCrate","fields":[null]},"tokens":null}],"inline":true},"attrs":[{"kind":{"variant":"Normal","fields":[{"path":{"span":{"lo":42,"hi":52},"segments":[{"ident":{"name":"crate_type","span":{"lo":42,"hi":52}},"id":2,"args":null}]},"args":{"variant":"Eq","fields":[{"lo":53,"hi":54},{"_field0":[[{"variant":"Token","fields":[{"kind":{"variant":"Literal","fields":[{"kind":"Str","symbol":"lib","suffix":null}]},"span":{"lo":55,"hi":60}}]},"NonJoint"]]}]}}]},"id":null,"style":"Inner","span":{"lo":39,"hi":61}}],"span":{"lo":39,"hi":250},"proc_macros":[]}
output: {"module":{"inner":{"lo":39,"hi":250},"items":[{"attrs":[{"kind":{"variant":"Normal","fields":[{"path":{"span":{"lo":0,"hi":0},"segments":[{"ident":{"name":"prelude_import","span":{"lo":0,"hi":0}},"id":4,"args":null}]},"args":"Empty"}]},"id":null,"style":"Outer","span":{"lo":0,"hi":0}}],"id":5,"span":{"lo":0,"hi":0},"vis":{"node":"Inherited","span":{"lo":0,"hi":0}},"ident":{"name":"","span":{"lo":0,"hi":0}},"kind":{"variant":"Use","fields":[{"prefix":{"span":{"lo":0,"hi":0},"segments":[{"ident":{"name":"{{root}}","span":{"lo":0,"hi":0}},"id":6,"args":null},{"ident":{"name":"std","span":{"lo":0,"hi":0}},"id":7,"args":null},{"ident":{"name":"prelude","span":{"lo":0,"hi":0}},"id":8,"args":null},{"ident":{"name":"v1","span":{"lo":0,"hi":0}},"id":9,"args":null}]},"kind":"Glob","span":{"lo":0,"hi":0}}]},"tokens":null},{"attrs":[{"kind":{"variant":"Normal","fields":[{"path":{"span":{"lo":0,"hi":0},"segments":[{"ident":{"name":"macro_use","span":{"lo":0,"hi":0}},"id":10,"args":null}]},"args":"Empty"}]},"id":null,"style":"Outer","span":{"lo":0,"hi":0}}],"id":11,"span":{"lo":0,"hi":0},"vis":{"node":"Inherited","span":{"lo":0,"hi":0}},"ident":{"name":"std","span":{"lo":0,"hi":0}},"kind":{"variant":"ExternCrate","fields":[null]},"tokens":null},{"attrs":[],"id":12,"span":{"lo":232,"hi":250},"vis":{"node":"Inherited","span":{"lo":232,"hi":232}},"ident":{"name":"core","span":{"lo":245,"hi":249}},"kind":{"variant":"ExternCrate","fields":[null]},"tokens":null}],"inline":true},"attrs":[{"kind":{"variant":"Normal","fields":[{"path":{"span":{"lo":42,"hi":52},"segments":[{"ident":{"name":"crate_type","span":{"lo":42,"hi":52}},"id":2,"args":null}]},"args":{"variant":"Eq","fields":[{"lo":53,"hi":54},{"_field0":[[{"variant":"Token","fields":[{"kind":{"variant":"Literal","fields":[{"kind":"Str","symbol":"lib","suffix":null}]},"span":{"lo":55,"hi":60}}]},"NonJoint"]]}]}}]},"id":null,"style":"Inner","span":{"lo":39,"hi":61}}],"span":{"lo":39,"hi":250},"proc_macros":[]}


failures:
    [ui] ui/ast-json/ast-json-noexpand-output.rs
    [ui] ui/ast-json/ast-json-noexpand-output.rs
    [ui] ui/ast-json/ast-json-output.rs

test result: FAILED. 10158 passed; 2 failed; 63 ignored; 0 measured; 0 filtered out

thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-8/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "8.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
Build completed unsuccessfully in 1:03:55
Build completed unsuccessfully in 1:03:55
== clock drift check ==
  local time: Mon May 25 21:04:25 UTC 2020
  network time: Mon, 25 May 2020 21:04:26 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/72585/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/72585/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (4342) (python)
##[section]Finishing: Finalize Job
