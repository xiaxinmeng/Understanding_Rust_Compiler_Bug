plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 14'
Agent machine name: 'fv-az578'
Current agent version: '2.169.1'
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
Downloading task: Bash (3.163.3)
Checking job knob settings.
   Knob: AgentToolsDirectory = /opt/hostedtoolcache Source: ${AGENT_TOOLSDIRECTORY} 
   Knob: AgentPerflog = /home/vsts/perflog Source: ${VSTS_AGENT_PERFLOG} 
Start tracking orphan processes.
##[section]Finishing: Initialize job
##[section]Starting: Configure Job Name
==============================================================================
---
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/7d7ec2c7-857e-4f79-8987-bb17cbcb0af6.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/73133/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/73133/merge:refs/remotes/pull/73133/merge
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
   Compiling rustc_parse_format v0.0.0 (/checkout/src/librustc_parse_format)
   Compiling chalk-rust-ir v0.10.0
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling chalk-solve v0.10.0
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
   Compiling hashbrown v0.6.2
   Compiling rustc-std-workspace-core v1.99.0 (/checkout/src/tools/rustc-std-workspace-core)
   Compiling cfg-if v0.1.10
   Compiling alloc v0.0.0 (/checkout/src/liballoc)
error: internal compiler error: broken MIR in DefId(0:343 ~ core[a39f]::num[0]::dec2flt[0]::num[0]::from_str_unchecked[0]) (end of phase Optimized) at bb4[0]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/num/dec2flt/num.rs:39:1: 48:2, scope: scope[0] }, kind: resume }), is_cleanup: true }
  --> src/libcore/num/dec2flt/num.rs:46:5
46 |     }
   |     ^


error: internal compiler error: broken MIR in DefId(0:459 ~ core[a39f]::num[0]::dec2flt[0]::rawfp[0]::encode_normal[0]::{{closure}}[0]) (end of phase Optimized) at bb2[0]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/num/dec2flt/rawfp.rs:295:49: 295:67, scope: scope[0] }, kind: resume }), is_cleanup: true }
    |
534 | / macro_rules! unreachable {
535 | |     () => ({
536 | |         panic!("internal error: entered unreachable code")
536 | |         panic!("internal error: entered unreachable code")
537 | |     });
    | |     ^
...   |
546 | |     });
547 | | }
    | |_- in this expansion of `unreachable!`
    | 
   ::: src/libcore/num/dec2flt/rawfp.rs:295:53
    |
295 |       T::from_bits(bits.try_into().unwrap_or_else(|_| unreachable!()))


error: internal compiler error: broken MIR in DefId(0:462 ~ core[a39f]::num[0]::dec2flt[0]::rawfp[0]::encode_subnormal[0]::{{closure}}[0]) (end of phase Optimized) at bb2[0]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/num/dec2flt/rawfp.rs:302:56: 302:74, scope: scope[0] }, kind: resume }), is_cleanup: true }
    |
534 | / macro_rules! unreachable {
535 | |     () => ({
536 | |         panic!("internal error: entered unreachable code")
536 | |         panic!("internal error: entered unreachable code")
537 | |     });
    | |     ^
...   |
546 | |     });
547 | | }
    | |_- in this expansion of `unreachable!`
    | 
   ::: src/libcore/num/dec2flt/rawfp.rs:302:60
    |
302 |       T::from_bits(significand.try_into().unwrap_or_else(|_| unreachable!()))


error: internal compiler error: broken MIR in DefId(0:466 ~ core[a39f]::num[0]::dec2flt[0]::rawfp[0]::next_float[0]) (end of phase Optimized) at bb12[1]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/num/dec2flt/rawfp.rs:349:1: 363:2, scope: scope[0] }, kind: resume }), is_cleanup: true }
   --> src/libcore/num/dec2flt/rawfp.rs:361:82
    |
361 |         Zero | Subnormal | Normal => T::from_bits(x.to_bits() + T::Bits::from(1u8)),


error: internal compiler error: broken MIR in DefId(0:466 ~ core[a39f]::num[0]::dec2flt[0]::rawfp[0]::next_float[0]) (end of phase Optimized) at bb13[0]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/num/dec2flt/rawfp.rs:349:1: 363:2, scope: scope[0] }, kind: resume }), is_cleanup: true }
   --> src/libcore/num/dec2flt/rawfp.rs:361:82
    |
