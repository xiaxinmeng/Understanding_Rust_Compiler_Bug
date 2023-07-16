plain
   Compiling std v0.0.0 (/checkout/library/std)
error: formatting may not be suitable for sub-register argument
     --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:30336:34
      |
30336 |          "vmovdqu32 {2}{{{1}}}, [{0}]",
      |                                  ^^^
30337 |          in(reg) mem_addr,
      |                  -------- for this argument
      |
      = note: `-D asm-sub-register` implied by `-D warnings`
      = help: use the `e` modifier to have the register formatted as `eax`
      = help: or use the `r` modifier to keep the default formatting of `rax`
error: formatting may not be suitable for sub-register argument
     --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:30355:40
      |
      |
30355 |          "vmovdqu32 {2}{{{1}}} {{z}}, [{0}]",
      |                                        ^^^
30356 |          in(reg) mem_addr,
      |                  -------- for this argument
      |
      = help: use the `e` modifier to have the register formatted as `eax`
      = help: or use the `r` modifier to keep the default formatting of `rax`
error: formatting may not be suitable for sub-register argument
     --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:30374:34
      |
      |
30374 |          "vmovdqu64 {2}{{{1}}}, [{0}]",
      |                                  ^^^
30375 |          in(reg) mem_addr,
      |                  -------- for this argument
      |
      = help: use the `e` modifier to have the register formatted as `eax`
      = help: or use the `r` modifier to keep the default formatting of `rax`
error: formatting may not be suitable for sub-register argument
     --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:30393:40
      |
      |
30393 |          "vmovdqu64 {2}{{{1}}} {{z}}, [{0}]",
      |                                        ^^^
30394 |          in(reg) mem_addr,
      |                  -------- for this argument
      |
      = help: use the `e` modifier to have the register formatted as `eax`
      = help: or use the `r` modifier to keep the default formatting of `rax`
error: formatting may not be suitable for sub-register argument
     --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:30412:32
      |
      |
30412 |          "vmovups {2}{{{1}}}, [{0}]",
      |                                ^^^
30413 |          in(reg) mem_addr,
      |                  -------- for this argument
      |
      = help: use the `e` modifier to have the register formatted as `eax`
      = help: or use the `r` modifier to keep the default formatting of `rax`
error: formatting may not be suitable for sub-register argument
     --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:30431:38
      |
      |
30431 |          "vmovups {2}{{{1}}} {{z}}, [{0}]",
      |                                      ^^^
30432 |          in(reg) mem_addr,
      |                  -------- for this argument
      |
      = help: use the `e` modifier to have the register formatted as `eax`
      = help: or use the `r` modifier to keep the default formatting of `rax`
error: formatting may not be suitable for sub-register argument
     --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:30450:32
      |
      |
30450 |          "vmovupd {2}{{{1}}}, [{0}]",
      |                                ^^^
30451 |          in(reg) mem_addr,
      |                  -------- for this argument
      |
      = help: use the `e` modifier to have the register formatted as `eax`
      = help: or use the `r` modifier to keep the default formatting of `rax`
error: formatting may not be suitable for sub-register argument
     --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:30469:38
      |
      |
30469 |          "vmovupd {2}{{{1}}} {{z}}, [{0}]",
      |                                      ^^^
30470 |          in(reg) mem_addr,
      |                  -------- for this argument
      |
      = help: use the `e` modifier to have the register formatted as `eax`
      = help: or use the `r` modifier to keep the default formatting of `rax`
error: formatting may not be suitable for sub-register argument
     --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:30488:34
      |
      |
30488 |          "vmovdqu32 {2}{{{1}}}, [{0}]",
      |                                  ^^^
30489 |          in(reg) mem_addr,
      |                  -------- for this argument
      |
      = help: use the `e` modifier to have the register formatted as `eax`
      = help: or use the `r` modifier to keep the default formatting of `rax`
error: formatting may not be suitable for sub-register argument
     --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:30507:40
      |
      |
30507 |          "vmovdqu32 {2}{{{1}}} {{z}}, [{0}]",
      |                                        ^^^
30508 |          in(reg) mem_addr,
      |                  -------- for this argument
      |
      = help: use the `e` modifier to have the register formatted as `eax`
      = help: or use the `r` modifier to keep the default formatting of `rax`
error: formatting may not be suitable for sub-register argument
     --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:30526:34
      |
      |
30526 |          "vmovdqu64 {2}{{{1}}}, [{0}]",
      |                                  ^^^
30527 |          in(reg) mem_addr,
      |                  -------- for this argument
      |
      = help: use the `e` modifier to have the register formatted as `eax`
      = help: or use the `r` modifier to keep the default formatting of `rax`
