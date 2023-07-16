plain
.................................................................................................... 9000/11196
.................................................................................................... 9100/11196
......................................................................................i......i...... 9200/11196
.................................................................................................... 9300/11196
.........................iiiiii...iiiiiii........................................................... 9400/11196
.................................................................................................... 9600/11196
...............................................................................................test [ui] ui/issues/issue-74564-if-expr-stack-overflow.rs has been running for over 60 seconds
..... 9700/11196
.................................................................................................... 9800/11196
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 27 tests
iiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.070 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i..i....ii.........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.49s

 finished in 2.573 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
    Checking hashbrown v0.9.0
    Checking object v0.22.0
    Checking addr2line v0.14.0
 Documenting std v0.0.0 (/checkout/library/std)
warning: this attribute can only be applied at the crate level
   |
   |
44 | #![doc(html_root_url = "https://docs.rs/backtrace")]
   |
   |
   = note: read https://doc.rust-lang.org/nightly/rustdoc/the-doc-attribute.html#at-the-crate-level for more information
warning: 1 warning emitted

    Finished release [optimized] target(s) in 22.53s
   Compiling std v0.0.0 (/checkout/library/std)
---

---- [ui] rustdoc-ui/check-codeblock-attrs.rs stdout ----
diff of stderr:

1 error: unknown attribute `compile-fail`. Did you mean `compile_fail`?
-   --> $DIR/check-attr.rs:3:1
3    |
3    |
4 LL | / /// foo
5 LL | |
11    | |_______^
12    |
13 note: the lint level is defined here
-   --> $DIR/check-attr.rs:1:9
-   --> $DIR/check-attr.rs:1:9
+   --> $DIR/check-codeblock-attrs.rs:1:9
15    |
16 LL | #![deny(invalid_codeblock_attributes)]


18    = help: the code block will either not be tested if not marked as a rust one or won't fail if it compiles successfully
19 
20 error: unknown attribute `compilefail`. Did you mean `compile_fail`?
-   --> $DIR/check-attr.rs:3:1
22    |
22    |
23 LL | / /// foo
24 LL | |

32    = help: the code block will either not be tested if not marked as a rust one or won't fail if it compiles successfully
33 
34 error: unknown attribute `comPile_fail`. Did you mean `compile_fail`?
-   --> $DIR/check-attr.rs:3:1
36    |
36    |
37 LL | / /// foo
38 LL | |

46    = help: the code block will either not be tested if not marked as a rust one or won't fail if it compiles successfully
47 
48 error: unknown attribute `should-panic`. Did you mean `should_panic`?
-   --> $DIR/check-attr.rs:13:1
50    |
50    |
51 LL | / /// bar
52 LL | |

60    = help: the code block will either not be tested if not marked as a rust one or won't fail if it doesn't panic when running
61 
62 error: unknown attribute `shouldpanic`. Did you mean `should_panic`?
-   --> $DIR/check-attr.rs:13:1
64    |
64    |
65 LL | / /// bar
66 LL | |

74    = help: the code block will either not be tested if not marked as a rust one or won't fail if it doesn't panic when running
75 
76 error: unknown attribute `sHould_panic`. Did you mean `should_panic`?
-   --> $DIR/check-attr.rs:13:1
78    |
78    |
79 LL | / /// bar
80 LL | |

88    = help: the code block will either not be tested if not marked as a rust one or won't fail if it doesn't panic when running
89 
90 error: unknown attribute `no-run`. Did you mean `no_run`?
-   --> $DIR/check-attr.rs:23:1
92    |
92    |
93 LL | / /// foobar
94 LL | |

102    = help: the code block will either not be tested if not marked as a rust one or will be run (which you might not want)
103 
104 error: unknown attribute `norun`. Did you mean `no_run`?
-   --> $DIR/check-attr.rs:23:1
106    |
106    |
107 LL | / /// foobar
108 LL | |

116    = help: the code block will either not be tested if not marked as a rust one or will be run (which you might not want)
117 
118 error: unknown attribute `no_Run`. Did you mean `no_run`?
-   --> $DIR/check-attr.rs:23:1
120    |
120    |
121 LL | / /// foobar
122 LL | |

130    = help: the code block will either not be tested if not marked as a rust one or will be run (which you might not want)
131 
132 error: unknown attribute `allow-fail`. Did you mean `allow_fail`?
-   --> $DIR/check-attr.rs:33:1
134    |
134    |
135 LL | / /// barfoo
136 LL | |

144    = help: the code block will either not be tested if not marked as a rust one or will be run (which you might not want)
145 
146 error: unknown attribute `allowfail`. Did you mean `allow_fail`?
-   --> $DIR/check-attr.rs:33:1
148    |
148    |
149 LL | / /// barfoo
150 LL | |

158    = help: the code block will either not be tested if not marked as a rust one or will be run (which you might not want)
159 
160 error: unknown attribute `alLow_fail`. Did you mean `allow_fail`?
-   --> $DIR/check-attr.rs:33:1
162    |
162    |
163 LL | / /// barfoo
164 LL | |

172    = help: the code block will either not be tested if not marked as a rust one or will be run (which you might not want)
173 
174 error: unknown attribute `test-harness`. Did you mean `test_harness`?
-   --> $DIR/check-attr.rs:43:1
176    |
176    |
177 LL | / /// b
178 LL | |

186    = help: the code block will either not be tested if not marked as a rust one or the code will be wrapped inside a main function
187 
188 error: unknown attribute `testharness`. Did you mean `test_harness`?
-   --> $DIR/check-attr.rs:43:1
190    |
190    |
191 LL | / /// b
192 LL | |

200    = help: the code block will either not be tested if not marked as a rust one or the code will be wrapped inside a main function
201 
202 error: unknown attribute `teSt_harness`. Did you mean `test_harness`?
-   --> $DIR/check-attr.rs:43:1
204    |
204    |
205 LL | / /// b
206 LL | |

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/check-codeblock-attrs/check-codeblock-attrs.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/check-codeblock-attrs/check-codeblock-attrs.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args check-codeblock-attrs.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/check-codeblock-attrs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/check-codeblock-attrs" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/check-codeblock-attrs/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unknown attribute `compile-fail`. Did you mean `compile_fail`?
   |
   |
LL | / /// foo
LL | | //~^ ERROR
LL | | //~^^ ERROR
LL | | //~^^^ ERROR
LL | | /// boo
LL | | /// 