361 |         Zero | Subnormal | Normal => T::from_bits(x.to_bits() + T::Bits::from(1u8)),


error: internal compiler error: broken MIR in DefId(0:466 ~ core[a39f]::num[0]::dec2flt[0]::rawfp[0]::next_float[0]) (end of phase Optimized) at bb13[0]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [_11 = const false], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/num/dec2flt/rawfp.rs:361:82: 361:83, scope: scope[0] }, kind: drop(_8) -> bb1 }), is_cleanup: true }
   --> src/libcore/num/dec2flt/rawfp.rs:361:82
    |
361 |         Zero | Subnormal | Normal => T::from_bits(x.to_bits() + T::Bits::from(1u8)),


error: internal compiler error: broken MIR in DefId(0:587 ~ core[a39f]::num[0]::flt2dec[0]::to_shortest_str[0]) (end of phase Optimized) at bb3[0]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/num/flt2dec/mod.rs:459:1: 501:2, scope: scope[0] }, kind: resume }), is_cleanup: true }
   --> src/libcore/num/flt2dec/mod.rs:501:1
501 | }
    | ^


error: internal compiler error: broken MIR in DefId(0:591 ~ core[a39f]::num[0]::flt2dec[0]::to_shortest_exp_str[0]) (end of phase Optimized) at bb3[0]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/num/flt2dec/mod.rs:522:1: 569:2, scope: scope[0] }, kind: resume }), is_cleanup: true }
   --> src/libcore/num/flt2dec/mod.rs:569:1
569 | }
    | ^


error: internal compiler error: broken MIR in DefId(0:596 ~ core[a39f]::num[0]::flt2dec[0]::to_exact_exp_str[0]) (end of phase Optimized) at bb3[0]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/num/flt2dec/mod.rs:610:1: 658:2, scope: scope[0] }, kind: resume }), is_cleanup: true }
   --> src/libcore/num/flt2dec/mod.rs:658:1
658 | }
    | ^


error: internal compiler error: broken MIR in DefId(0:600 ~ core[a39f]::num[0]::flt2dec[0]::to_exact_fixed_str[0]) (end of phase Optimized) at bb3[0]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/num/flt2dec/mod.rs:675:1: 739:2, scope: scope[0] }, kind: resume }), is_cleanup: true }
   --> src/libcore/num/flt2dec/mod.rs:739:1
739 | }
    | ^


error: internal compiler error: broken MIR in DefId(0:1237 ~ core[a39f]::mem[0]::replace[0]) (end of phase Optimized) at bb3[0]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/mem/mod.rs:804:1: 807:2, scope: scope[0] }, kind: resume }), is_cleanup: true }
    |
807 | }
    | ^


error: internal compiler error: broken MIR in DefId(0:1537 ~ core[a39f]::ptr[0]::swap_nonoverlapping_one[0]) (end of phase Optimized) at bb10[1]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/ptr/mod.rs:434:1: 444:2, scope: scope[0] }, kind: resume }), is_cleanup: true }
    |
441 |     } else {
    |     ^


error: internal compiler error: broken MIR in DefId(0:1537 ~ core[a39f]::ptr[0]::swap_nonoverlapping_one[0]) (end of phase Optimized) at bb11[0]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/ptr/mod.rs:434:1: 444:2, scope: scope[0] }, kind: resume }), is_cleanup: true }
    |
441 |     } else {
    |     ^


error: internal compiler error: broken MIR in DefId(0:1537 ~ core[a39f]::ptr[0]::swap_nonoverlapping_one[0]) (end of phase Optimized) at bb11[0]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [_18 = const false], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/ptr/mod.rs:441:5: 441:6, scope: scope[0] }, kind: drop(_5) -> bb1 }), is_cleanup: true }
    |
441 |     } else {
    |     ^


error: internal compiler error: broken MIR in DefId(0:1552 ~ core[a39f]::ptr[0]::replace[0]) (end of phase Optimized) at bb3[0]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/ptr/mod.rs:537:1: 540:2, scope: scope[0] }, kind: resume }), is_cleanup: true }
    |
540 | }
    | ^


