plain
.......i............................................................................................ 8600/12038
.................................................................................................... 8700/12038
.................................................................................................... 8800/12038
.................................................................................................... 8900/12038
...................................................................................F.F.............. 9000/12038
.........iiii.iiiii......................F.........................F....F..........ii............... 9100/12038
.................................................................................................... 9300/12038
.................................................................................................... 9400/12038
.................................................................................................... 9500/12038
.................................................................................................... 9600/12038
---

---- [ui] ui/proc-macro/debug/dump-debug-span-debug.rs stdout ----
diff of stderr:

68         span: $DIR/dump-debug-span-debug.rs:20:5: 20:9 (#0),
70     Literal {
70     Literal {
-         kind: StrRaw(0),
+         kind: StrRaw(
+         ),
72         symbol: "R",
73         suffix: None,
73         suffix: None,
74         span: $DIR/dump-debug-span-debug.rs:21:5: 21:9 (#0),
75     },
76     Literal {
76     Literal {
-         kind: StrRaw(2),
+         kind: StrRaw(
+         ),
78         symbol: "R",
79         suffix: None,
79         suffix: None,
80         span: $DIR/dump-debug-span-debug.rs:22:5: 22:13 (#0),
81     },
82     Literal {
82     Literal {
-         kind: ByteStrRaw(0),
+         kind: ByteStrRaw(
+         ),
84         symbol: "BR",
85         suffix: None,
85         suffix: None,
86         span: $DIR/dump-debug-span-debug.rs:23:5: 23:11 (#0),
87     },
88     Literal {
88     Literal {
-         kind: ByteStrRaw(2),
+         kind: ByteStrRaw(
+         ),
90         symbol: "BR",
91         suffix: None,
91         suffix: None,
92         span: $DIR/dump-debug-span-debug.rs:24:5: 24:15 (#0),

128         span: $DIR/dump-debug-span-debug.rs:32:5: 32:10 (#0),
130     Literal {
130     Literal {
-         kind: StrRaw(0),
+         kind: StrRaw(
+         ),
132         symbol: "R",
132         symbol: "R",
133         suffix: Some("q"),
134         span: $DIR/dump-debug-span-debug.rs:33:5: 33:10 (#0),
135     },
136     Literal {
136     Literal {
-         kind: StrRaw(2),
+         kind: StrRaw(
+         ),
138         symbol: "R",
138         symbol: "R",
139         suffix: Some("q"),
140         span: $DIR/dump-debug-span-debug.rs:34:5: 34:14 (#0),
141     },
142     Literal {
142     Literal {
-         kind: ByteStrRaw(0),
+         kind: ByteStrRaw(
+         ),
144         symbol: "BR",
144         symbol: "BR",
145         suffix: Some("q"),
146         span: $DIR/dump-debug-span-debug.rs:35:5: 35:12 (#0),
147     },
148     Literal {
148     Literal {
-         kind: ByteStrRaw(2),
+         kind: ByteStrRaw(
+         ),
150         symbol: "BR",
150         symbol: "BR",
151         suffix: Some("q"),
152         span: $DIR/dump-debug-span-debug.rs:36:5: 36:16 (#0),

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/debug/dump-debug-span-debug/dump-debug-span-debug.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args proc-macro/debug/dump-debug-span-debug.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/debug/dump-debug-span-debug/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "span-debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/debug/dump-debug-span-debug/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
TokenStream [Ident { ident: "ident", span: /checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs:9:5: 9:10 (#0) }, Ident { ident: "r#ident", span: /checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs:10:5: 10:12 (#0) }, Punct { ch: ',', spacing: Alone, span: /checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs:11:5: 11:6 (#0) }, Punct { ch: '=', spacing: Joint, span: /checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs:12:5: 12:7 (#0) }, Punct { ch: '=', spacing: Joint, span: /checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs:12:5: 12:7 (#0) }, Punct { ch: '>', spacing: Alone, span: /checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs:12:7: 12:8 (#0) }, Group { delimiter: Parenthesis, stream: TokenStream [], span: /checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs:13:5: 13:7 (#0) }, Group { delimiter: Bracket, stream: TokenStream [Ident { ident: "_", span: /checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs:14:6: 14:7 (#0) }], span: /checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs:14:5: 14:8 (#0) }, Literal { kind: Integer, symbol: "0", suffix: None, span: /checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs:17:5: 17:6 (#0) }, Literal { kind: Float, symbol: "1.0", suffix: None, span: /checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs:18:5: 18:8 (#0) }, Literal { kind: Str, symbol: "S", suffix: None, span: /checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs:19:5: 19:8 (#0) }, Literal { kind: ByteStr, symbol: "B", suffix: None, span: /checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs:20:5: 20:9 (#0) }, Literal { kind: StrRaw(0), symbol: "R", suffix: None, span: /checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs:21:5: 21:9 (#0) }, Literal { kind: StrRaw(2), symbol: "R", suffix: None, span: /checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs:22:5: 22:13 (#0) }, Literal { kind: ByteStrRaw(0), symbol: "BR", suffix: None, span: /checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs:23:5: 23:11 (#0) }, Literal { kind: ByteStrRaw(2), symbol: "BR", suffix: None, span: /checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs:24:5: 24:15 (#0) }, Literal { kind: Char, symbol: "C", suffix: None, span: /checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs:25:5: 25:8 (#0) }, Literal { kind: Byte, symbol: "B", suffix: None, span: /checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs:26:5: 26:9 (#0) }, Literal { kind: Integer, symbol: "0", suffix: Some("q"), span: /checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs:29:5: 29:7 (#0) }, Literal { kind: Float, symbol: "1.0", suffix: Some("q"), span: /checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs:30:5: 30:9 (#0) }, Literal { kind: Str, symbol: "S", suffix: Some("q"), span: /checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs:31:5: 31:9 (#0) }, Literal { kind: ByteStr, symbol: "B", suffix: Some("q"), span: /checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs:32:5: 32:10 (#0) }, Literal { kind: StrRaw(0), symbol: "R", suffix: Some("q"), span: /checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs:33:5: 33:10 (#0) }, Literal { kind: StrRaw(2), symbol: "R", suffix: Some("q"), span: /checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs:34:5: 34:14 (#0) }, Literal { kind: ByteStrRaw(0), symbol: "BR", suffix: Some("q"), span: /checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs:35:5: 35:12 (#0) }, Literal { kind: ByteStrRaw(2), symbol: "BR", suffix: Some("q"), span: /checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs:36:5: 36:16 (#0) }, Literal { kind: Char, symbol: "C", suffix: Some("q"), span: /checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs:37:5: 37:9 (#0) }, Literal { kind: Byte, symbol: "B", suffix: Some("q"), span: /checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs:38:5: 38:10 (#0) }]
TokenStream [
    Ident {
        ident: "ident",
        span: /checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs:9:5: 9:10 (#0),
    Ident {
    Ident {
        ident: "r#ident",
        span: /checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs:10:5: 10:12 (#0),
    Punct {
    Punct {
        ch: ',',
        spacing: Alone,
        span: /checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs:11:5: 11:6 (#0),
    Punct {
    Punct {
        ch: '=',
        spacing: Joint,
        span: /checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs:12:5: 12:7 (#0),
    Punct {
    Punct {
        ch: '=',
        spacing: Joint,
        span: /checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs:12:5: 12:7 (#0),
    Punct {
    Punct {
        ch: '>',
        spacing: Alone,
        span: /checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs:12:7: 12:8 (#0),
    Group {
        delimiter: Parenthesis,
        delimiter: Parenthesis,
        stream: TokenStream [],
        span: /checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs:13:5: 13:7 (#0),
    Group {
        delimiter: Bracket,
        delimiter: Bracket,
        stream: TokenStream [
            Ident {
                ident: "_",
                span: /checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs:14:6: 14:7 (#0),
        ],
        ],
        span: /checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs:14:5: 14:8 (#0),
    Literal {
        kind: Integer,
        symbol: "0",
        suffix: None,
        suffix: None,
        span: /checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs:17:5: 17:6 (#0),
    Literal {
        kind: Float,
        symbol: "1.0",
        suffix: None,
        suffix: None,
        span: /checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs:18:5: 18:8 (#0),
    Literal {
        kind: Str,
        symbol: "S",
        suffix: None,
        suffix: None,
        span: /checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs:19:5: 19:8 (#0),
    Literal {
        kind: ByteStr,
        symbol: "B",
        suffix: None,
        suffix: None,
        span: /checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs:20:5: 20:9 (#0),
    Literal {
        kind: StrRaw(
            0,
        ),
        ),
        symbol: "R",
        suffix: None,
        span: /checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs:21:5: 21:9 (#0),
    Literal {
        kind: StrRaw(
            2,
        ),
        ),
        symbol: "R",
        suffix: None,
        span: /checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs:22:5: 22:13 (#0),
    Literal {
        kind: ByteStrRaw(
            0,
        ),
        ),
        symbol: "BR",
        suffix: None,
        span: /checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs:23:5: 23:11 (#0),
    Literal {
        kind: ByteStrRaw(
            2,
        ),
        ),
        symbol: "BR",
        suffix: None,
        span: /checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs:24:5: 24:15 (#0),
    Literal {
        kind: Char,
        symbol: "C",
        suffix: None,
        suffix: None,
        span: /checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs:25:5: 25:8 (#0),
    Literal {
        kind: Byte,
        symbol: "B",
        suffix: None,
        suffix: None,
        span: /checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs:26:5: 26:9 (#0),
    Literal {
        kind: Integer,
        symbol: "0",
        symbol: "0",
        suffix: Some("q"),
        span: /checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs:29:5: 29:7 (#0),
    Literal {
        kind: Float,
        symbol: "1.0",
        symbol: "1.0",
        suffix: Some("q"),
        span: /checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs:30:5: 30:9 (#0),
    Literal {
        kind: Str,
        symbol: "S",
        symbol: "S",
        suffix: Some("q"),
        span: /checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs:31:5: 31:9 (#0),
    Literal {
        kind: ByteStr,
        symbol: "B",
        symbol: "B",
        suffix: Some("q"),
        span: /checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs:32:5: 32:10 (#0),
    Literal {
        kind: StrRaw(
            0,
        ),
        ),
        symbol: "R",
        suffix: Some("q"),
        span: /checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs:33:5: 33:10 (#0),
    Literal {
        kind: StrRaw(
            2,
        ),
        ),
        symbol: "R",
        suffix: Some("q"),
        span: /checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs:34:5: 34:14 (#0),
    Literal {
        kind: ByteStrRaw(
            0,
        ),
        ),
        symbol: "BR",
        suffix: Some("q"),
        span: /checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs:35:5: 35:12 (#0),
    Literal {
        kind: ByteStrRaw(
            2,
        ),
        ),
        symbol: "BR",
        suffix: Some("q"),
        span: /checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs:36:5: 36:16 (#0),
    Literal {
        kind: Char,
        symbol: "C",
        symbol: "C",
        suffix: Some("q"),
        span: /checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs:37:5: 37:9 (#0),
    Literal {
        kind: Byte,
        symbol: "B",
        symbol: "B",
        suffix: Some("q"),
        span: /checkout/src/test/ui/proc-macro/debug/dump-debug-span-debug.rs:38:5: 38:10 (#0),
]

------------------------------------------



---- [ui] ui/proc-macro/debug/dump-debug.rs stdout ----
diff of stderr:

68         span: #0 bytes(337..341),
70     Literal {
70     Literal {
-         kind: StrRaw(0),
+         kind: StrRaw(
+         ),
72         symbol: "R",
73         suffix: None,
73         suffix: None,
74         span: #0 bytes(346..350),
75     },
76     Literal {
76     Literal {
-         kind: StrRaw(2),
+         kind: StrRaw(
+         ),
78         symbol: "R",
79         suffix: None,
79         suffix: None,
80         span: #0 bytes(355..363),
81     },
82     Literal {
82     Literal {
-         kind: ByteStrRaw(0),
+         kind: ByteStrRaw(
+         ),
84         symbol: "BR",
85         suffix: None,
85         suffix: None,
86         span: #0 bytes(368..374),
87     },
88     Literal {
88     Literal {
-         kind: ByteStrRaw(2),
+         kind: ByteStrRaw(
+         ),
90         symbol: "BR",
91         suffix: None,
91         suffix: None,
92         span: #0 bytes(379..389),

128         span: #0 bytes(462..467),
130     Literal {
130     Literal {
-         kind: StrRaw(0),
+         kind: StrRaw(
+         ),
132         symbol: "R",
132         symbol: "R",
133         suffix: Some("q"),
134         span: #0 bytes(472..477),
135     },
136     Literal {
136     Literal {
-         kind: StrRaw(2),
+         kind: StrRaw(
+         ),
138         symbol: "R",
138         symbol: "R",
139         suffix: Some("q"),
140         span: #0 bytes(482..491),
141     },
142     Literal {
142     Literal {
-         kind: ByteStrRaw(0),
+         kind: ByteStrRaw(
+         ),
144         symbol: "BR",
144         symbol: "BR",
145         suffix: Some("q"),
146         span: #0 bytes(496..503),
147     },
148     Literal {
148     Literal {
-         kind: ByteStrRaw(2),
+         kind: ByteStrRaw(
+         ),
150         symbol: "BR",
150         symbol: "BR",
151         suffix: Some("q"),
152         span: #0 bytes(508..519),

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/debug/dump-debug/dump-debug.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args proc-macro/debug/dump-debug.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/debug/dump-debug.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/debug/dump-debug/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/debug/dump-debug/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
TokenStream [Ident { ident: "ident", span: #0 bytes(130..135) }, Ident { ident: "r#ident", span: #0 bytes(151..158) }, Punct { ch: ',', spacing: Alone, span: #0 bytes(176..177) }, Punct { ch: '=', spacing: Joint, span: #0 bytes(203..205) }, Punct { ch: '=', spacing: Joint, span: #0 bytes(203..205) }, Punct { ch: '>', spacing: Alone, span: #0 bytes(205..206) }, Group { delimiter: Parenthesis, stream: TokenStream [], span: #0 bytes(230..232) }, Group { delimiter: Bracket, stream: TokenStream [Ident { ident: "_", span: #0 bytes(258..259) }], span: #0 bytes(257..260) }, Literal { kind: Integer, symbol: "0", suffix: None, span: #0 bytes(315..316) }, Literal { kind: Float, symbol: "1.0", suffix: None, span: #0 bytes(321..324) }, Literal { kind: Str, symbol: "S", suffix: None, span: #0 bytes(329..332) }, Literal { kind: ByteStr, symbol: "B", suffix: None, span: #0 bytes(337..341) }, Literal { kind: StrRaw(0), symbol: "R", suffix: None, span: #0 bytes(346..350) }, Literal { kind: StrRaw(2), symbol: "R", suffix: None, span: #0 bytes(355..363) }, Literal { kind: ByteStrRaw(0), symbol: "BR", suffix: None, span: #0 bytes(368..374) }, Literal { kind: ByteStrRaw(2), symbol: "BR", suffix: None, span: #0 bytes(379..389) }, Literal { kind: Char, symbol: "C", suffix: None, span: #0 bytes(394..397) }, Literal { kind: Byte, symbol: "B", suffix: None, span: #0 bytes(402..406) }, Literal { kind: Integer, symbol: "0", suffix: Some("q"), span: #0 bytes(437..439) }, Literal { kind: Float, symbol: "1.0", suffix: Some("q"), span: #0 bytes(444..448) }, Literal { kind: Str, symbol: "S", suffix: Some("q"), span: #0 bytes(453..457) }, Literal { kind: ByteStr, symbol: "B", suffix: Some("q"), span: #0 bytes(462..467) }, Literal { kind: StrRaw(0), symbol: "R", suffix: Some("q"), span: #0 bytes(472..477) }, Literal { kind: StrRaw(2), symbol: "R", suffix: Some("q"), span: #0 bytes(482..491) }, Literal { kind: ByteStrRaw(0), symbol: "BR", suffix: Some("q"), span: #0 bytes(496..503) }, Literal { kind: ByteStrRaw(2), symbol: "BR", suffix: Some("q"), span: #0 bytes(508..519) }, Literal { kind: Char, symbol: "C", suffix: Some("q"), span: #0 bytes(524..528) }, Literal { kind: Byte, symbol: "B", suffix: Some("q"), span: #0 bytes(533..538) }]
TokenStream [
    Ident {
        ident: "ident",
        span: #0 bytes(130..135),
    Ident {
    Ident {
        ident: "r#ident",
        span: #0 bytes(151..158),
    Punct {
    Punct {
        ch: ',',
        spacing: Alone,
        span: #0 bytes(176..177),
    Punct {
    Punct {
        ch: '=',
        spacing: Joint,
        span: #0 bytes(203..205),
    Punct {
    Punct {
        ch: '=',
        spacing: Joint,
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
        span: #0 bytes(203..205),
    Punct {
    Punct {
        ch: '>',
        spacing: Alone,
        span: #0 bytes(205..206),
    Group {
        delimiter: Parenthesis,
        delimiter: Parenthesis,
        stream: TokenStream [],
        span: #0 bytes(230..232),
    Group {
        delimiter: Bracket,
        delimiter: Bracket,
        stream: TokenStream [
            Ident {
                ident: "_",
                span: #0 bytes(258..259),
        ],
        ],
        span: #0 bytes(257..260),
    Literal {
        kind: Integer,
        symbol: "0",
        suffix: None,
        suffix: None,
        span: #0 bytes(315..316),
    Literal {
        kind: Float,
        symbol: "1.0",
        suffix: None,
        suffix: None,
        span: #0 bytes(321..324),
    Literal {
        kind: Str,
        symbol: "S",
        suffix: None,
        suffix: None,
        span: #0 bytes(329..332),
    Literal {
        kind: ByteStr,
        symbol: "B",
        suffix: None,
        suffix: None,
        span: #0 bytes(337..341),
    Literal {
        kind: StrRaw(
            0,
        ),
        ),
        symbol: "R",
        suffix: None,
        span: #0 bytes(346..350),
    Literal {
        kind: StrRaw(
            2,
        ),
        ),
        symbol: "R",
        suffix: None,
        span: #0 bytes(355..363),
    Literal {
        kind: ByteStrRaw(
            0,
        ),
        ),
        symbol: "BR",
        suffix: None,
        span: #0 bytes(368..374),
    Literal {
        kind: ByteStrRaw(
            2,
        ),
        ),
        symbol: "BR",
        suffix: None,
        span: #0 bytes(379..389),
    Literal {
        kind: Char,
        symbol: "C",
        suffix: None,
        suffix: None,
        span: #0 bytes(394..397),
    Literal {
        kind: Byte,
        symbol: "B",
        suffix: None,
        suffix: None,
        span: #0 bytes(402..406),
---

---- [ui] ui/proc-macro/issue-81007-item-attrs.rs stdout ----
diff of stdout:

18                 span: $DIR/issue-81007-item-attrs.rs:21:5: 21:22 (#0),
20             Literal {
20             Literal {
-                 kind: StrRaw(0),
+                 kind: StrRaw(
+                 ),
+                 ),
22                 symbol: " A doc comment",
23                 suffix: None,
24                 span: $DIR/issue-81007-item-attrs.rs:21:5: 21:22 (#0),

75                 span: $DIR/issue-81007-item-attrs.rs:27:5: 27:32 (#0),
77             Literal {
77             Literal {
-                 kind: StrRaw(0),
+                 kind: StrRaw(
+                 ),
+                 ),
79                 symbol: " Another comment comment",
80                 suffix: None,
81                 span: $DIR/issue-81007-item-attrs.rs:27:5: 27:32 (#0),

The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/issue-81007-item-attrs/issue-81007-item-attrs.stdout
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args proc-macro/issue-81007-item-attrs.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/issue-81007-item-attrs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/issue-81007-item-attrs" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-Z" "span-debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/issue-81007-item-attrs/auxiliary"
------------------------------------------
------------------------------------------
PRINT-ATTR INPUT (DISPLAY): #[doc = r" A doc comment"] struct Foo { }
PRINT-ATTR INPUT (DEBUG): TokenStream [
    Punct {
        ch: '#',
        spacing: Alone,
        span: /checkout/src/test/ui/proc-macro/issue-81007-item-attrs.rs:21:5: 21:22 (#0),
    Group {
        delimiter: Bracket,
        delimiter: Bracket,
        stream: TokenStream [
                ident: "doc",
                ident: "doc",
                span: /checkout/src/test/ui/proc-macro/issue-81007-item-attrs.rs:21:5: 21:22 (#0),
            Punct {
            Punct {
                ch: '=',
                spacing: Alone,
                span: /checkout/src/test/ui/proc-macro/issue-81007-item-attrs.rs:21:5: 21:22 (#0),
            Literal {
                kind: StrRaw(
                    0,
                ),
                ),
                symbol: " A doc comment",
                suffix: None,
                span: /checkout/src/test/ui/proc-macro/issue-81007-item-attrs.rs:21:5: 21:22 (#0),
        ],
        ],
        span: /checkout/src/test/ui/proc-macro/issue-81007-item-attrs.rs:21:5: 21:22 (#0),
    Ident {
        ident: "struct",
        ident: "struct",
        span: /checkout/src/test/ui/proc-macro/issue-81007-item-attrs.rs:22:5: 22:11 (#0),
    Ident {
        ident: "Foo",
        ident: "Foo",
        span: /checkout/src/test/ui/proc-macro/issue-81007-item-attrs.rs:22:12: 22:15 (#0),
    Group {
        delimiter: Brace,
        delimiter: Brace,
        stream: TokenStream [],
        span: /checkout/src/test/ui/proc-macro/issue-81007-item-attrs.rs:22:16: 22:18 (#0),
]
]
PRINT-ATTR INPUT (DISPLAY): #[rustc_dummy] #[doc = r" Another comment comment"] struct Bar { }
PRINT-ATTR INPUT (DEBUG): TokenStream [
    Punct {
        ch: '#',
        spacing: Alone,
        span: /checkout/src/test/ui/proc-macro/issue-81007-item-attrs.rs:26:5: 26:6 (#0),
    Group {
        delimiter: Bracket,
        delimiter: Bracket,
        stream: TokenStream [
            Ident {
                ident: "rustc_dummy",
                span: /checkout/src/test/ui/proc-macro/issue-81007-item-attrs.rs:26:7: 26:18 (#0),
        ],
        ],
        span: /checkout/src/test/ui/proc-macro/issue-81007-item-attrs.rs:26:6: 26:19 (#0),
    Punct {
    Punct {
        ch: '#',
        spacing: Alone,
        span: /checkout/src/test/ui/proc-macro/issue-81007-item-attrs.rs:27:5: 27:32 (#0),
    Group {
        delimiter: Bracket,
        delimiter: Bracket,
        stream: TokenStream [
                ident: "doc",
                ident: "doc",
                span: /checkout/src/test/ui/proc-macro/issue-81007-item-attrs.rs:27:5: 27:32 (#0),
            Punct {
            Punct {
                ch: '=',
                spacing: Alone,
                span: /checkout/src/test/ui/proc-macro/issue-81007-item-attrs.rs:27:5: 27:32 (#0),
            Literal {
                kind: StrRaw(
                    0,
                ),
                ),
                symbol: " Another comment comment",
                suffix: None,
                span: /checkout/src/test/ui/proc-macro/issue-81007-item-attrs.rs:27:5: 27:32 (#0),
        ],
        ],
        span: /checkout/src/test/ui/proc-macro/issue-81007-item-attrs.rs:27:5: 27:32 (#0),
    Ident {
        ident: "struct",
        ident: "struct",
        span: /checkout/src/test/ui/proc-macro/issue-81007-item-attrs.rs:28:5: 28:11 (#0),
    Ident {
    Ident {
        ident: "Bar",
        span: /checkout/src/test/ui/proc-macro/issue-81007-item-attrs.rs:28:12: 28:15 (#0),
    Group {
        delimiter: Brace,
        delimiter: Brace,
        stream: TokenStream [],
        span: /checkout/src/test/ui/proc-macro/issue-81007-item-attrs.rs:28:16: 28:18 (#0),
]

------------------------------------------
stderr:
---
test result: FAILED. 11936 passed; 5 failed; 97 ignored; 0 measured; 0 filtered out; finished in 124.63s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:26
