plain
[00:55:04] failures:
[00:55:04] 
[00:55:04] ---- [ui] ui/proc-macro/dollar-crate.rs stdout ----
[00:55:04] 
[00:55:04] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:55:04] status: exit code: 101
[00:55:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/dollar-crate.rs" "--target=arm-unknown-linux-gnueabihf" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/dollar-crate/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/arm-unknown-linux-gnueabihf/native/rust-test-helpers" "-Clinker=arm-linux-gnueabihf-gcc" "--edition=2018" "--extern" "dollar_crate" "--extern" "dollar_crate_external" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/dollar-crate/auxiliary" "-A" "unused"
[00:55:04] ------------------------------------------
[00:55:04] ------------------------------------------
[00:55:04] PROC MACRO INPUT (PRETTY-PRINTED): struct M ( $crate :: S ) ;
[00:55:04] PROC MACRO INPUT: TokenStream [
[00:55:04]     Ident {
[00:55:04]         ident: "struct",
[00:55:04]         span: #2 bytes(491..497)
[00:55:04]     Ident {
[00:55:04]         ident: "M",
[00:55:04]         ident: "M",
[00:55:04]         span: #2 bytes(498..499)
[00:55:04]     Group {
[00:55:04]         delimiter: Parenthesis,
[00:55:04]         delimiter: Parenthesis,
[00:55:04]         stream: TokenStream [
[00:55:04]             Ident {
[00:55:04]                 ident: "$crate",
[00:55:04]                 span: #2 bytes(500..506)
[00:55:04]             Punct {
[00:55:04]             Punct {
[00:55:04]                 ch: ':',
[00:55:04]                 spacing: Joint,
[00:55:04]                 span: #2 bytes(506..508)
[00:55:04]             Punct {
[00:55:04]             Punct {
[00:55:04]                 ch: ':',
[00:55:04]                 spacing: Alone,
[00:55:04]                 span: #2 bytes(506..508)
[00:55:04]             Ident {
[00:55:04]                 ident: "S",
[00:55:04]                 ident: "S",
[00:55:04]                 span: #2 bytes(508..509)
[00:55:04]         ],
[00:55:04]         ],
[00:55:04]         span: #2 bytes(499..510)
[00:55:04]     Punct {
[00:55:04]     Punct {
[00:55:04]         ch: ';',
[00:55:04]         spacing: Alone,
[00:55:04]         span: #2 bytes(510..511)
[00:55:04] ]
[00:55:04] ]
[00:55:04] ATTRIBUTE INPUT (PRETTY-PRINTED): struct A(crate::S);
[00:55:04] ATTRIBUTE INPUT: TokenStream [
[00:55:04]     Ident {
[00:55:04]         ident: "struct",
[00:55:04]         span: #2 bytes(570..576)
[00:55:04]     Ident {
[00:55:04]         ident: "A",
[00:55:04]         ident: "A",
[00:55:04]         span: #2 bytes(577..578)
[00:55:04]     Group {
[00:55:04]         delimiter: Parenthesis,
[00:55:04]         delimiter: Parenthesis,
[00:55:04]         stream: TokenStream [
[00:55:04]             Ident {
[00:55:04]                 ident: "$crate",
[00:55:04]                 span: #2 bytes(579..585)
[00:55:04]             Punct {
[00:55:04]             Punct {
[00:55:04]                 ch: ':',
[00:55:04]                 spacing: Joint,
[00:55:04]                 span: #2 bytes(585..587)
[00:55:04]             Punct {
[00:55:04]             Punct {
[00:55:04]                 ch: ':',
[00:55:04]                 spacing: Alone,
[00:55:04]                 span: #2 bytes(585..587)
[00:55:04]             Ident {
[00:55:04]                 ident: "S",
[00:55:04]                 ident: "S",
[00:55:04]                 span: #2 bytes(587..588)
[00:55:04]         ],
[00:55:04]         ],
[00:55:04]         span: #2 bytes(578..589)
[00:55:04]     Punct {
[00:55:04]     Punct {
[00:55:04]         ch: ';',
[00:55:04]         spacing: Alone,
[00:55:04]         span: #2 bytes(589..590)
[00:55:04] ]
[00:55:04] ]
[00:55:04] DERIVE INPUT (PRETTY-PRINTED): struct D(crate::S);
[00:55:04] DERIVE INPUT: TokenStream [
[00:55:04]     Ident {
[00:55:04]         ident: "struct",
[00:55:04]         span: #2 bytes(643..649)
[00:55:04]     Ident {
[00:55:04]         ident: "D",
[00:55:04]         ident: "D",
[00:55:04]         span: #2 bytes(650..651)
[00:55:04]     Group {
[00:55:04]         delimiter: Parenthesis,
[00:55:04]         delimiter: Parenthesis,
[00:55:04]         stream: TokenStream [
[00:55:04]             Ident {
[00:55:04]                 ident: "$crate",
[00:55:04]                 span: #2 bytes(652..658)
[00:55:04]             Punct {
[00:55:04]             Punct {
[00:55:04]                 ch: ':',
[00:55:04]                 spacing: Joint,
[00:55:04]                 span: #2 bytes(658..660)
[00:55:04]             Punct {
[00:55:04]             Punct {
[00:55:04]                 ch: ':',
[00:55:04]                 spacing: Alone,
[00:55:04]                 span: #2 bytes(658..660)
[00:55:04]             Ident {
[00:55:04]                 ident: "S",
[00:55:04]                 ident: "S",
[00:55:04]                 span: #2 bytes(660..661)
[00:55:04]         ],
[00:55:04]         ],
[00:55:04]         span: #2 bytes(651..662)
[00:55:04]     Punct {
[00:55:04]     Punct {
[00:55:04]         ch: ';',
[00:55:04]         spacing: Alone,
[00:55:04]         span: #2 bytes(662..663)
[00:55:04] ]
[00:55:04] ]
[00:55:04] PROC MACRO INPUT (PRETTY-PRINTED): struct M ( $crate :: S ) ;
[00:55:04] PROC MACRO INPUT: TokenStream [
[00:55:04]     Ident {
[00:55:04]         ident: "struct",
[00:55:04]         span: #10 bytes(7045904..7045910)
[00:55:04]     Ident {
[00:55:04]         ident: "M",
[00:55:04]         ident: "M",
[00:55:04]         span: #10 bytes(7045911..7045912)
[00:55:04]     Group {
[00:55:04]         delimiter: Parenthesis,
[00:55:04]         delimiter: Parenthesis,
[00:55:04]         stream: TokenStream [
[00:55:04]             Ident {
[00:55:04]                 ident: "$crate",
[00:55:04]                 span: #10 bytes(7045915..7045922)
[00:55:04]             Punct {
[00:55:04]             Punct {
[00:55:04]                 ch: ':',
[00:55:04]                 spacing: Joint,
[00:55:04]                 span: #10 bytes(7045923..7045925)
[00:55:04]             Punct {
[00:55:04]             Punct {
[00:55:04]                 ch: ':',
[00:55:04]                 spacing: Alone,
[00:55:04]                 span: #10 bytes(7045923..7045925)
[00:55:04]             Ident {
[00:55:04]                 ident: "S",
[00:55:04]                 ident: "S",
[00:55:04]                 span: #10 bytes(7045926..7045927)
[00:55:04]         ],
[00:55:04]         ],
[00:55:04]         span: #10 bytes(7045913..7045929)
[00:55:04]     Punct {
[00:55:04]     Punct {
[00:55:04]         ch: ';',
[00:55:04]         spacing: Alone,
[00:55:04]         span: #10 bytes(7045930..7045931)
[00:55:04] ]
[00:55:04] ]
[00:55:04] ATTRIBUTE INPUT (PRETTY-PRINTED): struct A(::dollar_crate_external::S);
[00:55:04] ATTRIBUTE INPUT: TokenStream [
[00:55:04]     Ident {
[00:55:04]         ident: "struct",
[00:55:04]         span: #10 bytes(7045958..7045964)
[00:55:04]     Ident {
[00:55:04]         ident: "A",
[00:55:04]         ident: "A",
[00:55:04]         span: #10 bytes(7045965..7045966)
[00:55:04]     Group {
[00:55:04]         delimiter: Parenthesis,
[00:55:04]         delimiter: Parenthesis,
[00:55:04]         stream: TokenStream [
[00:55:04]             Ident {
[00:55:04]                 ident: "$crate",
[00:55:04]                 span: #10 bytes(7045969..7045976)
[00:55:04]             Punct {
[00:55:04]             Punct {
[00:55:04]                 ch: ':',
[00:55:04]                 spacing: Joint,
[00:55:04]                 span: #10 bytes(7045977..7045979)
[00:55:04]             Punct {
[00:55:04]             Punct {
[00:55:04]                 ch: ':',
[00:55:04]                 spacing: Alone,
[00:55:04]                 span: #10 bytes(7045977..7045979)
[00:55:04]             Ident {
[00:55:04]                 ident: "S",
[00:55:04]                 ident: "S",
[00:55:04]                 span: #10 bytes(7045980..7045981)
[00:55:04]         ],
[00:55:04]         ],
[00:55:04]         span: #10 bytes(7045967..7045983)
[00:55:04]     Punct {
[00:55:04]     Punct {
[00:55:04]         ch: ';',
[00:55:04]         spacing: Alone,
[00:55:04]         span: #10 bytes(7045984..7045985)
[00:55:04] ]
[00:55:04] ]
[00:55:04] DERIVE INPUT (PRETTY-PRINTED): struct D(::dollar_crate_external::S);
[00:55:04] DERIVE INPUT: TokenStream [
[00:55:04]     Ident {
[00:55:04]         ident: "struct",
[00:55:04]         span: #10 bytes(7046021..7046027)
[00:55:04]     Ident {
[00:55:04]         ident: "D",
[00:55:04]         ident: "D",
[00:55:04]         span: #10 bytes(7046028..7046029)
[00:55:04]     Group {
[00:55:04]         delimiter: Parenthesis,
[00:55:04]         delimiter: Parenthesis,
[00:55:04]         stream: TokenStream [
[00:55:04]             Ident {
[00:55:04]                 ident: "$crate",
[00:55:04]                 span: #10 bytes(7046032..7046039)
[00:55:04]             Punct {
[00:55:04]             Punct {
[00:55:04]                 ch: ':',
[00:55:04]                 spacing: Joint,
[00:55:04]                 span: #10 bytes(7046040..7046042)
[00:55:04]             Punct {
[00:55:04]             Punct {
[00:55:04]                 ch: ':',
[00:55:04]                 spacing: Alone,
[00:55:04]                 span: #10 bytes(7046040..7046042)
[00:55:04]             Ident {
[00:55:04]                 ident: "S",
[00:55:04]                 ident: "S",
[00:55:04]                 span: #10 bytes(7046043..7046044)
[00:55:04]         ],
[00:55:04]         ],
[00:55:04]         span: #10 bytes(7046030..7046046)
[00:55:04]     Punct {
[00:55:04]     Punct {
[00:55:04]         ch: ';',
[00:55:04]         spacing: Alone,
[00:55:04]         span: #10 bytes(7046047..7046048)
[00:55:04] ]
[00:55:04] 
[00:55:04] ------------------------------------------
[00:55:04] stderr:
[00:55:04] stderr:
[00:55:04] ------------------------------------------
[00:55:04] {"message":"the name `D` is defined multiple times","code":{"code":"E0428","explanation":"\nA type or module has been defined more than once.\n\nErroneous code example:\n\n