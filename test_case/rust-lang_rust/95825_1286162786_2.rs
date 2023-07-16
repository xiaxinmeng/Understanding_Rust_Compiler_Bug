shell
03918badd33d255de806b4a9a8aa75b031ed0738 2022-03-07 Auto merge of #94706 - matthiaskrgr:rollup-l5erynr, r=matthiaskrgr
fbd4cfa0f8272a5c74d6c5ed882032a08d5b8d09 2022-03-07 diagnostics: only talk about `Cargo.toml` if running under Cargo
170b02702277229ccaae3ffed916bf6dc57548fc 2022-03-07 Add comments based on code review feedback
840dc1047112294387a7cc16536315ab306c2de2 2022-03-07 Update clippy to new ExprUseVisitor delegate
9f0f46fa4da35bea07c3bedd7bd9e6742d375a27 2022-03-07 Update clippy to new ExprUseVisitor delegate
87ad6683d6cc3c1e2597a3a37e03fe4330414fd3 2022-03-07 Distinguish borrows of copies from other borrows
ac804f27a8ce2e02fd3a9c5eff83098258b78da6 2022-03-07 Trying to detect autorefs to avoid unnecessary borrowed temporaries
513a9c67a59c0e1b9a7903d7e5fca4b6da974673 2022-03-07 Move test to right place
60f5cad6ebaa683ac58132b7bb64002a90deb343 2022-03-07 try to fix issue 57017, but not quite there yet
b3261f8ce4e0a09e0534794a30f2570abe85be63 2022-03-07 Rollup merge of #94700 - GuillaumeGomez:update-minifier, r=notriddle
77562f2350c4dbe2d19a253f70046fb3edd9baa6 2022-03-07 Rollup merge of #94696 - GuillaumeGomez:align-line-numbers-right, r=notriddle
9d7166c66f7d2cd7d2e51abae3e42a520fb359e0 2022-03-07 Rollup merge of #93827 - eholk:stabilize-const_fn-features, r=wesleywiser
1ca8d0bf8c154b0f602fadac8c38f14df5dea77a 2022-03-07 Rollup merge of #93350 - gburgessiv:master, r=Mark-Simulacrum
3f4a095d3e8166d710919e06e21fc4cbc056b0ee 2022-03-07 Merge #11647
8e3057d0a72ccea84508cca7f3faaf9531349ac7 2022-03-07 Improve inlay hint padding
165b5583e56ad6d54922a7b5117a37c2ca2c89a1 2022-03-07 Fix typos in `LintExpectationId` docs
47f3f66240eb20da1cd7583115a6ea1a2d25e388 2022-03-07 Update unstable `ExpectationId`s in stored diagnostics
c03575275a56112edacd1c5cd26a8eb2d4763b74 2022-03-07 update recommended CI snippet, add GHA example
49646b71d49cc3c94aa0105223be0ff71cbb6b7d 2022-03-07 Merge #11445
88a2141b7702a553050dc46653251550300e6269 2022-03-07 Add inlayHints cap
43ce0a94af2539f28463197010a5f4147e33a0e0 2022-03-07 Update and fix clippy tests
bb6bcaa1deaa0102ffb9fd23e27f427939de43a1 2022-03-07 Update tests
1afbf3e4b1cbe1eb466da3da2319770db8ff5e9d 2022-03-07 Bump stabilization version to 1.61.0
8700b45b67b2cc138718a17fdb606db0944d03aa 2022-03-07 Stabilize const_impl_trait as well
801be21d11806d37bacd7c7adaeb93a33871f31e 2022-03-07 Remove dead/useless code
8e93a48c32852b09f4270348883b61edba5bb068 2022-03-07 Update and fix clippy tests
8fc835831c1a84ba27d8bbafec480f02b2e29663 2022-03-07 Update tests after feature stabilization
b86620da377102a1a5e84a96910b70c9c957b810 2022-03-07 Stabilize const_fn_fn_ptr_basics and const_fn_trait_bound
7723506d13a03e3191d407aaf3709f157bf75193 2022-03-07 Stabilize const_fn_fn_ptr_basics and const_fn_trait_bound
96515f421a9efe88b46091e58533c1bfe875e09e 2022-03-07 Do not allow `#[rustc_legacy_const_generics]` on methods
3e7035fa86af022ed047e7a1b1540584e2b61234 2022-03-07 Auto merge of #94690 - nnethercote:clarify-Layout-interning, r=fee1-dead
ecb867ec3cefa97a5807453a68758392730e3ed9 2022-03-07 Auto merge of #94690 - nnethercote:clarify-Layout-interning, r=fee1-dead
64b086a8e26cbf7daddcf4581e6351a55718a913 2022-03-07 Auto merge of #2004 - RalfJung:simd, r=RalfJung
ffa59765e0114de175e2489845b06e0377c77923 2022-03-07 Gather more profile data for LLVM PGO
b87a9c90e1ca983eb81778d79b23db8c52eace54 2022-03-07 fix handling of NaNs in simd max/min
2f97eb68a0f77d3829151bc57855d42535465a6d 2022-03-07 implement simd_fmax/fmin
9851b743c1a4a1fdb62579c2c8bb6d6f543a7028 2022-03-07 implement simd_reduce_min/max
db06d4998fa8e64c290a7ae439fb2f8aefb2223e 2022-03-07 implement simd_cast, simd_as
dd42a47f0a458380a1ea03ec6ef360ece3674692 2022-03-07 Auto merge of #2005 - RalfJung:rustup, r=RalfJung
594a70a28901efa14c8119242c5692027dbd8eb9 2022-03-07 rustup
a496fa4fc15276da2a47b84bcc43824a7c0c667b 2022-03-07 Update minifier version
97109d73f89d050dd92194485374d3bd6925b3c1 2022-03-07 Add missing parameter
26d2e88b0424740edf6aad03bccb62315bd334e2 2022-03-07 Fix parameter hint position
04128de55396b256b593422315f7af87b7598190 2022-03-07 Bump esbuild
b9d74fe2ed578f0b48f7773cde432df80531bb0b 2022-03-07 Update LSP docs
d137c3a7bd3b180317044f8ccb9a8b4b3bb07db3 2022-03-07 Auto merge of #94695 - matthiaskrgr:rollup-5pi3acz, r=matthiaskrgr
5b5649fd73755eb2d9c4473efad0a1f264e3cfd0 2022-03-07 Add missing documentation for std::char types
36bb53d4978cdf1f35d1436ad77998bddbaaa5bd 2022-03-07 BTree: remove dead data needlessly complicating insert
18d0faf027a0ea2808b842f2dfbd7b7e12438f44 2022-03-07 Merge #11645
6e62d93c9541080f45c964d0afe2e2df723dede5 2022-03-07 Update manual.adoc
f23d6d3a47c15bc0fc3fbf2abdd318c3a2c09c7c 2022-03-07 Add GUI test to ensure that line numbers text is aligned to the right
e89efb8634a1ffc8e7573b6f4cf53aa9b36d92ae 2022-03-07 Remove unneeded whitespace generation and use CSS instead instead to align line numbers to the right
c0c452ba9bd66eac4e6bee9508bbf5be273f5b13 2022-03-07 Rollup merge of #94688 - compiler-errors:free-regions-in-copy-predicate-check, r=oli-obk
a795f0f5367d35c7c5e8f03228037ae0aa0fee6a 2022-03-07 Rollup merge of #94685 - RalfJung:saturating, r=oli-obk
87df3f663d302431d745172f35f34997b05945d1 2022-03-07 Rollup merge of #94614 - pierwill:localexpnid-noord, r=lcnr
2c98edaaa795d3b203052ec4d738d188598b47b4 2022-03-07 Rollup merge of #94553 - lcnr:add-tests, r=Dylan-DPC
db3fcf8df7269f9265e4643d4aa81f11d550e06b 2022-03-07 add basic code to check nop match blocks modify `manual_map_option` uitest because one test case has confliction.
31ad347b0ef7f2e98496b1cf7a99bff5e59d87c4 2022-03-07 Merge #11644
6da122889856f40bd165c68c6daf58de23def98e 2022-03-07 Emit more detailed highlighting for `%`, `>>`, `<<`
297273c45b205820a4c055082c71677197a40b55 2022-03-07 Auto merge of #94692 - matthiaskrgr:rollup-64p7ya7, r=matthiaskrgr
dad81b65db1b67be7f24b2a418c66db10aecd4b5 2022-03-07 add tests for #94502
a1119fd6999aa034d026d1d97d4acff8a2662f18 2022-03-07 Rollup merge of #94684 - compiler-errors:gat-anon-late-bound, r=notriddle
f7eb3830df557c66512bf2424cc03fa8de888c9c 2022-03-07 Rollup merge of #94681 - RalfJung:miri-cast, r=oli-obk
02539e16121a3ad81172892753a7b1ed09746acd 2022-03-07 Rollup merge of #94676 - TaKO8Ki:remove-unnecessary-pattens-for-ignoring-remaining-parts, r=Dylan-DPC
9c7ff1add7c395686831059f609efe3f530bb98b 2022-03-07 Rollup merge of #94636 - compiler-errors:issue-94599, r=davidtwco
2631aeef823a9e16d31f999d3f07001e5fcc4b3d 2022-03-07 Auto merge of #94272 - tavianator:readdir-reclen-for-real, r=cuviper
b726bfb56902eaa743f7164ce502afbdd232562c 2022-03-06 allow referencing impl substs from rustc_on_unimplemented
e4766776d24a17cc1500c8e420a00c4c0e05423b 2022-03-07 add test for trait recursion
e9ddb8f8fb6fb42d533deb9b092e34c046b45b66 2022-03-06 use impl substs in on_unimplemented
8723fe0b6b6f8563c2ca0f15c83adf309e780d2a 2022-03-07 Clarify `Layout` interning.
b36924b4ac73c4eab2fd9e4cc2486d5fa1b6bf41 2022-03-07 Clarify `Layout` interning.
d35fc85a44b99022d7e7fd506b72e2bbfe5c19bb 2022-03-07 Clarify `Layout` interning.
4f008e06c33e1ef729bce839c0899154e71902b0 2022-03-07 Clarify `Layout` interning.
0e36868bc7a94c5a282ff3598744c72e621276ca 2022-03-07 Auto merge of #94638 - erikdesjardins:noextranull, r=nagisa
3d1eaf4b6203ab0168da5b99049942385aa753e0 2022-03-07 Auto merge of #94638 - erikdesjardins:noextranull, r=nagisa
5ddaa2d5e53344ff8b88d98e61e3dd36edb803e9 2022-03-06 Erase regions when checking for missing Copy predicates
ac844986d8f5646b686e2a5619e5a90372953b38 2022-03-06 use singed_int_max/min helper methods
956659e5ced806360b726c103eb7ab1d960c163a 2022-03-06 move saturating_add/sub into (pub) helper method
890a44f66b355d40a284349a22ac82aea82f0520 2022-03-06 Fix rustdoc for GATs with with anonymous bound regions
8876ca3dd46b99fe7e6ad937f11493d37996231e 2022-03-06 Auto merge of #94597 - nnethercote:ConstAllocation, r=fee1-dead
96eb1168d177a8e37fb4866051e665e3f1988c25 2022-03-07 Introduce `ConstAllocation`.
7c1a318c7b801e7020918b238428f484f0f13aa7 2022-03-07 Introduce `ConstAllocation`.
92d1850f17f8a2f0e300e2350cc9a53463d0f23a 2022-03-07 Introduce `ConstAllocation`.
4852291417127d86c3f8404ef03cb1706d89a3e6 2022-03-07 Introduce `ConstAllocation`.
38a0b81b1c32764d6a583a5efb6f306b8c44c503 2022-03-06 Auto merge of #94679 - matthiaskrgr:rollup-9vd7w6a, r=matthiaskrgr