error: internal compiler error: broken MIR in DefId(0:1560 ~ core[a39f]::ptr[0]::write_unaligned[0]) (end of phase Optimized) at bb5[1]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/ptr/mod.rs:933:1: 937:2, scope: scope[0] }, kind: resume }), is_cleanup: true }
    |
937 | }
    | ^


error: internal compiler error: broken MIR in DefId(0:1560 ~ core[a39f]::ptr[0]::write_unaligned[0]) (end of phase Optimized) at bb6[0]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/ptr/mod.rs:933:1: 937:2, scope: scope[0] }, kind: resume }), is_cleanup: true }
    |
937 | }
    | ^


error: internal compiler error: broken MIR in DefId(0:1560 ~ core[a39f]::ptr[0]::write_unaligned[0]) (end of phase Optimized) at bb6[0]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [_12 = const false], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/ptr/mod.rs:937:1: 937:2, scope: scope[0] }, kind: drop(_2) -> bb1 }), is_cleanup: true }
    |
937 | }
    | ^


error: internal compiler error: broken MIR in DefId(0:1564 ~ core[a39f]::ptr[0]::write_volatile[0]) (end of phase Optimized) at bb3[1]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/ptr/mod.rs:1074:1: 1077:2, scope: scope[0] }, kind: resume }), is_cleanup: true }
     |
1077 | }
     | ^


error: internal compiler error: broken MIR in DefId(0:1564 ~ core[a39f]::ptr[0]::write_volatile[0]) (end of phase Optimized) at bb4[0]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/ptr/mod.rs:1074:1: 1077:2, scope: scope[0] }, kind: resume }), is_cleanup: true }
     |
1077 | }
     | ^


error: internal compiler error: broken MIR in DefId(0:1564 ~ core[a39f]::ptr[0]::write_volatile[0]) (end of phase Optimized) at bb4[0]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [_6 = const false], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/ptr/mod.rs:1077:1: 1077:2, scope: scope[0] }, kind: drop(_2) -> bb1 }), is_cleanup: true }
     |
1077 | }
     | ^


error: internal compiler error: broken MIR in DefId(0:1662 ~ core[a39f]::cmp[0]::Ord[0]::clamp[0]) (end of phase Optimized) at bb3[0]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/cmp.rs:647:5: 659:6, scope: scope[0] }, kind: resume }), is_cleanup: true }
   --> src/libcore/cmp.rs:659:5
659 |     }
    |     ^


error: internal compiler error: broken MIR in DefId(0:1662 ~ core[a39f]::cmp[0]::Ord[0]::clamp[0]) (end of phase Optimized) at bb3[0]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [_16 = const false], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/cmp.rs:659:5: 659:6, scope: scope[0] }, kind: drop(_1) -> bb1 }), is_cleanup: true }
   --> src/libcore/cmp.rs:659:5
659 |     }
    |     ^


error: internal compiler error: broken MIR in DefId(0:1662 ~ core[a39f]::cmp[0]::Ord[0]::clamp[0]) (end of phase Optimized) at bb4[0]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/cmp.rs:659:5: 659:6, scope: scope[0] }, kind: switchInt(_16) -> [false: bb1, otherwise: bb19] }), is_cleanup: true }
   --> src/libcore/cmp.rs:659:5
659 |     }
    |     ^


error: internal compiler error: broken MIR in DefId(0:1662 ~ core[a39f]::cmp[0]::Ord[0]::clamp[0]) (end of phase Optimized) at bb4[0]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [_17 = const false], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/cmp.rs:659:5: 659:6, scope: scope[0] }, kind: drop(_2) -> bb3 }), is_cleanup: true }
   --> src/libcore/cmp.rs:659:5
659 |     }
    |     ^


error: internal compiler error: broken MIR in DefId(0:1662 ~ core[a39f]::cmp[0]::Ord[0]::clamp[0]) (end of phase Optimized) at bb5[1]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/cmp.rs:659:5: 659:6, scope: scope[0] }, kind: switchInt(_17) -> [false: bb3, otherwise: bb20] }), is_cleanup: true }
   --> src/libcore/cmp.rs:659:5
659 |     }
    |     ^


error: internal compiler error: broken MIR in DefId(0:1662 ~ core[a39f]::cmp[0]::Ord[0]::clamp[0]) (end of phase Optimized) at bb19[1]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/cmp.rs:647:5: 659:6, scope: scope[0] }, kind: resume }), is_cleanup: true }
   --> src/libcore/cmp.rs:659:5
