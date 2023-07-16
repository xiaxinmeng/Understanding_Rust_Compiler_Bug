plain
..........................................i......................................................... 6000/11995
.................................................................................................... 6100/11995
................................................i.........FF........................................ 6200/11995
.....................i.............................................................................. 6300/11995
....................F......F.......................................................ii.ii.......i...i 6400/11995
............................i....i..................................i...........................i... 6600/11995
.................................................................................................... 6700/11995
.....................................i.............................................................. 6800/11995
.................................................................................................... 6900/11995
---
.................................................................................................... 8600/11995
.................................................................................................... 8700/11995
.................................................................................................... 8800/11995
.................................................................................................... 8900/11995
..................................................................................................ii 9000/11995
ii.iiiii................................................................ii.......F.......i........F. 9100/11995
......F............................................................................................. 9200/11995
.................................................................................................... 9400/11995
.................................................................................................... 9500/11995
.................................................................................................... 9600/11995
.................................................................................................... 9700/11995
---
failures:

---- [ui] ui/const-generics/defaults/pretty-printing-ast.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/defaults/pretty-printing-ast.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/defaults/pretty-printing-ast" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unpretty=expanded" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/defaults/pretty-printing-ast/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'already borrowed: BorrowMutError', compiler/rustc_interface/src/queries.rs:36:38
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

error: internal compiler error: unexpected panic


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (16635a6c4 2021-06-20) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unpretty=expanded -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack

------------------------------------------
------------------------------------------


---- [ui] ui/hygiene/unpretty-debug.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hygiene/unpretty-debug.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/unpretty-debug" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunpretty=expanded,hygiene" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/unpretty-debug/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'already borrowed: BorrowMutError', compiler/rustc_interface/src/queries.rs:36:38

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (16635a6c4 2021-06-20) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unpretty=expanded,hygiene -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack

------------------------------------------
------------------------------------------


---- [ui] ui/issues/issue-37665.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-37665.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-37665" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unpretty=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-37665/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'already borrowed: BorrowMutError', compiler/rustc_interface/src/queries.rs:36:38

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (16635a6c4 2021-06-20) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unpretty=mir -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack

------------------------------------------
------------------------------------------


---- [ui] ui/issues/issue-60662.rs#full_tait stdout ----

error in revision `full_tait`: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-60662.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full_tait" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-60662.full_tait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unpretty=hir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-60662.full_tait/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'already borrowed: BorrowMutError', compiler/rustc_interface/src/queries.rs:36:38

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (16635a6c4 2021-06-20) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unpretty=hir -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack

------------------------------------------
------------------------------------------


---- [ui] ui/issues/issue-60662.rs#min_tait stdout ----

error in revision `min_tait`: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-60662.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "min_tait" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-60662.min_tait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unpretty=hir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-60662.min_tait/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'already borrowed: BorrowMutError', compiler/rustc_interface/src/queries.rs:36:38

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (16635a6c4 2021-06-20) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unpretty=hir -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack

------------------------------------------
------------------------------------------


---- [ui] ui/issues/issue-81918.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-81918.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-81918" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unpretty=mir-cfg" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-81918/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'already borrowed: BorrowMutError', compiler/rustc_interface/src/queries.rs:36:38

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (16635a6c4 2021-06-20) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unpretty=mir-cfg -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack

------------------------------------------
------------------------------------------


---- [ui] ui/issues/issue-83048.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-83048.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-83048" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unpretty=thir-tree" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-83048/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'already borrowed: BorrowMutError', compiler/rustc_interface/src/queries.rs:36:38

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (16635a6c4 2021-06-20) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unpretty=thir-tree -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack

------------------------------------------
------------------------------------------


---- [ui] ui/match/issue-82392.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/match/issue-82392.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/issue-82392" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunpretty=hir,typed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/issue-82392/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'already borrowed: BorrowMutError', compiler/rustc_interface/src/queries.rs:36:38

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (16635a6c4 2021-06-20) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unpretty=hir,typed -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack

------------------------------------------
------------------------------------------


---- [ui] ui/mir-unpretty.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mir-unpretty.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir-unpretty" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unpretty=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir-unpretty/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'already borrowed: BorrowMutError', compiler/rustc_interface/src/queries.rs:36:38

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (16635a6c4 2021-06-20) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unpretty=mir -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack

------------------------------------------
------------------------------------------