error: formatting may not be suitable for sub-register argument
     --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:30545:40
      |
      |
30545 |          "vmovdqu64 {2}{{{1}}} {{z}}, [{0}]",
      |                                        ^^^
30546 |          in(reg) mem_addr,
      |                  -------- for this argument
      |
      = help: use the `e` modifier to have the register formatted as `eax`
      = help: or use the `r` modifier to keep the default formatting of `rax`
error: formatting may not be suitable for sub-register argument
     --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:30564:32
      |
      |
30564 |          "vmovups {2}{{{1}}}, [{0}]",
      |                                ^^^
30565 |          in(reg) mem_addr,
      |                  -------- for this argument
      |
      = help: use the `e` modifier to have the register formatted as `eax`
      = help: or use the `r` modifier to keep the default formatting of `rax`
error: formatting may not be suitable for sub-register argument
     --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:30583:38
      |
      |
30583 |          "vmovups {2}{{{1}}} {{z}}, [{0}]",
      |                                      ^^^
30584 |          in(reg) mem_addr,
      |                  -------- for this argument
      |
      = help: use the `e` modifier to have the register formatted as `eax`
      = help: or use the `r` modifier to keep the default formatting of `rax`
error: formatting may not be suitable for sub-register argument
     --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:30602:32
      |
      |
30602 |          "vmovupd {2}{{{1}}}, [{0}]",
      |                                ^^^
30603 |          in(reg) mem_addr,
      |                  -------- for this argument
      |
      = help: use the `e` modifier to have the register formatted as `eax`
      = help: or use the `r` modifier to keep the default formatting of `rax`
error: formatting may not be suitable for sub-register argument
     --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:30621:38
      |
      |
30621 |          "vmovupd {2}{{{1}}} {{z}}, [{0}]",
      |                                      ^^^
30622 |          in(reg) mem_addr,
      |                  -------- for this argument
      |
      = help: use the `e` modifier to have the register formatted as `eax`
      = help: or use the `r` modifier to keep the default formatting of `rax`
error: formatting may not be suitable for sub-register argument
     --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:30640:34
      |
      |
30640 |          "vmovdqu32 {2}{{{1}}}, [{0}]",
      |                                  ^^^
30641 |          in(reg) mem_addr,
      |                  -------- for this argument
      |
      = help: use the `e` modifier to have the register formatted as `eax`
      = help: or use the `r` modifier to keep the default formatting of `rax`
error: formatting may not be suitable for sub-register argument
     --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:30659:40
      |
      |
30659 |          "vmovdqu32 {2}{{{1}}} {{z}}, [{0}]",
      |                                        ^^^
30660 |          in(reg) mem_addr,
      |                  -------- for this argument
      |
      = help: use the `e` modifier to have the register formatted as `eax`
      = help: or use the `r` modifier to keep the default formatting of `rax`
error: formatting may not be suitable for sub-register argument
     --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:30678:34
      |
      |
30678 |          "vmovdqu64 {2}{{{1}}}, [{0}]",
      |                                  ^^^
30679 |          in(reg) mem_addr,
      |                  -------- for this argument
      |
      = help: use the `e` modifier to have the register formatted as `eax`
      = help: or use the `r` modifier to keep the default formatting of `rax`
error: formatting may not be suitable for sub-register argument
     --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:30697:40
      |
      |
30697 |          "vmovdqu64 {2}{{{1}}} {{z}}, [{0}]",
      |                                        ^^^
30698 |          in(reg) mem_addr,
      |                  -------- for this argument
      |
      = help: use the `e` modifier to have the register formatted as `eax`
      = help: or use the `r` modifier to keep the default formatting of `rax`
error: formatting may not be suitable for sub-register argument
     --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:30716:32
      |
      |
30716 |          "vmovups {2}{{{1}}}, [{0}]",
      |                                ^^^
30717 |          in(reg) mem_addr,
      |                  -------- for this argument
      |
      = help: use the `e` modifier to have the register formatted as `eax`
      = help: or use the `r` modifier to keep the default formatting of `rax`
error: formatting may not be suitable for sub-register argument
     --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:30735:38
      |
      |
30735 |          "vmovups {2}{{{1}}} {{z}}, [{0}]",
      |                                      ^^^
30736 |          in(reg) mem_addr,
      |                  -------- for this argument
      |
      = help: use the `e` modifier to have the register formatted as `eax`
      = help: or use the `r` modifier to keep the default formatting of `rax`
error: formatting may not be suitable for sub-register argument
     --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:30754:32
      |
      |
30754 |          "vmovupd {2}{{{1}}}, [{0}]",
      |                                ^^^