659 |     }
    |     ^


error: internal compiler error: broken MIR in DefId(0:1662 ~ core[a39f]::cmp[0]::Ord[0]::clamp[0]) (end of phase Optimized) at bb20[1]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/cmp.rs:659:5: 659:6, scope: scope[0] }, kind: switchInt(_16) -> [false: bb1, otherwise: bb19] }), is_cleanup: true }
   --> src/libcore/cmp.rs:659:5
659 |     }
    |     ^


error: internal compiler error: broken MIR in DefId(0:1679 ~ core[a39f]::cmp[0]::min_by[0]) (end of phase Optimized) at bb3[0]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/cmp.rs:949:1: 954:2, scope: scope[0] }, kind: resume }), is_cleanup: true }
   --> src/libcore/cmp.rs:954:1
954 | }
    | ^


error: internal compiler error: broken MIR in DefId(0:1679 ~ core[a39f]::cmp[0]::min_by[0]) (end of phase Optimized) at bb3[0]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [_12 = const false], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/cmp.rs:954:1: 954:2, scope: scope[0] }, kind: drop(_1) -> bb1 }), is_cleanup: true }
   --> src/libcore/cmp.rs:954:1
954 | }
    | ^


error: internal compiler error: broken MIR in DefId(0:1679 ~ core[a39f]::cmp[0]::min_by[0]) (end of phase Optimized) at bb4[1]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/cmp.rs:954:1: 954:2, scope: scope[0] }, kind: switchInt(_12) -> [false: bb1, otherwise: bb11] }), is_cleanup: true }
   --> src/libcore/cmp.rs:954:1
954 | }
    | ^


error: internal compiler error: broken MIR in DefId(0:1679 ~ core[a39f]::cmp[0]::min_by[0]) (end of phase Optimized) at bb11[1]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/cmp.rs:949:1: 954:2, scope: scope[0] }, kind: resume }), is_cleanup: true }
   --> src/libcore/cmp.rs:954:1
954 | }
    | ^


error: internal compiler error: broken MIR in DefId(0:1686 ~ core[a39f]::cmp[0]::min_by_key[0]::{{closure}}[0]) (end of phase Optimized) at bb4[0]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/cmp.rs:974:20: 974:46, scope: scope[0] }, kind: resume }), is_cleanup: true }
   --> src/libcore/cmp.rs:974:45
    |
974 |     min_by(v1, v2, |v1, v2| f(v1).cmp(&f(v2)))


error: internal compiler error: broken MIR in DefId(0:1686 ~ core[a39f]::cmp[0]::min_by_key[0]::{{closure}}[0]) (end of phase Optimized) at bb6[0]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/cmp.rs:974:45: 974:46, scope: scope[0] }, kind: drop(_5) -> bb1 }), is_cleanup: true }
   --> src/libcore/cmp.rs:974:45
    |
974 |     min_by(v1, v2, |v1, v2| f(v1).cmp(&f(v2)))


error: internal compiler error: broken MIR in DefId(0:1682 ~ core[a39f]::cmp[0]::min_by_key[0]) (end of phase Optimized) at bb3[0]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/cmp.rs:973:1: 975:2, scope: scope[0] }, kind: resume }), is_cleanup: true }
   --> src/libcore/cmp.rs:975:1
975 | }
    | ^


error: internal compiler error: broken MIR in DefId(0:1689 ~ core[a39f]::cmp[0]::max_by[0]) (end of phase Optimized) at bb3[0]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/cmp.rs:1015:1: 1020:2, scope: scope[0] }, kind: resume }), is_cleanup: true }
    --> src/libcore/cmp.rs:1020:1
1020 | }
     | ^


error: internal compiler error: broken MIR in DefId(0:1689 ~ core[a39f]::cmp[0]::max_by[0]) (end of phase Optimized) at bb3[0]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [_12 = const false], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/cmp.rs:1020:1: 1020:2, scope: scope[0] }, kind: drop(_1) -> bb1 }), is_cleanup: true }
    --> src/libcore/cmp.rs:1020:1
1020 | }
     | ^