---- [ui] ui/proc-macro/meta-macro-hygiene.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/meta-macro-hygiene.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/meta-macro-hygiene" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-Z" "span-debug" "-Z" "macro-backtrace" "-Z" "unpretty=expanded,hygiene" "-Z" "trim-diagnostic-paths=no" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/meta-macro-hygiene/auxiliary"
------------------------------------------
------------------------------------------
Def site: /checkout/src/test/ui/proc-macro/auxiliary/make-macro.rs:7:9: 7:56 (#5)
Input: TokenStream [Ident { ident: "$crate", span: /checkout/src/test/ui/proc-macro/meta-macro-hygiene.rs:23:37: 23:43 (#4) }, Punct { ch: ':', spacing: Joint, span: /checkout/src/test/ui/proc-macro/meta-macro-hygiene.rs:23:43: 23:45 (#4) }, Punct { ch: ':', spacing: Alone, span: /checkout/src/test/ui/proc-macro/meta-macro-hygiene.rs:23:43: 23:45 (#4) }, Ident { ident: "dummy", span: /checkout/src/test/ui/proc-macro/meta-macro-hygiene.rs:23:45: 23:50 (#4) }, Punct { ch: '!', spacing: Alone, span: /checkout/src/test/ui/proc-macro/meta-macro-hygiene.rs:23:50: 23:51 (#4) }, Group { delimiter: Parenthesis, stream: TokenStream [], span: /checkout/src/test/ui/proc-macro/meta-macro-hygiene.rs:23:51: 23:53 (#4) }]
Respanned: TokenStream [Ident { ident: "$crate", span: /checkout/src/test/ui/proc-macro/auxiliary/make-macro.rs:7:9: 7:56 (#5) }, Punct { ch: ':', spacing: Joint, span: /checkout/src/test/ui/proc-macro/auxiliary/make-macro.rs:7:9: 7:56 (#5) }, Punct { ch: ':', spacing: Alone, span: /checkout/src/test/ui/proc-macro/auxiliary/make-macro.rs:7:9: 7:56 (#5) }, Ident { ident: "dummy", span: /checkout/src/test/ui/proc-macro/auxiliary/make-macro.rs:7:9: 7:56 (#5) }, Punct { ch: '!', spacing: Alone, span: /checkout/src/test/ui/proc-macro/auxiliary/make-macro.rs:7:9: 7:56 (#5) }, Group { delimiter: Parenthesis, stream: TokenStream [], span: /checkout/src/test/ui/proc-macro/auxiliary/make-macro.rs:7:9: 7:56 (#5) }]
------------------------------------------
stderr:
------------------------------------------
------------------------------------------
thread 'rustc' panicked at 'already borrowed: BorrowMutError', compiler/rustc_interface/src/queries.rs:36:38

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (16635a6c4 2021-06-20) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z span-debug -Z macro-backtrace -Z unpretty=expanded,hygiene -Z trim-diagnostic-paths=no -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack

------------------------------------------
------------------------------------------


---- [ui] ui/proc-macro/quote-debug.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/quote-debug.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/quote-debug" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unpretty=expanded" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/quote-debug/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'already borrowed: BorrowMutError', compiler/rustc_interface/src/queries.rs:36:38

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (16635a6c4 2021-06-20) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unpretty=expanded -C codegen-units=1 -C rpath -C debuginfo=0
query stack during panic:
end of query stack

------------------------------------------
------------------------------------------


---- [ui] ui/proc-macro/nonterminal-token-hygiene.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/nonterminal-token-hygiene.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/nonterminal-token-hygiene" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "span-debug" "-Z" "macro-backtrace" "-Z" "unpretty=expanded,hygiene" "-Z" "trim-diagnostic-paths=no" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/nonterminal-token-hygiene/auxiliary"
------------------------------------------
------------------------------------------
PRINT-BANG INPUT (DISPLAY): struct S;
PRINT-BANG RE-COLLECTED (DISPLAY): struct S ;
PRINT-BANG INPUT (DEBUG): TokenStream [
    Group {
        delimiter: None,
        stream: TokenStream [
                ident: "struct",
                ident: "struct",
                span: /checkout/src/test/ui/proc-macro/nonterminal-token-hygiene.rs:30:5: 30:11 (#5),
            Ident {
                ident: "S",
                ident: "S",
                span: /checkout/src/test/ui/proc-macro/nonterminal-token-hygiene.rs:30:12: 30:13 (#5),
            Punct {
            Punct {
                ch: ';',
                spacing: Alone,
                span: /checkout/src/test/ui/proc-macro/nonterminal-token-hygiene.rs:30:13: 30:14 (#5),
        ],
        ],
        span: /checkout/src/test/ui/proc-macro/nonterminal-token-hygiene.rs:20:27: 20:32 (#6),
]

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'already borrowed: BorrowMutError', compiler/rustc_interface/src/queries.rs:36:38

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (16635a6c4 2021-06-20) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z span-debug -Z macro-backtrace -Z unpretty=expanded,hygiene -Z trim-diagnostic-paths=no -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack

------------------------------------------
------------------------------------------


---- [ui] ui/rfc-2497-if-let-chains/ast-pretty-check.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2497-if-let-chains/ast-pretty-check.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/ast-pretty-check" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unpretty=expanded" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/ast-pretty-check/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'already borrowed: BorrowMutError', compiler/rustc_interface/src/queries.rs:36:38

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (16635a6c4 2021-06-20) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unpretty=expanded -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack

------------------------------------------
---
test result: FAILED. 11881 passed; 13 failed; 101 ignored; 0 measured; 0 filtered out; finished in 100.75s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:10:08
