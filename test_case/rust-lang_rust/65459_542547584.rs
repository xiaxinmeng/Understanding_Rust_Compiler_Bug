plain
2019-10-16T05:28:31.2236143Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-16T05:28:31.2441595Z ##[command]git config gc.auto 0
2019-10-16T05:28:31.2545852Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-16T05:28:31.2649939Z ##[command]git config --get-all http.proxy
2019-10-16T05:28:31.2802282Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65459/merge:refs/remotes/pull/65459/merge
---
2019-10-16T06:32:52.8142508Z .................................................................................................... 1600/9196
2019-10-16T06:32:58.2894758Z .................................................................................................... 1700/9196
2019-10-16T06:33:11.7834349Z .............................i...............i...................................................... 1800/9196
2019-10-16T06:33:19.6065887Z .................................................................................................... 1900/9196
2019-10-16T06:33:34.5214427Z ...................iiiii............................................................................ 2000/9196
2019-10-16T06:33:45.5471860Z .................................................................................................... 2200/9196
2019-10-16T06:33:48.2005081Z .................................................................................................... 2300/9196
2019-10-16T06:33:53.8561964Z .................................................................................................... 2400/9196
2019-10-16T06:34:16.8425035Z .................................................................................................... 2500/9196
---
2019-10-16T06:37:21.9403110Z ......................i...............i............................................................. 4800/9196
2019-10-16T06:37:34.1288913Z .................................................................................................... 4900/9196
2019-10-16T06:37:40.8727614Z .................................................................................................... 5000/9196
2019-10-16T06:37:51.8680481Z .................................................................................................... 5100/9196
2019-10-16T06:37:58.4949950Z ......................ii.ii......................................................................... 5200/9196
2019-10-16T06:38:09.7211081Z .................................................................................................... 5400/9196
2019-10-16T06:38:20.3153098Z ........................................................................................i........... 5500/9196
2019-10-16T06:38:28.8941570Z .................................................................................................... 5600/9196
2019-10-16T06:38:34.2954874Z .................................................................................................... 5700/9196
2019-10-16T06:38:34.2954874Z .................................................................................................... 5700/9196
2019-10-16T06:38:45.4829487Z .....................................................................................ii...i..ii..... 5800/9196
2019-10-16T06:39:14.0573365Z .................................................................................................... 6000/9196
2019-10-16T06:39:24.2799508Z .................................................................................................... 6100/9196
2019-10-16T06:39:33.4725265Z .................................................................................................... 6200/9196
2019-10-16T06:39:33.4725265Z .................................................................................................... 6200/9196
2019-10-16T06:39:48.1607015Z .......i..ii........................................................................................ 6300/9196
2019-10-16T06:40:08.8190889Z ...................................................................i................................ 6500/9196
2019-10-16T06:40:11.1705919Z .................................................................................................... 6600/9196
2019-10-16T06:40:13.8194034Z .........................................i.......................................................... 6700/9196
2019-10-16T06:40:17.9888291Z .................................................................................................... 6800/9196
---
2019-10-16T06:44:58.1807404Z running 57 tests
2019-10-16T06:45:03.6409005Z .....................F...................................
2019-10-16T06:45:03.6409280Z failures:
2019-10-16T06:45:03.6409784Z 
2019-10-16T06:45:03.6417296Z ---- [mir-opt] mir-opt/graphviz.rs stdout ----
2019-10-16T06:45:03.6417373Z [ERROR compiletest::runtest] None
2019-10-16T06:45:03.6418699Z thread '[mir-opt] mir-opt/graphviz.rs' panicked at 'Did not find expected line, error: Mismatch in lines
2019-10-16T06:45:03.6419124Z Current block: None
2019-10-16T06:45:03.6419200Z Actual Line: "    bb0__0_12 [shape=\"none\", label=<<table border=\"0\" cellborder=\"1\" cellspacing=\"0\"><tr><td bgcolor=\"gray\" align=\"center\" colspan=\"1\">0</td></tr><tr><td align=\"left\" balign=\"left\">_0 = ()<br/></td></tr><tr><td align=\"left\">goto</td></tr></table>>];"
2019-10-16T06:45:03.6420373Z Expected Line: "    bb0 [shape=\"none\", label=<<table border=\"0\" cellborder=\"1\" cellspacing=\"0\"><tr><td bgcolor=\"gray\" align=\"center\" colspan=\"1\">0</td></tr><tr><td align=\"left\" balign=\"left\">_0 = ()<br/></td></tr><tr><td align=\"left\">goto</td></tr></table>"
2019-10-16T06:45:03.6420580Z Test Name: rustc.main.mir_map.0.dot
2019-10-16T06:45:03.6420892Z ... (elided)
2019-10-16T06:45:03.6420892Z ... (elided)
2019-10-16T06:45:03.6421115Z digraph Mir_0_12 { // The name here MUST be an ASCII identifier.
2019-10-16T06:45:03.6421295Z     graph [fontname="monospace"];
2019-10-16T06:45:03.6421397Z     node [fontname="monospace"];
2019-10-16T06:45:03.6421441Z     edge [fontname="monospace"];
2019-10-16T06:45:03.6422258Z     label=<fn main() -&gt; ()<br align="left"/>>;
2019-10-16T06:45:03.6422375Z     bb0 [shape="none", label=<<table border="0" cellborder="1" cellspacing="0"><tr><td bgcolor="gray" align="center" colspan="1">0</td></tr><tr><td align="left" balign="left">_0 = ()<br/></td></tr><tr><td align="left">goto</td></tr></table>
2019-10-16T06:45:03.6423046Z >];
2019-10-16T06:45:03.6423230Z     bb1 [shape="none", label=<<table border="0" cellborder="1" cellspacing="0"><tr><td bgcolor="gray" align="center" colspan="1">1</td></tr><tr><td align="left">resume</td></tr></table>
2019-10-16T06:45:03.6423320Z >];
2019-10-16T06:45:03.6423384Z     bb2 [shape="none", label=<<table border="0" cellborder="1" cellspacing="0"><tr><td bgcolor="gray" align="center" colspan="1">2</td></tr><tr><td align="left">return</td></tr></table>
2019-10-16T06:45:03.6423449Z >];
2019-10-16T06:45:03.6424220Z     bb0 -> bb2 [label=""];
2019-10-16T06:45:03.6425308Z Actual:
2019-10-16T06:45:03.6425308Z Actual:
2019-10-16T06:45:03.6425372Z digraph Mir_0_12 {
2019-10-16T06:45:03.6425415Z     graph [fontname="monospace"];
2019-10-16T06:45:03.6425458Z     node [fontname="monospace"];
2019-10-16T06:45:03.6425517Z     edge [fontname="monospace"];
2019-10-16T06:45:03.6425888Z     label=<fn main() -&gt; ()<br align="left"/>>;
2019-10-16T06:45:03.6426150Z     bb0__0_12 [shape="none", label=<<table border="0" cellborder="1" cellspacing="0"><tr><td bgcolor="gray" align="center" colspan="1">0</td></tr><tr><td align="left" balign="left">_0 = ()<br/></td></tr><tr><td align="left">goto</td></tr></table>>];
2019-10-16T06:45:03.6426436Z     bb1__0_12 [shape="none", label=<<table border="0" cellborder="1" cellspacing="0"><tr><td bgcolor="gray" align="center" colspan="1">1</td></tr><tr><td align="left">resume</td></tr></table>>];
2019-10-16T06:45:03.6426510Z     bb2__0_12 [shape="none", label=<<table border="0" cellborder="1" cellspacing="0"><tr><td bgcolor="gray" align="center" colspan="1">2</td></tr><tr><td align="left">return</td></tr></table>>];
2019-10-16T06:45:03.6427310Z     bb0__0_12 -> bb2__0_12 [label=""];
2019-10-16T06:45:03.6428128Z }', src/tools/compiletest/src/runtest.rs:3296:13
2019-10-16T06:45:03.6428226Z 
2019-10-16T06:45:03.6428286Z 
2019-10-16T06:45:03.6428476Z failures:
2019-10-16T06:45:03.6428999Z     [mir-opt] mir-opt/graphviz.rs
2019-10-16T06:45:03.6428999Z     [mir-opt] mir-opt/graphviz.rs
2019-10-16T06:45:03.6430204Z 
2019-10-16T06:45:03.6431440Z test result: FAILED. 56 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
2019-10-16T06:45:03.6431492Z 
2019-10-16T06:45:03.6440853Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-16T06:45:03.6445479Z 
2019-10-16T06:45:03.6445722Z 
2019-10-16T06:45:03.6458761Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-16T06:45:03.6463344Z 
2019-10-16T06:45:03.6463376Z 
2019-10-16T06:45:03.6466286Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-16T06:45:03.6466545Z Build completed unsuccessfully in 1:09:18
2019-10-16T06:45:03.6466545Z Build completed unsuccessfully in 1:09:18
2019-10-16T06:45:03.6525917Z == clock drift check ==
2019-10-16T06:45:03.6544118Z   local time: Wed Oct 16 06:45:03 UTC 2019
2019-10-16T06:45:04.1889939Z   network time: Wed, 16 Oct 2019 06:45:04 GMT
2019-10-16T06:45:04.1893422Z == end clock drift check ==
2019-10-16T06:45:08.6370593Z ##[error]Bash exited with code '1'.
2019-10-16T06:45:08.6413268Z ##[section]Starting: Checkout
2019-10-16T06:45:08.6415467Z ==============================================================================
2019-10-16T06:45:08.6415548Z Task         : Get sources
2019-10-16T06:45:08.6415603Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