error: internal compiler error: broken MIR in DefId(0:1689 ~ core[a39f]::cmp[0]::max_by[0]) (end of phase Optimized) at bb4[1]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/cmp.rs:1020:1: 1020:2, scope: scope[0] }, kind: switchInt(_12) -> [false: bb1, otherwise: bb11] }), is_cleanup: true }
    --> src/libcore/cmp.rs:1020:1
1020 | }
     | ^


error: internal compiler error: broken MIR in DefId(0:1689 ~ core[a39f]::cmp[0]::max_by[0]) (end of phase Optimized) at bb11[1]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/cmp.rs:1015:1: 1020:2, scope: scope[0] }, kind: resume }), is_cleanup: true }
    --> src/libcore/cmp.rs:1020:1
1020 | }
     | ^


error: internal compiler error: broken MIR in DefId(0:1696 ~ core[a39f]::cmp[0]::max_by_key[0]::{{closure}}[0]) (end of phase Optimized) at bb4[0]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/cmp.rs:1040:20: 1040:46, scope: scope[0] }, kind: resume }), is_cleanup: true }
    --> src/libcore/cmp.rs:1040:45
     |
1040 |     max_by(v1, v2, |v1, v2| f(v1).cmp(&f(v2)))


error: internal compiler error: broken MIR in DefId(0:1696 ~ core[a39f]::cmp[0]::max_by_key[0]::{{closure}}[0]) (end of phase Optimized) at bb6[0]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/cmp.rs:1040:45: 1040:46, scope: scope[0] }, kind: drop(_5) -> bb1 }), is_cleanup: true }
    --> src/libcore/cmp.rs:1040:45
     |
1040 |     max_by(v1, v2, |v1, v2| f(v1).cmp(&f(v2)))


error: internal compiler error: broken MIR in DefId(0:1692 ~ core[a39f]::cmp[0]::max_by_key[0]) (end of phase Optimized) at bb3[0]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/cmp.rs:1039:1: 1041:2, scope: scope[0] }, kind: resume }), is_cleanup: true }
    --> src/libcore/cmp.rs:1041:1
1041 | }
     | ^


error: internal compiler error: broken MIR in DefId(0:2147 ~ core[a39f]::ops[0]::generator[0]::{{impl}}[0]::resume[0]) (end of phase Optimized) at bb5[1]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/ops/generator.rs:121:5: 123:6, scope: scope[0] }, kind: resume }), is_cleanup: true }
   --> src/libcore/ops/generator.rs:123:5
123 |     }
    |     ^


error: internal compiler error: broken MIR in DefId(0:2147 ~ core[a39f]::ops[0]::generator[0]::{{impl}}[0]::resume[0]) (end of phase Optimized) at bb6[0]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/ops/generator.rs:121:5: 123:6, scope: scope[0] }, kind: resume }), is_cleanup: true }
   --> src/libcore/ops/generator.rs:123:5
123 |     }
    |     ^


error: internal compiler error: broken MIR in DefId(0:2147 ~ core[a39f]::ops[0]::generator[0]::{{impl}}[0]::resume[0]) (end of phase Optimized) at bb6[0]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [_8 = const false], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/ops/generator.rs:123:5: 123:6, scope: scope[0] }, kind: drop(_2) -> bb1 }), is_cleanup: true }
   --> src/libcore/ops/generator.rs:123:5
123 |     }
    |     ^


error: internal compiler error: broken MIR in DefId(0:2153 ~ core[a39f]::ops[0]::generator[0]::{{impl}}[1]::resume[0]) (end of phase Optimized) at bb5[1]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/ops/generator.rs:131:5: 133:6, scope: scope[0] }, kind: resume }), is_cleanup: true }
   --> src/libcore/ops/generator.rs:133:5
133 |     }
    |     ^


error: internal compiler error: broken MIR in DefId(0:2153 ~ core[a39f]::ops[0]::generator[0]::{{impl}}[1]::resume[0]) (end of phase Optimized) at bb6[0]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/ops/generator.rs:131:5: 133:6, scope: scope[0] }, kind: resume }), is_cleanup: true }
   --> src/libcore/ops/generator.rs:133:5
133 |     }
    |     ^


