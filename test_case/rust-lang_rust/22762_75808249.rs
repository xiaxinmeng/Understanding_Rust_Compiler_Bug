
~/dragons$ cargo new hurrdurr
~/dragons$ cd hurrdurr 
~/dragons/hurrdurr$ valgrind cargo build
==8063== Memcheck, a memory error detector
==8063== Copyright (C) 2002-2013, and GNU GPL'd, by Julian Seward et al.
==8063== Using Valgrind-3.10.1 and LibVEX; rerun with -h for copyright info
==8063== Command: cargo build
==8063== 
==8063== Conditional jump or move depends on uninitialised value(s)
==8063==    at 0x3BBDBC: synonym::SynonymMap$LT$K$C$$u20$V$GT$::insert::h3724314155202151280 (in /usr/local/bin/cargo)
==8063==    by 0x3C1FAE: parse::PatParser$LT$$u27$a$GT$::add_atom_ifnotexists::h5c3eddacff495282Glc (in /usr/local/bin/cargo)
==8063==    by 0x3BD0D4: parse::PatParser$LT$$u27$a$GT$::pattern::hf8a955ff6e4a5adet2b (in /usr/local/bin/cargo)
==8063==    by 0x3C1ACE: parse::PatParser$LT$$u27$a$GT$::group::he9c6ce2fbc0887eaimc (in /usr/local/bin/cargo)
==8063==    by 0x3BE266: parse::PatParser$LT$$u27$a$GT$::pattern::hf8a955ff6e4a5adet2b (in /usr/local/bin/cargo)
==8063==    by 0x3B83BC: parse::PatParser$LT$$u27$a$GT$::parse::h90fccbc0b26f2df600b (in /usr/local/bin/cargo)
==8063==    by 0x3B2E7B: parse::Parser::new::h93460f743e7f69d7wsb (in /usr/local/bin/cargo)
==8063==    by 0x1729AC: Docopt::new::h4465346578811968731 (in /usr/local/bin/cargo)
==8063==    by 0x170C42: call_main_without_stdin::h6322948571204322835 (in /usr/local/bin/cargo)
==8063==    by 0x16F80C: main::h14917aa7cb75c12bwca (in /usr/local/bin/cargo)
==8063==    by 0x6A0C18: rust_try_inner (in /usr/local/bin/cargo)
==8063==    by 0x6A0C05: rust_try (in /usr/local/bin/cargo)
==8063== 
==8063== Conditional jump or move depends on uninitialised value(s)
==8063==    at 0x23B64C: ops::cargo_compile::compile_pkg::hbb6f4c5ae72d634apNh (in /usr/local/bin/cargo)
==8063==    by 0x23A827: ops::cargo_compile::compile::h459aef7669fb9faapIh (in /usr/local/bin/cargo)
==8063==    by 0x184512: call_main_without_stdin::h16575523244349200905 (in /usr/local/bin/cargo)
==8063==    by 0x175E15: execute::hecf542bd15794221Ica (in /usr/local/bin/cargo)
==8063==    by 0x171E08: call_main_without_stdin::h6322948571204322835 (in /usr/local/bin/cargo)
==8063==    by 0x16F80C: main::h14917aa7cb75c12bwca (in /usr/local/bin/cargo)
==8063==    by 0x6A0C18: rust_try_inner (in /usr/local/bin/cargo)
==8063==    by 0x6A0C05: rust_try (in /usr/local/bin/cargo)
==8063==    by 0x69DF0C: rt::lang_start::h61cf7984fd279964KzJ (in /usr/local/bin/cargo)
==8063==    by 0x54787FF: (below main) (in /usr/lib/libc-2.21.so)
==8063== 
==8063== Conditional jump or move depends on uninitialised value(s)
==8063==    at 0x2C85ED: sync::mpsc::Receiver$LT$T$GT$::recv::h13638071427880253445 (in /usr/local/bin/cargo)
==8063==    by 0x2467D9: ops::cargo_rustc::compile_targets::haf1e4fe4221dba30P9m (in /usr/local/bin/cargo)
==8063==    by 0x23F0E6: ops::cargo_compile::compile_pkg::hbb6f4c5ae72d634apNh (in /usr/local/bin/cargo)
==8063==    by 0x23A827: ops::cargo_compile::compile::h459aef7669fb9faapIh (in /usr/local/bin/cargo)
==8063==    by 0x184512: call_main_without_stdin::h16575523244349200905 (in /usr/local/bin/cargo)
==8063==    by 0x175E15: execute::hecf542bd15794221Ica (in /usr/local/bin/cargo)
==8063==    by 0x171E08: call_main_without_stdin::h6322948571204322835 (in /usr/local/bin/cargo)
==8063==    by 0x16F80C: main::h14917aa7cb75c12bwca (in /usr/local/bin/cargo)
==8063==    by 0x6A0C18: rust_try_inner (in /usr/local/bin/cargo)
==8063==    by 0x6A0C05: rust_try (in /usr/local/bin/cargo)
==8063==    by 0x69DF0C: rt::lang_start::h61cf7984fd279964KzJ (in /usr/local/bin/cargo)
==8063==    by 0x54787FF: (below main) (in /usr/lib/libc-2.21.so)
==8063== 
==8063== Conditional jump or move depends on uninitialised value(s)
==8063==    at 0x2C3233: ops::cargo_rustc::job_queue::JobQueue$LT$$u27$a$C$$u20$$u27$b$GT$::run::ha0306ea69322c899Fxm (in /usr/local/bin/cargo)
==8063==    by 0x2467AE: ops::cargo_rustc::compile_targets::haf1e4fe4221dba30P9m (in /usr/local/bin/cargo)
==8063==    by 0x23F0E6: ops::cargo_compile::compile_pkg::hbb6f4c5ae72d634apNh (in /usr/local/bin/cargo)
==8063==    by 0x23A827: ops::cargo_compile::compile::h459aef7669fb9faapIh (in /usr/local/bin/cargo)
==8063==    by 0x184512: call_main_without_stdin::h16575523244349200905 (in /usr/local/bin/cargo)
==8063==    by 0x175E15: execute::hecf542bd15794221Ica (in /usr/local/bin/cargo)
==8063==    by 0x171E08: call_main_without_stdin::h6322948571204322835 (in /usr/local/bin/cargo)
==8063==    by 0x16F80C: main::h14917aa7cb75c12bwca (in /usr/local/bin/cargo)
==8063==    by 0x6A0C18: rust_try_inner (in /usr/local/bin/cargo)
==8063==    by 0x6A0C05: rust_try (in /usr/local/bin/cargo)
==8063==    by 0x69DF0C: rt::lang_start::h61cf7984fd279964KzJ (in /usr/local/bin/cargo)
==8063==    by 0x54787FF: (below main) (in /usr/lib/libc-2.21.so)
==8063==