30755 |          in(reg) mem_addr,
      |                  -------- for this argument
      |
      = help: use the `e` modifier to have the register formatted as `eax`
      = help: or use the `r` modifier to keep the default formatting of `rax`
error: formatting may not be suitable for sub-register argument
     --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:30773:38
      |
      |
30773 |          "vmovupd {2}{{{1}}} {{z}}, [{0}]",
      |                                      ^^^
30774 |          in(reg) mem_addr,
      |                  -------- for this argument
      |
      = help: use the `e` modifier to have the register formatted as `eax`
      = help: or use the `r` modifier to keep the default formatting of `rax`
error: formatting may not be suitable for sub-register argument
     --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:30792:34
      |
      |
30792 |          "vmovdqa32 {2}{{{1}}}, [{0}]",
      |                                  ^^^
30793 |          in(reg) mem_addr,
      |                  -------- for this argument
      |
      = help: use the `e` modifier to have the register formatted as `eax`
      = help: or use the `r` modifier to keep the default formatting of `rax`
error: formatting may not be suitable for sub-register argument
     --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:30811:40
      |
      |
30811 |          "vmovdqa32 {2}{{{1}}} {{z}}, [{0}]",
      |                                        ^^^
30812 |          in(reg) mem_addr,
      |                  -------- for this argument
      |
      = help: use the `e` modifier to have the register formatted as `eax`
      = help: or use the `r` modifier to keep the default formatting of `rax`
error: formatting may not be suitable for sub-register argument
     --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:30830:34
      |
      |
30830 |          "vmovdqa64 {2}{{{1}}}, [{0}]",
      |                                  ^^^
30831 |          in(reg) mem_addr,
      |                  -------- for this argument
      |
      = help: use the `e` modifier to have the register formatted as `eax`
      = help: or use the `r` modifier to keep the default formatting of `rax`
error: formatting may not be suitable for sub-register argument
     --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:30849:40
      |
      |
30849 |          "vmovdqa64 {2}{{{1}}} {{z}}, [{0}]",
      |                                        ^^^
30850 |          in(reg) mem_addr,
      |                  -------- for this argument
      |
      = help: use the `e` modifier to have the register formatted as `eax`
      = help: or use the `r` modifier to keep the default formatting of `rax`
error: formatting may not be suitable for sub-register argument
     --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:30868:32
      |
      |
30868 |          "vmovaps {2}{{{1}}}, [{0}]",
      |                                ^^^
30869 |          in(reg) mem_addr,
      |                  -------- for this argument
      |
      = help: use the `e` modifier to have the register formatted as `eax`
      = help: or use the `r` modifier to keep the default formatting of `rax`
error: formatting may not be suitable for sub-register argument
     --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:30887:38
      |
      |
30887 |          "vmovaps {2}{{{1}}} {{z}}, [{0}]",
      |                                      ^^^
30888 |          in(reg) mem_addr,
      |                  -------- for this argument
      |
      = help: use the `e` modifier to have the register formatted as `eax`
      = help: or use the `r` modifier to keep the default formatting of `rax`
error: formatting may not be suitable for sub-register argument
     --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:30906:32
      |
      |
30906 |          "vmovapd {2}{{{1}}}, [{0}]",
      |                                ^^^
30907 |          in(reg) mem_addr,
      |                  -------- for this argument
      |
      = help: use the `e` modifier to have the register formatted as `eax`
      = help: or use the `r` modifier to keep the default formatting of `rax`
error: formatting may not be suitable for sub-register argument
     --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:30925:38
      |
      |
30925 |          "vmovapd {2}{{{1}}} {{z}}, [{0}]",
      |                                      ^^^
30926 |          in(reg) mem_addr,
      |                  -------- for this argument
      |
      = help: use the `e` modifier to have the register formatted as `eax`
      = help: or use the `r` modifier to keep the default formatting of `rax`
error: formatting may not be suitable for sub-register argument
     --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:30944:34
      |
      |
30944 |          "vmovdqa32 {2}{{{1}}}, [{0}]",
      |                                  ^^^
30945 |          in(reg) mem_addr,
      |                  -------- for this argument
      |
      = help: use the `e` modifier to have the register formatted as `eax`
      = help: or use the `r` modifier to keep the default formatting of `rax`
error: formatting may not be suitable for sub-register argument
     --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:30963:40
      |
      |
30963 |          "vmovdqa32 {2}{{{1}}} {{z}}, [{0}]",
      |                                        ^^^
30964 |          in(reg) mem_addr,
      |                  -------- for this argument
      |
      = help: use the `e` modifier to have the register formatted as `eax`
      = help: or use the `r` modifier to keep the default formatting of `rax`