error: internal compiler error: broken MIR in DefId(0:2153 ~ core[a39f]::ops[0]::generator[0]::{{impl}}[1]::resume[0]) (end of phase Optimized) at bb6[0]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [_9 = const false], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/ops/generator.rs:133:5: 133:6, scope: scope[0] }, kind: drop(_2) -> bb1 }), is_cleanup: true }
   --> src/libcore/ops/generator.rs:133:5
133 |     }
    |     ^


error: internal compiler error: broken MIR in DefId(0:2474 ~ core[a39f]::array[0]::iter[0]::{{impl}}[0]::new[0]) (end of phase Optimized) at bb4[0]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/array/iter.rs:78:5: 78:6, scope: scope[0] }, kind: switchInt(_11) -> [false: bb1, otherwise: bb5] }), is_cleanup: true }
  --> src/libcore/array/iter.rs:75:9
75 |         };
   |         ^


error: internal compiler error: broken MIR in DefId(0:2474 ~ core[a39f]::array[0]::iter[0]::{{impl}}[0]::new[0]) (end of phase Optimized) at bb5[1]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/array/iter.rs:52:5: 78:6, scope: scope[0] }, kind: resume }), is_cleanup: true }
  --> src/libcore/array/iter.rs:78:5
78 |     }
   |     ^


error: internal compiler error: broken MIR in DefId(0:2474 ~ core[a39f]::array[0]::iter[0]::{{impl}}[0]::new[0]) (end of phase Optimized) at bb6[0]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/array/iter.rs:52:5: 78:6, scope: scope[0] }, kind: resume }), is_cleanup: true }
  --> src/libcore/array/iter.rs:78:5
78 |     }
   |     ^


error: internal compiler error: broken MIR in DefId(0:2474 ~ core[a39f]::array[0]::iter[0]::{{impl}}[0]::new[0]) (end of phase Optimized) at bb6[0]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [_11 = const false], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/array/iter.rs:78:5: 78:6, scope: scope[0] }, kind: drop(_1) -> bb1 }), is_cleanup: true }
  --> src/libcore/array/iter.rs:78:5
78 |     }
   |     ^


error: internal compiler error: broken MIR in DefId(0:2487 ~ core[a39f]::array[0]::iter[0]::{{impl}}[1]::count[0]) (end of phase Optimized) at bb3[0]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/array/iter.rs:139:5: 141:6, scope: scope[0] }, kind: resume }), is_cleanup: true }
   --> src/libcore/array/iter.rs:141:5
141 |     }
    |     ^


error: internal compiler error: broken MIR in DefId(0:2488 ~ core[a39f]::array[0]::iter[0]::{{impl}}[1]::last[0]) (end of phase Optimized) at bb3[0]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/array/iter.rs:143:5: 145:6, scope: scope[0] }, kind: resume }), is_cleanup: true }
   --> src/libcore/array/iter.rs:145:5
145 |     }
    |     ^


error: internal compiler error: broken MIR in DefId(0:2517 ~ core[a39f]::array[0]::iter[0]::{{impl}}[7]::clone[0]) (end of phase Optimized) at bb17[0]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/array/iter.rs:247:9: 247:10, scope: scope[1] }, kind: switchInt(_34) -> [false: bb1, otherwise: bb18] }), is_cleanup: true }
   --> src/libcore/array/iter.rs:246:62
    |
246 |             Self { data: new_data, alive: self.alive.clone() }


error: internal compiler error: broken MIR in DefId(0:2517 ~ core[a39f]::array[0]::iter[0]::{{impl}}[7]::clone[0]) (end of phase Optimized) at bb18[1]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/array/iter.rs:227:5: 248:6, scope: scope[0] }, kind: resume }), is_cleanup: true }
   --> src/libcore/array/iter.rs:247:9
247 |         }
    |         ^


error: internal compiler error: broken MIR in DefId(0:2517 ~ core[a39f]::array[0]::iter[0]::{{impl}}[7]::clone[0]) (end of phase Optimized) at bb19[0]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/array/iter.rs:227:5: 248:6, scope: scope[0] }, kind: resume }), is_cleanup: true }
   --> src/libcore/array/iter.rs:247:9
247 |         }
    |         ^


error: internal compiler error: broken MIR in DefId(0:2517 ~ core[a39f]::array[0]::iter[0]::{{impl}}[7]::clone[0]) (end of phase Optimized) at bb19[0]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [_34 = const false], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/array/iter.rs:247:9: 247:10, scope: scope[1] }, kind: drop(_2) -> bb1 }), is_cleanup: true }
   --> src/libcore/array/iter.rs:247:9
