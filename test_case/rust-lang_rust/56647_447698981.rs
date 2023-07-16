plain
[00:44:25] failures:
[00:44:25] 
[00:44:25] ---- [ui] ui/proc-macro/dollar-crate.rs stdout ----
[00:44:25] 
[00:44:25] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:44:25] status: exit code: 101
[00:44:25] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/dollar-crate.rs" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/dollar-crate/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "--edition=2018" "--extern" "dollar_crate" "--extern" "dollar_crate_external" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/dollar-crate/auxiliary" "-A" "unused"
[00:44:25] ------------------------------------------
[00:44:25] ------------------------------------------
[00:44:25] PROC MACRO INPUT (PRETTY-PRINTED): struct M ( $crate :: S ) ;
[00:44:25] PROC MACRO INPUT: TokenStream [
[00:44:25]     Ident {
[00:44:25]         ident: "struct",
[00:44:25]         span: #2 bytes(491..497)
[00:44:25]     Ident {
[00:44:25]         ident: "M",
[00:44:25]         ident: "M",
[00:44:25]         span: #2 bytes(498..499)
[00:44:25]     Group {
[00:44:25]         delimiter: Parenthesis,
[00:44:25]         delimiter: Parenthesis,
[00:44:25]         stream: TokenStream [
[00:44:25]             Ident {
[00:44:25]                 ident: "$crate",
[00:44:25]                 span: #2 bytes(500..506)
[00:44:25]             Punct {
[00:44:25]             Punct {
[00:44:25]                 ch: ':',
[00:44:25]                 spacing: Joint,
[00:44:25]                 span: #2 bytes(506..508)
[00:44:25]             Punct {
[00:44:25]             Punct {
[00:44:25]                 ch: ':',
[00:44:25]                 spacing: Alone,
[00:44:25]                 span: #2 bytes(506..508)
[00:44:25]             Ident {
[00:44:25]                 ident: "S",
[00:44:25]                 ident: "S",
[00:44:25]                 span: #2 bytes(508..509)
[00:44:25]         ],
[00:44:25]         ],
[00:44:25]         span: #2 bytes(499..510)
[00:44:25]     Punct {
[00:44:25]     Punct {
[00:44:25]         ch: ';',
[00:44:25]         spacing: Alone,
[00:44:25]         span: #2 bytes(510..511)
[00:44:25] ]
[00:44:25] ]
[00:44:25] ATTRIBUTE INPUT (PRETTY-PRINTED): struct A(crate::S);
[00:44:25] ATTRIBUTE INPUT: TokenStream [
[00:44:25]     Ident {
[00:44:25]         ident: "struct",
[00:44:25]         span: #2 bytes(570..576)
[00:44:25]     Ident {
[00:44:25]         ident: "A",
[00:44:25]         ident: "A",
[00:44:25]         span: #2 bytes(577..578)
[00:44:25]     Group {
[00:44:25]         delimiter: Parenthesis,
[00:44:25]         delimiter: Parenthesis,
[00:44:25]         stream: TokenStream [
[00:44:25]             Ident {
[00:44:25]                 ident: "$crate",
[00:44:25]                 span: #2 bytes(579..585)
[00:44:25]             Punct {
[00:44:25]             Punct {
[00:44:25]                 ch: ':',
[00:44:25]                 spacing: Joint,
[00:44:25]                 span: #2 bytes(585..587)
[00:44:25]             Punct {
[00:44:25]             Punct {
[00:44:25]                 ch: ':',
[00:44:25]                 spacing: Alone,
[00:44:25]                 span: #2 bytes(585..587)
[00:44:25]             Ident {
[00:44:25]                 ident: "S",
[00:44:25]                 ident: "S",
[00:44:25]                 span: #2 bytes(587..588)
[00:44:25]         ],
[00:44:25]         ],
[00:44:25]         span: #2 bytes(578..589)
[00:44:25]     Punct {
[00:44:25]     Punct {
[00:44:25]         ch: ';',
[00:44:25]         spacing: Alone,
[00:44:25]         span: #2 bytes(589..590)
[00:44:25] ]
[00:44:25] ]
[00:44:25] DERIVE INPUT (PRETTY-PRINTED): struct D(crate::S);
[00:44:25] DERIVE INPUT: TokenStream [
[00:44:25]     Ident {
[00:44:25]         ident: "struct",
[00:44:25]         span: #2 bytes(643..649)
[00:44:25]     Ident {
[00:44:25]         ident: "D",
[00:44:25]         ident: "D",
[00:44:25]         span: #2 bytes(650..651)
[00:44:25]     Group {
[00:44:25]         delimiter: Parenthesis,
[00:44:25]         delimiter: Parenthesis,
[00:44:25]         stream: TokenStream [
[00:44:25]             Ident {
[00:44:25]                 ident: "$crate",
[00:44:25]                 span: #2 bytes(652..658)
[00:44:25]             Punct {
[00:44:25]             Punct {
[00:44:25]                 ch: ':',
[00:44:25]                 spacing: Joint,
[00:44:25]                 span: #2 bytes(658..660)
[00:44:25]             Punct {
[00:44:25]             Punct {
[00:44:25]                 ch: ':',
[00:44:25]                 spacing: Alone,
[00:44:25]                 span: #2 bytes(658..660)
[00:44:25]             Ident {
[00:44:25]                 ident: "S",
[00:44:25]                 ident: "S",
[00:44:25]                 span: #2 bytes(660..661)
[00:44:25]         ],
[00:44:25]         ],
[00:44:25]         span: #2 bytes(651..662)
[00:44:25]     Punct {
[00:44:25]     Punct {
[00:44:25]         ch: ';',
[00:44:25]         spacing: Alone,
[00:44:25]         span: #2 bytes(662..663)
[00:44:25] ]
[00:44:25] ]
[00:44:25] PROC MACRO INPUT (PRETTY-PRINTED): struct M ( $crate :: S ) ;
[00:44:25] PROC MACRO INPUT: TokenStream [
[00:44:25]     Ident {
[00:44:25]         ident: "struct",
[00:44:25]         span: #10 bytes(8067617..8067623)
[00:44:25]     Ident {
[00:44:25]         ident: "M",
[00:44:25]         ident: "M",
[00:44:25]         span: #10 bytes(8067624..8067625)
[00:44:25]     Group {
[00:44:25]         delimiter: Parenthesis,
[00:44:25]         delimiter: Parenthesis,
[00:44:25]         stream: TokenStream [
[00:44:25]             Ident {
[00:44:25]                 ident: "$crate",
[00:44:25]                 span: #10 bytes(8067628..8067635)
[00:44:25]             Punct {
[00:44:25]             Punct {
[00:44:25]                 ch: ':',
[00:44:25]                 spacing: Joint,
[00:44:25]                 span: #10 bytes(8067636..8067638)
[00:44:25]             Punct {
[00:44:25]             Punct {
[00:44:25]                 ch: ':',
[00:44:25]                 spacing: Alone,
[00:44:25]                 span: #10 bytes(8067636..8067638)
[00:44:25]             Ident {
[00:44:25]                 ident: "S",
[00:44:25]                 ident: "S",
[00:44:25]                 span: #10 bytes(8067639..8067640)
[00:44:25]         ],
[00:44:25]         ],
[00:44:25]         span: #10 bytes(8067626..8067642)
[00:44:25]     Punct {
[00:44:25]     Punct {
[00:44:25]         ch: ';',
[00:44:25]         spacing: Alone,
[00:44:25]         span: #10 bytes(8067643..8067644)
[00:44:25] ]
[00:44:25] ]
[00:44:25] ATTRIBUTE INPUT (PRETTY-PRINTED): struct A(::dollar_crate_external::S);
[00:44:25] ATTRIBUTE INPUT: TokenStream [
[00:44:25]     Ident {
[00:44:25]         ident: "struct",
[00:44:25]         span: #10 bytes(8067671..8067677)
[00:44:25]     Ident {
[00:44:25]         ident: "A",
[00:44:25]         ident: "A",
[00:44:25]         span: #10 bytes(8067678..8067679)
[00:44:25]     Group {
[00:44:25]         delimiter: Parenthesis,
[00:44:25]         delimiter: Parenthesis,
[00:44:25]         stream: TokenStream [
[00:44:25]             Ident {
[00:44:25]                 ident: "$crate",
[00:44:25]                 span: #10 bytes(8067682..8067689)
[00:44:25]             Punct {
[00:44:25]             Punct {
[00:44:25]                 ch: ':',
[00:44:25]                 spacing: Joint,
[00:44:25]                 span: #10 bytes(8067690..8067692)
[00:44:25]             Punct {
[00:44:25]             Punct {
[00:44:25]                 ch: ':',
[00:44:25]                 spacing: Alone,
[00:44:25]                 span: #10 bytes(8067690..8067692)
[00:44:25]             Ident {
[00:44:25]                 ident: "S",
[00:44:25]                 ident: "S",
[00:44:25]                 span: #10 bytes(8067693..8067694)
[00:44:25]         ],
[00:44:25]         ],
[00:44:25]         span: #10 bytes(8067680..8067696)
[00:44:25]     Punct {
[00:44:25]     Punct {
[00:44:25]         ch: ';',
[00:44:25]         spacing: Alone,
[00:44:25]         span: #10 bytes(8067697..8067698)
[00:44:25] ]
[00:44:25] ]
[00:44:25] DERIVE INPUT (PRETTY-PRINTED): struct D(::dollar_crate_external::S);
[00:44:25] DERIVE INPUT: TokenStream [
[00:44:25]     Ident {
[00:44:25]         ident: "struct",
[00:44:25]         span: #10 bytes(8067734..8067740)
[00:44:25]     Ident {
[00:44:25]         ident: "D",
[00:44:25]         ident: "D",
[00:44:25]         span: #10 bytes(8067741..8067742)
[00:44:25]     Group {
[00:44:25]         delimiter: Parenthesis,
[00:44:25]         delimiter: Parenthesis,
[00:44:25]         stream: TokenStream [
[00:44:25]             Ident {
[00:44:25]                 ident: "$crate",
[00:44:25]                 span: #10 bytes(8067745..8067752)
[00:44:25]             Punct {
[00:44:25]             Punct {
[00:44:25]                 ch: ':',
[00:44:25]                 spacing: Joint,
[00:44:25]                 span: #10 bytes(8067753..8067755)
[00:44:25]             Punct {
[00:44:25]             Punct {
[00:44:25]                 ch: ':',
[00:44:25]                 spacing: Alone,
[00:44:25]                 span: #10 bytes(8067753..8067755)
[00:44:25]             Ident {
[00:44:25]                 ident: "S",
[00:44:25]                 ident: "S",
[00:44:25]                 span: #10 bytes(8067756..8067757)
[00:44:25]         ],
[00:44:25]         ],
[00:44:25]         span: #10 bytes(8067743..8067759)
[00:44:25]     Punct {
[00:44:25]     Punct {
[00:44:25]         ch: ';',
[00:44:25]         spacing: Alone,
[00:44:25]         span: #10 bytes(8067760..8067761)
[00:44:25] ]
[00:44:25] 
[00:44:25] ------------------------------------------
[00:44:25] stderr:
[00:44:25] stderr:
[00:44:25] ------------------------------------------
[00:44:25] {"message":"the name `D` is defined multiple times","code":{"code":"E0428","explanation":"\nA type or module has been defined more than once.\n\nErroneous code example:\n\n