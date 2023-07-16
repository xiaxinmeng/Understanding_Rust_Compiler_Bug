plain
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 33 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiii..
test result: ok. 2 passed; 0 failed; 31 ignored; 0 measured; 0 filtered out; finished in 0.04s

 finished in 0.106 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.39s

 finished in 2.464 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---

error: expect test failed
   --> compiler/rustc_lexer/src/tests.rs:135:9

You can update all `expect![[]]` tests by running:

    env UPDATE_EXPECT=1 cargo test

To update a single test, place the cursor on `expect` token and use `run` feature of rust-analyzer.
Expect:
----
----
Token { kind: BlockComment { doc_style: None, terminated: true }, len: 20 }
Token { kind: Whitespace, len: 1 }
Token { kind: Ident, len: 2 }
Token { kind: Whitespace, len: 1 }
Token { kind: Ident, len: 4 }
Token { kind: OpenParen, len: 1 }
Token { kind: CloseParen, len: 1 }
Token { kind: Whitespace, len: 1 }
Token { kind: OpenBrace, len: 1 }
Token { kind: Whitespace, len: 1 }
Token { kind: Ident, len: 7 }
Token { kind: Bang, len: 1 }
Token { kind: OpenParen, len: 1 }
Token { kind: Literal { kind: Str { terminated: true }, suffix_start: 7 }, len: 7 }
Token { kind: CloseParen, len: 1 }
Token { kind: Semi, len: 1 }
Token { kind: Whitespace, len: 1 }
Token { kind: CloseBrace, len: 1 }
Token { kind: Whitespace, len: 1 }
----

Actual:
----
----
Token { kind: BlockComment { doc_style: None, terminated: true }, len: 20 }
Token { kind: Whitespace, len: 1 }
Token { kind: Ident { followed_by: ' ' }, len: 2 }
Token { kind: Whitespace, len: 1 }
Token { kind: Ident { followed_by: '(' }, len: 4 }
Token { kind: OpenParen, len: 1 }
Token { kind: CloseParen, len: 1 }
Token { kind: Whitespace, len: 1 }
Token { kind: OpenBrace, len: 1 }
Token { kind: Whitespace, len: 1 }
Token { kind: Ident { followed_by: '!' }, len: 7 }
Token { kind: Bang, len: 1 }
Token { kind: OpenParen, len: 1 }
Token { kind: Literal { kind: Str { terminated: true }, suffix_start: 7 }, len: 7 }
Token { kind: CloseParen, len: 1 }
Token { kind: Semi, len: 1 }
Token { kind: Whitespace, len: 1 }
Token { kind: CloseBrace, len: 1 }
Token { kind: Whitespace, len: 1 }
----

Diff:
----
----
Token { kind: BlockComment { doc_style: None, terminated: true }, len: 20 }
Token { kind: Whitespace, len: 1 }
Token { kind: Ident { followed_by: ' ' }, len: 2 }
Token { kind: Ident, len: 2 }
Token { kind: Whitespace, len: 1 }
Token { kind: Ident { followed_by: '(' }, len: 4 }
Token { kind: Ident, len: 4 }
Token { kind: OpenParen, len: 1 }
Token { kind: CloseParen, len: 1 }
Token { kind: Whitespace, len: 1 }
Token { kind: OpenBrace, len: 1 }
Token { kind: Whitespace, len: 1 }
Token { kind: Ident { followed_by: '!' }, len: 7 }
Token { kind: Ident, len: 7 }
Token { kind: Bang, len: 1 }
Token { kind: OpenParen, len: 1 }
Token { kind: Literal { kind: Str { terminated: true }, suffix_start: 7 }, len: 7 }
Token { kind: CloseParen, len: 1 }
Token { kind: Semi, len: 1 }
Token { kind: Whitespace, len: 1 }
Token { kind: CloseBrace, len: 1 }
Token { kind: Whitespace, len: 1 }

----




failures:
    tests::smoke_test

test result: FAILED. 29 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '-p rustc_lexer --lib'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc_lexer" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:21:38
