plain
.................................................................................................... 8200/11248
................................................................................................i... 8300/11248
.................................................................................................... 8400/11248
.................................................................................................... 8500/11248
........F...........................................iiii.iiii......................F.............FF. 8600/11248
..............i..F...........iF..................................................................... 8700/11248
.................................................................................................... 8900/11248
.................................................................................................... 9000/11248
.................................................................................................... 9100/11248
.................................................................................................... 9200/11248
.................................................................................................... 9200/11248
............................................i.......i............................................... 9300/11248
...................................................................................iiiiii..iiiiii.i. 9400/11248
.................................................................................................... 9600/11248
.................................................................................................... 9700/11248
.................................................................................................... 9800/11248
.................................................................................................... 9900/11248
---
.......................................i............................................................ 11200/11248
................................................
failures:

---- [ui] ui/proc-macro/attr-complex-fn.rs stdout ----


- PRINT-ATTR INPUT (DISPLAY): fn foo < T : MyTrait < MyStruct > > () { }
+ PRINT-ATTR INPUT (DISPLAY): fn foo < T : MyTrait < MyStruct >> () { }
2 PRINT-ATTR INPUT (DEBUG): TokenStream [
3     Ident {
4         ident: "fn",
37     },
38     Punct {
38     Punct {
39         ch: '>',
-         spacing: Alone,
-         span: $DIR/attr-complex-fn.rs:12:27: 12:28 (#0),
+         spacing: Joint,
+         span: $DIR/attr-complex-fn.rs:12:27: 12:29 (#0),
43     Punct {
43     Punct {
44         ch: '>',
45         spacing: Alone,
45         spacing: Alone,
-         span: $DIR/attr-complex-fn.rs:12:27: 12:28 (#0),
+         span: $DIR/attr-complex-fn.rs:12:27: 12:29 (#0),
48     Group {
49         delimiter: Parenthesis,



The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/attr-complex-fn/attr-complex-fn.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args proc-macro/attr-complex-fn.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/attr-complex-fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/attr-complex-fn" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "span-debug" "--error-format" "human" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/attr-complex-fn/auxiliary"
------------------------------------------
------------------------------------------
PRINT-ATTR INPUT (DISPLAY): fn foo < T : MyTrait < MyStruct >> () { }
PRINT-ATTR INPUT (DEBUG): TokenStream [
    Ident {
        ident: "fn",
        span: /checkout/src/test/ui/proc-macro/attr-complex-fn.rs:12:1: 12:3 (#0),
    Ident {
    Ident {
        ident: "foo",
        span: /checkout/src/test/ui/proc-macro/attr-complex-fn.rs:12:4: 12:7 (#0),
    Punct {
    Punct {
        ch: '<',
        spacing: Alone,
        span: /checkout/src/test/ui/proc-macro/attr-complex-fn.rs:12:7: 12:8 (#0),
    Ident {
        ident: "T",
        ident: "T",
        span: /checkout/src/test/ui/proc-macro/attr-complex-fn.rs:12:8: 12:9 (#0),
    Punct {
        ch: ':',
        spacing: Alone,
        spacing: Alone,
        span: /checkout/src/test/ui/proc-macro/attr-complex-fn.rs:12:9: 12:10 (#0),
    Ident {
    Ident {
        ident: "MyTrait",
        span: /checkout/src/test/ui/proc-macro/attr-complex-fn.rs:12:11: 12:18 (#0),
    Punct {
    Punct {
        ch: '<',
        spacing: Alone,
        span: /checkout/src/test/ui/proc-macro/attr-complex-fn.rs:12:18: 12:19 (#0),
    Ident {
    Ident {
        ident: "MyStruct",
        span: /checkout/src/test/ui/proc-macro/attr-complex-fn.rs:12:19: 12:27 (#0),
    Punct {
    Punct {
        ch: '>',
        spacing: Joint,
        span: /checkout/src/test/ui/proc-macro/attr-complex-fn.rs:12:27: 12:29 (#0),
    Punct {
    Punct {
        ch: '>',
        spacing: Alone,
        span: /checkout/src/test/ui/proc-macro/attr-complex-fn.rs:12:27: 12:29 (#0),
    Group {
        delimiter: Parenthesis,
        delimiter: Parenthesis,
        stream: TokenStream [],
        span: /checkout/src/test/ui/proc-macro/attr-complex-fn.rs:12:29: 12:31 (#0),
    Group {
        delimiter: Brace,
        delimiter: Brace,
        stream: TokenStream [],
        span: /checkout/src/test/ui/proc-macro/attr-complex-fn.rs:12:32: 12:34 (#0),
]

------------------------------------------
stderr:
stderr:
------------------------------------------

------------------------------------------


---- [ui] ui/proc-macro/inner-attrs.rs stdout ----

error: test compilation failed although it shouldn't!
failed to decode compiler output as json: line: { # ! [print_target_and_args(third)] # ! [print_target_and_args(fourth)] }
output: PRINT-ATTR_ARGS INPUT (DISPLAY): first
PRINT-ATTR_ARGS INPUT (DEBUG): TokenStream [
    Ident {
        ident: "first",
        span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:10:25: 10:30 (#0),
]
]
PRINT-ATTR INPUT (DISPLAY): #[print_target_and_args(second)] fn foo()
{ # ! [print_target_and_args(third)] # ! [print_target_and_args(fourth)] }
PRINT-ATTR INPUT (DEBUG): TokenStream [
    Punct {
        ch: '#',
        spacing: Alone,
        span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:11:1: 11:2 (#0),
    Group {
        delimiter: Bracket,
        delimiter: Bracket,
        stream: TokenStream [
            Ident {
                ident: "print_target_and_args",
                span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:11:3: 11:24 (#0),
            Group {
                delimiter: Parenthesis,
                delimiter: Parenthesis,
                stream: TokenStream [
                    Ident {
                        ident: "second",
                        span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:11:25: 11:31 (#0),
                ],
                ],
                span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:11:24: 11:32 (#0),
        ],
        ],
        span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:11:2: 11:33 (#0),
    Ident {
    Ident {
        ident: "fn",
        span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:12:1: 12:3 (#0),
    Ident {
    Ident {
        ident: "foo",
        span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:12:4: 12:7 (#0),
    Group {
        delimiter: Parenthesis,
        delimiter: Parenthesis,
        stream: TokenStream [],
        span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:12:7: 12:9 (#0),
    Group {
        delimiter: Brace,
        delimiter: Brace,
        stream: TokenStream [
            Punct {
                ch: '#',
                spacing: Joint,
                span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:13:5: 13:6 (#0),
            Punct {
            Punct {
                ch: '!',
                spacing: Alone,
                span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:13:6: 13:7 (#0),
            Group {
                delimiter: Bracket,
                delimiter: Bracket,
                stream: TokenStream [
                    Ident {
                        ident: "print_target_and_args",
                        span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:13:8: 13:29 (#0),
                    Group {
                        delimiter: Parenthesis,
                        delimiter: Parenthesis,
                        stream: TokenStream [
                            Ident {
                                ident: "third",
                                span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:13:30: 13:35 (#0),
                        ],
                        ],
                        span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:13:29: 13:36 (#0),
                ],
                ],
                span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:13:7: 13:37 (#0),
            Punct {
            Punct {
                ch: '#',
                spacing: Joint,
                span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:14:5: 14:6 (#0),
            Punct {
            Punct {
                ch: '!',
                spacing: Alone,
                span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:14:6: 14:7 (#0),
            Group {
                delimiter: Bracket,
                delimiter: Bracket,
                stream: TokenStream [
                    Ident {
                        ident: "print_target_and_args",
                        span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:14:8: 14:29 (#0),
                    Group {
                        delimiter: Parenthesis,
                        delimiter: Parenthesis,
                        stream: TokenStream [
                            Ident {
                                ident: "fourth",
                                span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:14:30: 14:36 (#0),
                        ],
                        ],
                        span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:14:29: 14:37 (#0),
                ],
                ],
                span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:14:7: 14:38 (#0),
        ],
        ],
        span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:12:10: 15:2 (#0),
]
]
PRINT-ATTR_ARGS INPUT (DISPLAY): second
PRINT-ATTR_ARGS INPUT (DEBUG): TokenStream [
    Ident {
        ident: "second",
        span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:11:25: 11:31 (#0),
]
]
PRINT-ATTR INPUT (DISPLAY): fn foo()
{ # ! [print_target_and_args(third)] # ! [print_target_and_args(fourth)] }
PRINT-ATTR INPUT (DEBUG): TokenStream [
    Ident {
        ident: "fn",
        span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:12:1: 12:3 (#0),
    Ident {
    Ident {
        ident: "foo",
        span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:12:4: 12:7 (#0),
    Group {
        delimiter: Parenthesis,
        delimiter: Parenthesis,
        stream: TokenStream [],
        span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:12:7: 12:9 (#0),
    Group {
        delimiter: Brace,
        delimiter: Brace,
        stream: TokenStream [
            Punct {
                ch: '#',
                spacing: Joint,
                span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:13:5: 13:6 (#0),
            Punct {
            Punct {
                ch: '!',
                spacing: Alone,
                span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:13:6: 13:7 (#0),
            Group {
                delimiter: Bracket,
                delimiter: Bracket,
                stream: TokenStream [
                    Ident {
                        ident: "print_target_and_args",
                        span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:13:8: 13:29 (#0),
                    Group {
                        delimiter: Parenthesis,
                        delimiter: Parenthesis,
                        stream: TokenStream [
                            Ident {
                                ident: "third",
                                span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:13:30: 13:35 (#0),
                        ],
                        ],
                        span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:13:29: 13:36 (#0),
                ],
                ],
                span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:13:7: 13:37 (#0),
            Punct {
            Punct {
                ch: '#',
                spacing: Joint,
                span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:14:5: 14:6 (#0),
            Punct {
            Punct {
                ch: '!',
                spacing: Alone,
                span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:14:6: 14:7 (#0),
            Group {
                delimiter: Bracket,
                delimiter: Bracket,
                stream: TokenStream [
                    Ident {
                        ident: "print_target_and_args",
                        span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:14:8: 14:29 (#0),
                    Group {
                        delimiter: Parenthesis,
                        delimiter: Parenthesis,
                        stream: TokenStream [
                            Ident {
                                ident: "fourth",
                                span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:14:30: 14:36 (#0),
                        ],
                        ],
                        span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:14:29: 14:37 (#0),
                ],
                ],
                span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:14:7: 14:38 (#0),
        ],
        ],
        span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:12:10: 15:2 (#0),
]
]
PRINT-ATTR_ARGS INPUT (DISPLAY): third
PRINT-ATTR_ARGS INPUT (DEBUG): TokenStream [
    Ident {
        ident: "third",
        span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:13:30: 13:35 (#0),
]
]
PRINT-ATTR INPUT (DISPLAY): fn foo()
{ # ! [print_target_and_args(fourth)] # ! [print_target_and_args(third)] }
PRINT-ATTR INPUT (DEBUG): TokenStream [
    Ident {
        ident: "fn",
        span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:12:1: 12:3 (#0),
    Ident {
    Ident {
        ident: "foo",
        span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:12:4: 12:7 (#0),
    Group {
        delimiter: Parenthesis,
        delimiter: Parenthesis,
        stream: TokenStream [],
        span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:12:7: 12:9 (#0),
    Group {
        delimiter: Brace,
        delimiter: Brace,
        stream: TokenStream [
            Punct {
                ch: '#',
                spacing: Joint,
                span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:14:5: 14:6 (#0),
            Punct {
            Punct {
                ch: '!',
                spacing: Alone,
                span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:14:6: 14:7 (#0),
            Group {
                delimiter: Bracket,
                delimiter: Bracket,
                stream: TokenStream [
                    Ident {
                        ident: "print_target_and_args",
                        span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:14:8: 14:29 (#0),
                    Group {
                        delimiter: Parenthesis,
                        delimiter: Parenthesis,
                        stream: TokenStream [
                            Ident {
                                ident: "fourth",
                                span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:14:30: 14:36 (#0),
                        ],
                        ],
                        span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:14:29: 14:37 (#0),
                ],
                ],
                span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:14:7: 14:38 (#0),
            Punct {
            Punct {
                ch: '#',
                spacing: Joint,
                span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:13:5: 13:6 (#0),
            Punct {
            Punct {
                ch: '!',
                spacing: Alone,
                span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:13:6: 13:7 (#0),
            Group {
                delimiter: Bracket,
                delimiter: Bracket,
                stream: TokenStream [
                    Ident {
                        ident: "print_target_and_args",
                        span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:13:8: 13:29 (#0),
                    Group {
                        delimiter: Parenthesis,
                        delimiter: Parenthesis,
                        stream: TokenStream [
                            Ident {
                                ident: "third",
                                span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:13:30: 13:35 (#0),
                        ],
                        ],
                        span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:13:29: 13:36 (#0),
                ],
                ],
                span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:13:7: 13:37 (#0),
        ],
        ],
        span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:12:10: 15:2 (#0),
]
]
PRINT-ATTR_ARGS INPUT (DISPLAY): fourth
PRINT-ATTR_ARGS INPUT (DEBUG): TokenStream [
    Ident {
        ident: "fourth",
        span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:14:30: 14:36 (#0),
]
]
PRINT-ATTR INPUT (DISPLAY): fn foo()
{ # ! [print_target_and_args(third)] # ! [print_target_and_args(fourth)] }
PRINT-ATTR INPUT (DEBUG): TokenStream [
    Ident {
        ident: "fn",
        span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:12:1: 12:3 (#0),
    Ident {
    Ident {
        ident: "foo",
        span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:12:4: 12:7 (#0),
    Group {
        delimiter: Parenthesis,
        delimiter: Parenthesis,
        stream: TokenStream [],
        span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:12:7: 12:9 (#0),
    Group {
        delimiter: Brace,
        delimiter: Brace,
        stream: TokenStream [
            Punct {
                ch: '#',
                spacing: Joint,
                span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:13:5: 13:6 (#0),
            Punct {
            Punct {
                ch: '!',
                spacing: Alone,
                span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:13:6: 13:7 (#0),
            Group {
                delimiter: Bracket,
                delimiter: Bracket,
                stream: TokenStream [
                    Ident {
                        ident: "print_target_and_args",
                        span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:13:8: 13:29 (#0),
                    Group {
                        delimiter: Parenthesis,
                        delimiter: Parenthesis,
                        stream: TokenStream [
                            Ident {
                                ident: "third",
                                span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:13:30: 13:35 (#0),
                        ],
                        ],
                        span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:13:29: 13:36 (#0),
                ],
                ],
                span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:13:7: 13:37 (#0),
            Punct {
            Punct {
                ch: '#',
                spacing: Joint,
                span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:14:5: 14:6 (#0),
            Punct {
            Punct {
                ch: '!',
                spacing: Alone,
                span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:14:6: 14:7 (#0),
            Group {
                delimiter: Bracket,
                delimiter: Bracket,
                stream: TokenStream [
                    Ident {
                        ident: "print_target_and_args",
                        span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:14:8: 14:29 (#0),
                    Group {
                        delimiter: Parenthesis,
                        delimiter: Parenthesis,
                        stream: TokenStream [
                            Ident {
                                ident: "fourth",
                                span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:14:30: 14:36 (#0),
                        ],
                        ],
                        span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:14:29: 14:37 (#0),
                ],
                ],
                span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:14:7: 14:38 (#0),
        ],
        ],
        span: /checkout/src/test/ui/proc-macro/inner-attrs.rs:12:10: 15:2 (#0),
]
]
PRINT-ATTR_ARGS INPUT (DISPLAY): third
PRINT-ATTR_ARGS INPUT (DEBUG): TokenStream [
    Ident {
---

---- [ui] ui/proc-macro/issue-75930-derive-cfg.rs stdout ----
diff of stdout:

1194         span: $DIR/issue-75930-derive-cfg.rs:20:32: 63:2 (#0),
1196 ]
1196 ]
- PRINT-DERIVE INPUT (DISPLAY): #[allow(dead_code)] #[print_helper(b)] #[print_helper(a)] struct Foo < B >
+ PRINT-DERIVE INPUT (DISPLAY): #[allow(dead_code)] #[print_helper(b)] #[print_helper(a)] struct Foo <
+ #[cfg(FALSE)] A, B >
1198 {
-     second : bool, third :
+     #[cfg(FALSE)] first : String, #[cfg_attr(FALSE, deny(warnings))] second :
+     bool, third :
1200     [u8 ;
1201      {
-          #[cfg(not(FALSE))] struct Inner ; match true
-          { #[allow(warnings)] false => { }, _ => { } } ; #[print_helper(c)]
-          #[cfg(not(FALSE))] fn kept_fn()
-          { # ! [cfg(not(FALSE))] let my_val = true ; } enum TupleEnum
-          { Foo(#[cfg(not(FALSE))] i32, u8) } struct
-          TupleStruct(#[cfg(not(FALSE))] i32, u8) ; 0
+          #[cfg(FALSE)] struct Bar ; #[cfg(not(FALSE))] struct Inner ;
+          #[cfg(FALSE)] let a = 25 ; match true
+          {
+              #[cfg(FALSE)] true => { },
+              #[cfg_attr(not(FALSE), allow(warnings))] false => { }, _ => { }
+          } ; #[print_helper(should_be_removed)] fn removed_fn()
+          { # ! [cfg(FALSE)] } #[print_helper(c)] #[cfg(not(FALSE))] fn
+          kept_fn() { # ! [cfg(not(FALSE))] let my_val = true ; } enum
+          TupleEnum
+          {
+              Foo(#[cfg(FALSE)] u8, #[cfg(FALSE)] bool, #[cfg(not(FALSE))] i32,
+                  #[cfg(FALSE)] String, u8)
+          } struct
+          TupleStruct(#[cfg(FALSE)] String, #[cfg(not(FALSE))] i32,
+                      #[cfg(FALSE)] bool, u8) ; 0
1208      }], #[print_helper(d)] fourth : B
1209 }
1210 PRINT-DERIVE INPUT (DEBUG): TokenStream [
1296         spacing: Joint,
1296         spacing: Joint,
1297         span: $DIR/issue-75930-derive-cfg.rs:20:11: 20:12 (#0),
+     Punct {
+     Punct {
+         ch: '#',
+         spacing: Alone,
+         span: $DIR/issue-75930-derive-cfg.rs:20:12: 20:13 (#0),
+     Group {
+         delimiter: Bracket,
+         delimiter: Bracket,
+         stream: TokenStream [
+             Ident {
+                 ident: "cfg",
+                 span: $DIR/issue-75930-derive-cfg.rs:20:14: 20:17 (#0),
+             Group {
+                 delimiter: Parenthesis,
+                 delimiter: Parenthesis,
+                 stream: TokenStream [
+                     Ident {
+                         ident: "FALSE",
+                         span: $DIR/issue-75930-derive-cfg.rs:20:18: 20:23 (#0),
+                 ],
+                 ],
+                 span: $DIR/issue-75930-derive-cfg.rs:20:17: 20:24 (#0),
+         ],
+         ],
+         span: $DIR/issue-75930-derive-cfg.rs:20:13: 20:25 (#0),
1299     Ident {
+         ident: "A",
+         ident: "A",
+         span: $DIR/issue-75930-derive-cfg.rs:20:26: 20:27 (#0),
+     Punct {
+     Punct {
+         ch: ',',
+         spacing: Alone,
+         span: $DIR/issue-75930-derive-cfg.rs:20:27: 20:28 (#0),
+     Ident {
+     Ident {
1300         ident: "B",
1301         span: $DIR/issue-75930-derive-cfg.rs:20:29: 20:30 (#0),

1308     Group {
1309         delimiter: Brace,
1309         delimiter: Brace,
1310         stream: TokenStream [
+             Punct {
+                 ch: '#',
+                 spacing: Alone,
+                 span: $DIR/issue-75930-derive-cfg.rs:21:5: 21:6 (#0),
+             Group {
+                 delimiter: Bracket,
+                 delimiter: Bracket,
+                 stream: TokenStream [
+                     Ident {
+                         ident: "cfg",
+                         span: $DIR/issue-75930-derive-cfg.rs:21:7: 21:10 (#0),
+                     Group {
+                         delimiter: Parenthesis,
+                         delimiter: Parenthesis,
+                         stream: TokenStream [
+                             Ident {
+                                 ident: "FALSE",
+                                 span: $DIR/issue-75930-derive-cfg.rs:21:11: 21:16 (#0),
+                         ],
+                         ],
+                         span: $DIR/issue-75930-derive-cfg.rs:21:10: 21:17 (#0),
+                 ],
+                 ],
+                 span: $DIR/issue-75930-derive-cfg.rs:21:6: 21:18 (#0),
1311             Ident {
1311             Ident {
+                 ident: "first",
+                 span: $DIR/issue-75930-derive-cfg.rs:21:19: 21:24 (#0),
+             Punct {
+                 ch: ':',
+                 spacing: Alone,
+                 spacing: Alone,
+                 span: $DIR/issue-75930-derive-cfg.rs:21:24: 21:25 (#0),
+             Ident {
+                 ident: "String",
+                 ident: "String",
+                 span: $DIR/issue-75930-derive-cfg.rs:21:26: 21:32 (#0),
+             Punct {
+             Punct {
+                 ch: ',',
+                 spacing: Alone,
+                 span: $DIR/issue-75930-derive-cfg.rs:21:32: 21:33 (#0),
+             Punct {
+             Punct {
+                 ch: '#',
+                 spacing: Alone,
+                 span: $DIR/issue-75930-derive-cfg.rs:22:5: 22:6 (#0),
+             Group {
+                 delimiter: Bracket,
+                 delimiter: Bracket,
+                 stream: TokenStream [
+                     Ident {
+                         ident: "cfg_attr",
+                         span: $DIR/issue-75930-derive-cfg.rs:22:7: 22:15 (#0),
+                     Group {
+                         delimiter: Parenthesis,
+                         delimiter: Parenthesis,
+                         stream: TokenStream [
+                             Ident {
+                                 ident: "FALSE",
+                                 span: $DIR/issue-75930-derive-cfg.rs:22:16: 22:21 (#0),
+                             Punct {
+                             Punct {
+                                 ch: ',',
+                                 spacing: Alone,
+                                 span: $DIR/issue-75930-derive-cfg.rs:22:21: 22:22 (#0),
+                             Ident {
+                             Ident {
+                                 ident: "deny",
+                                 span: $DIR/issue-75930-derive-cfg.rs:22:23: 22:27 (#0),
+                             Group {
+                                 delimiter: Parenthesis,
+                                 delimiter: Parenthesis,
+                                 stream: TokenStream [
+                                     Ident {
+                                         ident: "warnings",
+                                         span: $DIR/issue-75930-derive-cfg.rs:22:28: 22:36 (#0),
+                                 ],
+                                 ],
+                                 span: $DIR/issue-75930-derive-cfg.rs:22:27: 22:37 (#0),
+                         ],
+                         ],
+                         span: $DIR/issue-75930-derive-cfg.rs:22:15: 22:38 (#0),
+                 ],
+                 ],
+                 span: $DIR/issue-75930-derive-cfg.rs:22:6: 22:39 (#0),
+             Ident {
+             Ident {
1312                 ident: "second",
1313                 span: $DIR/issue-75930-derive-cfg.rs:22:40: 22:46 (#0),

1353                             Punct {
1353                             Punct {
1354                                 ch: '#',
1355                                 spacing: Alone,
+                                 span: $DIR/issue-75930-derive-cfg.rs:24:9: 24:10 (#0),
+                             Group {
+                                 delimiter: Bracket,
+                                 delimiter: Bracket,
+                                 stream: TokenStream [
+                                     Ident {
+                                         ident: "cfg",
+                                         span: $DIR/issue-75930-derive-cfg.rs:24:11: 24:14 (#0),
+                                     Group {
+                                         delimiter: Parenthesis,
+                                         delimiter: Parenthesis,
+                                         stream: TokenStream [
+                                             Ident {
+                                                 ident: "FALSE",
+                                                 span: $DIR/issue-75930-derive-cfg.rs:24:15: 24:20 (#0),
+                                         ],
+                                         ],
+                                         span: $DIR/issue-75930-derive-cfg.rs:24:14: 24:21 (#0),
+                                 ],
+                                 ],
+                                 span: $DIR/issue-75930-derive-cfg.rs:24:10: 24:22 (#0),
+                             Ident {
+                                 ident: "struct",
+                                 ident: "struct",
+                                 span: $DIR/issue-75930-derive-cfg.rs:24:23: 24:29 (#0),
+                             Ident {
+                             Ident {
+                                 ident: "Bar",
+                                 span: $DIR/issue-75930-derive-cfg.rs:24:30: 24:33 (#0),
+                             Punct {
+                             Punct {
+                                 ch: ';',
+                                 spacing: Alone,
+                                 span: $DIR/issue-75930-derive-cfg.rs:24:33: 24:34 (#0),
+                             Punct {
+                             Punct {
+                                 ch: '#',
+                                 spacing: Alone,
1356                                 span: $DIR/issue-75930-derive-cfg.rs:25:9: 25:10 (#0),
1358                             Group {

1398                                 spacing: Alone,
1398                                 spacing: Alone,
1399                                 span: $DIR/issue-75930-derive-cfg.rs:25:40: 25:41 (#0),
+                             Punct {
+                             Punct {
+                                 ch: '#',
+                                 spacing: Alone,
+                                 span: $DIR/issue-75930-derive-cfg.rs:26:9: 26:10 (#0),
+                             Group {
+                                 delimiter: Bracket,
+                                 delimiter: Bracket,
+                                 stream: TokenStream [
+                                     Ident {
+                                         ident: "cfg",
+                                         span: $DIR/issue-75930-derive-cfg.rs:26:11: 26:14 (#0),
+                                     Group {
+                                         delimiter: Parenthesis,
+                                         delimiter: Parenthesis,
+                                         stream: TokenStream [
+                                             Ident {
+                                                 ident: "FALSE",
+                                                 span: $DIR/issue-75930-derive-cfg.rs:26:15: 26:20 (#0),
+                                         ],
+                                         ],
+                                         span: $DIR/issue-75930-derive-cfg.rs:26:14: 26:21 (#0),
+                                 ],
+                                 ],
+                                 span: $DIR/issue-75930-derive-cfg.rs:26:10: 26:22 (#0),
1401                             Ident {
1401                             Ident {
+                                 ident: "let",
+                                 span: $DIR/issue-75930-derive-cfg.rs:26:23: 26:26 (#0),
+                             Ident {
+                                 ident: "a",
+                                 ident: "a",
+                                 span: $DIR/issue-75930-derive-cfg.rs:26:27: 26:28 (#0),
+                             Punct {
+                             Punct {
+                                 ch: '=',
+                                 spacing: Alone,
+                                 span: $DIR/issue-75930-derive-cfg.rs:26:29: 26:30 (#0),
+                             Literal {
+                                 kind: Integer,
+                                 symbol: "25",
+                                 suffix: None,
+                                 suffix: None,
+                                 span: $DIR/issue-75930-derive-cfg.rs:26:31: 26:33 (#0),
+                             Punct {
+                             Punct {
+                                 ch: ';',
+                                 spacing: Alone,
+                                 span: $DIR/issue-75930-derive-cfg.rs:26:33: 26:34 (#0),
+                             Ident {
1402                                 ident: "match",
1402                                 ident: "match",
1403                                 span: $DIR/issue-75930-derive-cfg.rs:27:9: 27:14 (#0),

1412                                     Punct {
1412                                     Punct {
1413                                         ch: '#',
1414                                         spacing: Alone,
-                                         span: $DIR/issue-75930-derive-cfg.rs:29:13: 29:14 (#0),
+                                         span: $DIR/issue-75930-derive-cfg.rs:28:13: 28:14 (#0),
1417                                     Group {
1418                                         delimiter: Bracket,


1419                                         stream: TokenStream [
-                                                 ident: "allow",
-                                                 ident: "allow",
-                                                 span: $DIR/issue-75930-derive-cfg.rs:29:36: 29:41 (#0),
+                                                 ident: "cfg",
+                                                 span: $DIR/issue-75930-derive-cfg.rs:28:15: 28:18 (#0),
1424                                             Group {
1425                                                 delimiter: Parenthesis,


1426                                                 stream: TokenStream [
1427                                                     Ident {
-                                                         ident: "warnings",
-                                                         span: $DIR/issue-75930-derive-cfg.rs:29:42: 29:50 (#0),
+                                                         ident: "FALSE",
+                                                         span: $DIR/issue-75930-derive-cfg.rs:28:19: 28:24 (#0),
1431                                                 ],
1431                                                 ],
-                                                 span: $DIR/issue-75930-derive-cfg.rs:29:41: 29:51 (#0),
+                                                 span: $DIR/issue-75930-derive-cfg.rs:28:18: 28:25 (#0),
1434                                         ],
1434                                         ],
+                                         span: $DIR/issue-75930-derive-cfg.rs:28:14: 28:26 (#0),
+                                     Ident {
+                                     Ident {
+                                         ident: "true",
+                                         span: $DIR/issue-75930-derive-cfg.rs:28:27: 28:31 (#0),
+                                     Punct {
+                                     Punct {
+                                         ch: '=',
+                                         spacing: Joint,
+                                         span: $DIR/issue-75930-derive-cfg.rs:28:32: 28:34 (#0),
+                                     Punct {
+                                     Punct {
+                                         ch: '>',
+                                         spacing: Alone,
+                                         span: $DIR/issue-75930-derive-cfg.rs:28:32: 28:34 (#0),
+                                     Group {
+                                         delimiter: Brace,
+                                         delimiter: Brace,
+                                         stream: TokenStream [],
+                                         span: $DIR/issue-75930-derive-cfg.rs:28:35: 28:37 (#0),
+                                     Punct {
+                                     Punct {
+                                         ch: ',',
+                                         spacing: Alone,
+                                         span: $DIR/issue-75930-derive-cfg.rs:28:37: 28:38 (#0),
+                                     Punct {
+                                     Punct {
+                                         ch: '#',
+                                         spacing: Alone,
1435                                         span: $DIR/issue-75930-derive-cfg.rs:29:13: 29:14 (#0),
+                                     Group {
+                                         delimiter: Bracket,
+                                         delimiter: Bracket,
+                                         stream: TokenStream [
+                                             Ident {
+                                                 ident: "cfg_attr",
+                                                 span: $DIR/issue-75930-derive-cfg.rs:29:15: 29:23 (#0),
+                                             Group {
+                                                 delimiter: Parenthesis,
+                                                 delimiter: Parenthesis,
+                                                 stream: TokenStream [
+                                                         ident: "not",
+                                                         ident: "not",
+                                                         span: $DIR/issue-75930-derive-cfg.rs:29:24: 29:27 (#0),
+                                                     Group {
+                                                         delimiter: Parenthesis,
+                                                         delimiter: Parenthesis,
+                                                         stream: TokenStream [
+                                                             Ident {
+                                                                 ident: "FALSE",
+                                                                 span: $DIR/issue-75930-derive-cfg.rs:29:28: 29:33 (#0),
+                                                         ],
+                                                         ],
+                                                         span: $DIR/issue-75930-derive-cfg.rs:29:27: 29:34 (#0),
+                                                     Punct {
+                                                     Punct {
+                                                         ch: ',',
+                                                         spacing: Alone,
+                                                         span: $DIR/issue-75930-derive-cfg.rs:29:34: 29:35 (#0),
+                                                     Ident {
+                                                         ident: "allow",
+                                                         ident: "allow",
+                                                         span: $DIR/issue-75930-derive-cfg.rs:29:36: 29:41 (#0),
+                                                     Group {
+                                                         delimiter: Parenthesis,
+                                                         delimiter: Parenthesis,
+                                                         stream: TokenStream [
+                                                             Ident {
+                                                                 ident: "warnings",
+                                                                 span: $DIR/issue-75930-derive-cfg.rs:29:42: 29:50 (#0),
+                                                         ],
+                                                         ],
+                                                         span: $DIR/issue-75930-derive-cfg.rs:29:41: 29:51 (#0),
+                                                 ],
+                                                 ],
+                                                 span: $DIR/issue-75930-derive-cfg.rs:29:23: 29:52 (#0),
+                                         ],
+                                         ],
+                                         span: $DIR/issue-75930-derive-cfg.rs:29:14: 29:53 (#0),
1437                                     Ident {
1437                                     Ident {
1438                                         ident: "false",
1439                                         span: $DIR/issue-75930-derive-cfg.rs:29:54: 29:59 (#0),
1488                             Punct {
1488                             Punct {
1489                                 ch: '#',
1490                                 spacing: Alone,
+                                 span: $DIR/issue-75930-derive-cfg.rs:33:9: 33:10 (#0),
+                             Group {
+                                 delimiter: Bracket,
+                                 delimiter: Bracket,
+                                 stream: TokenStream [
+                                     Ident {
+                                         ident: "print_helper",
+                                         span: $DIR/issue-75930-derive-cfg.rs:33:11: 33:23 (#0),
+                                     Group {
+                                         delimiter: Parenthesis,
+                                         delimiter: Parenthesis,
+                                         stream: TokenStream [
+                                             Ident {
+                                                 ident: "should_be_removed",
+                                                 span: $DIR/issue-75930-derive-cfg.rs:33:24: 33:41 (#0),
+                                         ],
+                                         ],
+                                         span: $DIR/issue-75930-derive-cfg.rs:33:23: 33:42 (#0),
+                                 ],
+                                 ],
+                                 span: $DIR/issue-75930-derive-cfg.rs:33:10: 33:43 (#0),
+                             Ident {
+                             Ident {
+                                 ident: "fn",
+                                 span: $DIR/issue-75930-derive-cfg.rs:34:9: 34:11 (#0),
+                             Ident {
+                                 ident: "removed_fn",
+                                 ident: "removed_fn",
+                                 span: $DIR/issue-75930-derive-cfg.rs:34:12: 34:22 (#0),
+                             Group {
+                                 delimiter: Parenthesis,
+                                 delimiter: Parenthesis,
+                                 stream: TokenStream [],
+                                 span: $DIR/issue-75930-derive-cfg.rs:34:22: 34:24 (#0),
+                             Group {
+                                 delimiter: Brace,
+                                 delimiter: Brace,
+                                 stream: TokenStream [
+                                     Punct {
+                                         ch: '#',
+                                         spacing: Joint,
+                                         span: $DIR/issue-75930-derive-cfg.rs:35:13: 35:14 (#0),
+                                     Punct {
+                                     Punct {
+                                         ch: '!',
+                                         spacing: Alone,
+                                         span: $DIR/issue-75930-derive-cfg.rs:35:14: 35:15 (#0),
+                                     Group {
+                                         delimiter: Bracket,
+                                         delimiter: Bracket,
+                                         stream: TokenStream [
+                                             Ident {
+                                                 ident: "cfg",
+                                                 span: $DIR/issue-75930-derive-cfg.rs:35:16: 35:19 (#0),
+                                             Group {
+                                                 delimiter: Parenthesis,
+                                                 delimiter: Parenthesis,
+                                                 stream: TokenStream [
+                                                     Ident {
+                                                         ident: "FALSE",
+                                                         span: $DIR/issue-75930-derive-cfg.rs:35:20: 35:25 (#0),
+                                                 ],
+                                                 ],
+                                                 span: $DIR/issue-75930-derive-cfg.rs:35:19: 35:26 (#0),
+                                         ],
+                                         ],
+                                         span: $DIR/issue-75930-derive-cfg.rs:35:15: 35:27 (#0),
+                                 ],
+                                 ],
+                                 span: $DIR/issue-75930-derive-cfg.rs:34:25: 36:10 (#0),
+                             Punct {
+                             Punct {
+                                 ch: '#',
+                                 spacing: Alone,
1491                                 span: $DIR/issue-75930-derive-cfg.rs:38:9: 38:10 (#0),
1493                             Group {

1647                                             Punct {
1647                                             Punct {
1648                                                 ch: '#',
1649                                                 spacing: Alone,
+                                                 span: $DIR/issue-75930-derive-cfg.rs:45:17: 45:18 (#0),
+                                             Group {
+                                                 delimiter: Bracket,
+                                                 delimiter: Bracket,
+                                                 stream: TokenStream [
+                                                     Ident {
+                                                         ident: "cfg",
+                                                         span: $DIR/issue-75930-derive-cfg.rs:45:19: 45:22 (#0),
+                                                     Group {
+                                                         delimiter: Parenthesis,
+                                                         delimiter: Parenthesis,
+                                                         stream: TokenStream [
+                                                             Ident {
+                                                                 ident: "FALSE",
+                                                                 span: $DIR/issue-75930-derive-cfg.rs:45:23: 45:28 (#0),
+                                                         ],
+                                                         ],
+                                                         span: $DIR/issue-75930-derive-cfg.rs:45:22: 45:29 (#0),
+                                                 ],
+                                                 ],
+                                                 span: $DIR/issue-75930-derive-cfg.rs:45:18: 45:30 (#0),
---
test result: FAILED. 11157 passed; 6 failed; 85 ignored; 0 measured; 0 filtered out; finished in 144.40s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:17:16