247 |         }
    |         ^


error: internal compiler error: broken MIR in DefId(0:2517 ~ core[a39f]::array[0]::iter[0]::{{impl}}[7]::clone[0]) (end of phase Optimized) at bb20[1]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/array/iter.rs:247:9: 247:10, scope: scope[1] }, kind: switchInt(_34) -> [false: bb1, otherwise: bb18] }), is_cleanup: true }
   --> src/libcore/array/iter.rs:244:13
244 |             }
    |             ^


error: internal compiler error: broken MIR in DefId(0:2517 ~ core[a39f]::array[0]::iter[0]::{{impl}}[7]::clone[0]) (end of phase Optimized) at bb21[0]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/array/iter.rs:247:9: 247:10, scope: scope[1] }, kind: switchInt(_34) -> [false: bb1, otherwise: bb18] }), is_cleanup: true }
   --> src/libcore/array/iter.rs:244:13
244 |             }
    |             ^


error: internal compiler error: broken MIR in DefId(0:2517 ~ core[a39f]::array[0]::iter[0]::{{impl}}[7]::clone[0]) (end of phase Optimized) at bb21[0]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [_35 = const false], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/array/iter.rs:244:13: 244:14, scope: scope[6] }, kind: drop(_16) -> bb19 }), is_cleanup: true }
   --> src/libcore/array/iter.rs:244:13
244 |             }
    |             ^


error: internal compiler error: broken MIR in DefId(0:2795 ~ core[a39f]::cell[0]::{{impl}}[9]::replace[0]) (end of phase Optimized) at bb4[1]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/cell.rs:390:5: 394:6, scope: scope[0] }, kind: resume }), is_cleanup: true }
    |
394 |     }
    |     ^


error: internal compiler error: broken MIR in DefId(0:2795 ~ core[a39f]::cell[0]::{{impl}}[9]::replace[0]) (end of phase Optimized) at bb5[0]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/cell.rs:390:5: 394:6, scope: scope[0] }, kind: resume }), is_cleanup: true }
    |
394 |     }
    |     ^


error: internal compiler error: broken MIR in DefId(0:2795 ~ core[a39f]::cell[0]::{{impl}}[9]::replace[0]) (end of phase Optimized) at bb5[0]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [_9 = const false], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/cell.rs:394:5: 394:6, scope: scope[0] }, kind: drop(_2) -> bb1 }), is_cleanup: true }
    |
394 |     }
    |     ^


error: internal compiler error: broken MIR in DefId(0:2800 ~ core[a39f]::cell[0]::{{impl}}[10]::update[0]) (end of phase Optimized) at bb5[1]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/cell.rs:451:5: 459:6, scope: scope[0] }, kind: resume }), is_cleanup: true }
    |
459 |     }
    |     ^


error: internal compiler error: broken MIR in DefId(0:2800 ~ core[a39f]::cell[0]::{{impl}}[10]::update[0]) (end of phase Optimized) at bb6[0]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/cell.rs:451:5: 459:6, scope: scope[0] }, kind: resume }), is_cleanup: true }
    |
459 |     }
    |     ^


error: internal compiler error: broken MIR in DefId(0:2800 ~ core[a39f]::cell[0]::{{impl}}[10]::update[0]) (end of phase Optimized) at bb6[0]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [_11 = const false], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/cell.rs:459:5: 459:6, scope: scope[0] }, kind: drop(_2) -> bb1 }), is_cleanup: true }
    |
459 |     }
    |     ^


error: internal compiler error: broken MIR in DefId(0:2838 ~ core[a39f]::cell[0]::{{impl}}[19]::new[0]) (end of phase Optimized) at bb3[0]:
encountered jump that does not respect unwind invariants BasicBlockData { statements: [], terminator: Some(Terminator { source_info: SourceInfo { span: src/libcore/cell.rs:658:5: 660:6, scope: scope[0] }, kind: resume }), is_cleanup: true }
    |
---
  local time: Mon Jun  8 13:26:08 UTC 2020
  network time: Mon, 08 Jun 2020 13:26:08 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/73133/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/73133/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3809) (python)
##[section]Finishing: Finalize Job
