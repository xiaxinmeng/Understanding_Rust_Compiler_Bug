plain

---- [ui] src/test/ui/proc-macro/expand-to-derive.rs stdout ----
diff of stdout:

1 PRINT-DERIVE INPUT (DISPLAY): struct Foo
-     field :
+     field_field_field :
+     field_field_field :
4     [bool ; { #[rustc_dummy] struct Inner { other_inner_field : u8, } 0 }]
5 }
6 PRINT-DERIVE INPUT (DEBUG): TokenStream [
16         delimiter: Brace,
16         delimiter: Brace,
17         stream: TokenStream [
18             Ident {
-                 ident: "field",
-                 span: $DIR/expand-to-derive.rs:18:13: 18:18 (#4),
+                 ident: "field_field_field",
+                 span: $DIR/expand-to-derive.rs:18:13: 18:30 (#4),
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
22             Punct {
23                 ch: ':',

24                 spacing: Alone,
24                 spacing: Alone,
-                 span: $DIR/expand-to-derive.rs:18:18: 18:19 (#4),
+                 span: $DIR/expand-to-derive.rs:18:30: 18:31 (#4),
27             Group {
28                 delimiter: Bracket,


29                 stream: TokenStream [
30                     Ident {
31                         ident: "bool",
-                         span: $DIR/expand-to-derive.rs:18:21: 18:25 (#4),
+                         span: $DIR/expand-to-derive.rs:18:33: 18:37 (#4),
34                     Punct {
34                     Punct {
35                         ch: ';',
36                         spacing: Alone,
36                         spacing: Alone,
-                         span: $DIR/expand-to-derive.rs:18:25: 18:26 (#4),
+                         span: $DIR/expand-to-derive.rs:18:37: 18:38 (#4),
39                     Group {
40                         delimiter: Brace,


93                                 span: $DIR/expand-to-derive.rs:20:17: 20:18 (#4),
95                         ],
95                         ],
-                         span: $DIR/expand-to-derive.rs:18:27: 21:14 (#4),
+                         span: $DIR/expand-to-derive.rs:18:39: 21:14 (#4),
98                 ],
98                 ],
-                 span: $DIR/expand-to-derive.rs:18:20: 21:15 (#4),
+                 span: $DIR/expand-to-derive.rs:18:32: 21:15 (#4),
101         ],
101         ],
102         span: $DIR/expand-to-derive.rs:16:20: 22:10 (#4),

The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/expand-to-derive/expand-to-derive.stdout
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args proc-macro/expand-to-derive.rs`

error: 1 errors occurred comparing output.
failed to decode compiler output as json: line: {
output: PRINT-DERIVE INPUT (DISPLAY): struct Foo
    field_field_field :
    field_field_field :
    [bool ; { #[rustc_dummy] struct Inner { other_inner_field : u8, } 0 }]
}
PRINT-DERIVE INPUT (DEBUG): TokenStream [
        ident: "struct",
        ident: "struct",
        span: /checkout/src/test/ui/proc-macro/expand-to-derive.rs:16:9: 16:15 (#4),
    Ident {
        ident: "Foo",
        ident: "Foo",
        span: /checkout/src/test/ui/proc-macro/expand-to-derive.rs:16:16: 16:19 (#4),
    Group {
        delimiter: Brace,
        delimiter: Brace,
        stream: TokenStream [
            Ident {
                ident: "field_field_field",
                span: /checkout/src/test/ui/proc-macro/expand-to-derive.rs:18:13: 18:30 (#4),
            Punct {
                ch: ':',
                spacing: Alone,
                spacing: Alone,
                span: /checkout/src/test/ui/proc-macro/expand-to-derive.rs:18:30: 18:31 (#4),
            Group {
                delimiter: Bracket,
                delimiter: Bracket,
                stream: TokenStream [
                    Ident {
                        ident: "bool",
                        span: /checkout/src/test/ui/proc-macro/expand-to-derive.rs:18:33: 18:37 (#4),
                    Punct {
                    Punct {
                        ch: ';',
                        spacing: Alone,
                        span: /checkout/src/test/ui/proc-macro/expand-to-derive.rs:18:37: 18:38 (#4),
                    Group {
                        delimiter: Brace,
                        delimiter: Brace,
                        stream: TokenStream [
                            Punct {
                                ch: '#',
                                spacing: Alone,
                                span: /checkout/src/test/ui/proc-macro/expand-to-derive.rs:27:5: 27:6 (#0),
                            Group {
                                delimiter: Bracket,
                                delimiter: Bracket,
                                stream: TokenStream [
                                    Ident {
                                        ident: "rustc_dummy",
                                        span: /checkout/src/test/ui/proc-macro/expand-to-derive.rs:27:28: 27:39 (#0),
                                ],
                                ],
                                span: /checkout/src/test/ui/proc-macro/expand-to-derive.rs:27:5: 27:6 (#0),
                            Ident {
                                ident: "struct",
                                ident: "struct",
                                span: /checkout/src/test/ui/proc-macro/expand-to-derive.rs:28:5: 28:11 (#0),
                            Ident {
                                ident: "Inner",
                                ident: "Inner",
                                span: /checkout/src/test/ui/proc-macro/expand-to-derive.rs:28:12: 28:17 (#0),
                            Group {
                                delimiter: Brace,
                                delimiter: Brace,
                                stream: TokenStream [
                                    Ident {
                                        ident: "other_inner_field",
                                        span: /checkout/src/test/ui/proc-macro/expand-to-derive.rs:30:9: 30:26 (#0),
                                    Punct {
                                        ch: ':',
                                        spacing: Alone,
                                        spacing: Alone,
                                        span: /checkout/src/test/ui/proc-macro/expand-to-derive.rs:30:26: 30:27 (#0),
                                    Ident {
                                        ident: "u8",
                                        ident: "u8",
                                        span: /checkout/src/test/ui/proc-macro/expand-to-derive.rs:30:28: 30:30 (#0),
                                    Punct {
                                    Punct {
                                        ch: ',',
                                        spacing: Alone,
                                        span: /checkout/src/test/ui/proc-macro/expand-to-derive.rs:30:30: 30:31 (#0),
                                ],
                                ],
                                span: /checkout/src/test/ui/proc-macro/expand-to-derive.rs:28:18: 31:6 (#0),
                            Literal {
                                kind: Integer,
                                symbol: "0",
                                suffix: None,
                                suffix: None,
                                span: /checkout/src/test/ui/proc-macro/expand-to-derive.rs:20:17: 20:18 (#4),
                        ],
                        ],
                        span: /checkout/src/test/ui/proc-macro/expand-to-derive.rs:18:39: 21:14 (#4),
                ],
                ],
                span: /checkout/src/test/ui/proc-macro/expand-to-derive.rs:18:32: 21:15 (#4),
        ],
        ],
        span: /checkout/src/test/ui/proc-macro/expand-to-derive.rs:16:20: 22:10 (#4),
]
thread '[ui] src/test/ui/proc-macro/expand-to-derive.rs' panicked at 'explicit panic', src/tools/compiletest/src/json.rs:132:21