error: formatting may not be suitable for sub-register argument
     --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:30982:34
      |
      |
30982 |          "vmovdqa64 {2}{{{1}}}, [{0}]",
      |                                  ^^^
30983 |          in(reg) mem_addr,
      |                  -------- for this argument
      |
      = help: use the `e` modifier to have the register formatted as `eax`
      = help: or use the `r` modifier to keep the default formatting of `rax`
error: formatting may not be suitable for sub-register argument
     --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:31001:40
      |
      |
31001 |          "vmovdqa64 {2}{{{1}}} {{z}}, [{0}]",
      |                                        ^^^
31002 |          in(reg) mem_addr,
      |                  -------- for this argument
      |
      = help: use the `e` modifier to have the register formatted as `eax`
      = help: or use the `r` modifier to keep the default formatting of `rax`
error: formatting may not be suitable for sub-register argument
     --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:31020:32
      |
      |
31020 |          "vmovaps {2}{{{1}}}, [{0}]",
      |                                ^^^
31021 |          in(reg) mem_addr,
      |                  -------- for this argument
      |
      = help: use the `e` modifier to have the register formatted as `eax`
      = help: or use the `r` modifier to keep the default formatting of `rax`
error: formatting may not be suitable for sub-register argument
     --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:31039:38
      |
      |
31039 |          "vmovaps {2}{{{1}}} {{z}}, [{0}]",
      |                                      ^^^
31040 |          in(reg) mem_addr,
      |                  -------- for this argument
      |
      = help: use the `e` modifier to have the register formatted as `eax`
      = help: or use the `r` modifier to keep the default formatting of `rax`
error: formatting may not be suitable for sub-register argument
     --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:31058:32
      |
      |
31058 |          "vmovapd {2}{{{1}}}, [{0}]",
      |                                ^^^
31059 |          in(reg) mem_addr,
      |                  -------- for this argument
      |
      = help: use the `e` modifier to have the register formatted as `eax`
      = help: or use the `r` modifier to keep the default formatting of `rax`
error: formatting may not be suitable for sub-register argument
     --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:31077:38
      |
      |
31077 |          "vmovapd {2}{{{1}}} {{z}}, [{0}]",
      |                                      ^^^
31078 |          in(reg) mem_addr,
      |                  -------- for this argument
      |
      = help: use the `e` modifier to have the register formatted as `eax`
      = help: or use the `r` modifier to keep the default formatting of `rax`
error: formatting may not be suitable for sub-register argument
     --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:31096:34
      |
      |
31096 |          "vmovdqa32 {2}{{{1}}}, [{0}]",
      |                                  ^^^
31097 |          in(reg) mem_addr,
      |                  -------- for this argument
      |
      = help: use the `e` modifier to have the register formatted as `eax`
      = help: or use the `r` modifier to keep the default formatting of `rax`
error: formatting may not be suitable for sub-register argument
     --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:31115:40
      |
      |
31115 |          "vmovdqa32 {2}{{{1}}} {{z}}, [{0}]",
      |                                        ^^^
31116 |          in(reg) mem_addr,
      |                  -------- for this argument
      |
      = help: use the `e` modifier to have the register formatted as `eax`
      = help: or use the `r` modifier to keep the default formatting of `rax`
error: formatting may not be suitable for sub-register argument
     --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:31134:34
      |
      |
31134 |          "vmovdqa64 {2}{{{1}}}, [{0}]",
      |                                  ^^^
31135 |          in(reg) mem_addr,
      |                  -------- for this argument
      |
      = help: use the `e` modifier to have the register formatted as `eax`
      = help: or use the `r` modifier to keep the default formatting of `rax`
error: formatting may not be suitable for sub-register argument
     --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:31153:40
      |
      |
31153 |          "vmovdqa64 {2}{{{1}}} {{z}}, [{0}]",
      |                                        ^^^
31154 |          in(reg) mem_addr,
      |                  -------- for this argument
      |
      = help: use the `e` modifier to have the register formatted as `eax`
      = help: or use the `r` modifier to keep the default formatting of `rax`
error: formatting may not be suitable for sub-register argument
     --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:31172:32
      |
      |
31172 |          "vmovaps {2}{{{1}}}, [{0}]",
      |                                ^^^
31173 |          in(reg) mem_addr,
      |                  -------- for this argument
      |
      = help: use the `e` modifier to have the register formatted as `eax`
      = help: or use the `r` modifier to keep the default formatting of `rax`
error: formatting may not be suitable for sub-register argument
     --> library/core/src/../../stdarch/crates/core_arch/src/x86/avx512f.rs:31191:38
      |
      |
