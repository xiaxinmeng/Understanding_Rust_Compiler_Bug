plain
  |
1 | ::abc::def::return
  |             ^^^^^^ expected identifier, found keyword

............F..............................F...

---- parse::tests::string_to_tts_1 stdout ----
---- parse::tests::string_to_tts_1 stdout ----
thread 'parse::tests::string_to_tts_1' panicked at 'assertion failed: `(left == right)`
  left: `TokenStream([(Token(Token { kind: Ident("fn", false), span: Span { lo: BytePos(0), hi: BytePos(2), ctxt: #0 } }), Alone), (Token(Token { kind: Ident("a", false), span: Span { lo: BytePos(3), hi: BytePos(4), ctxt: #0 } }), Alone), (Delimited(DelimSpan { open: Span { lo: BytePos(5), hi: BytePos(6), ctxt: #0 }, close: Span { lo: BytePos(13), hi: BytePos(14), ctxt: #0 } }, Parenthesis, TokenStream([(Token(Token { kind: Ident("b", false), span: Span { lo: BytePos(6), hi: BytePos(7), ctxt: #0 } }), Alone), (Token(Token { kind: Colon, span: Span { lo: BytePos(8), hi: BytePos(9), ctxt: #0 } }), Alone), (Token(Token { kind: Ident("i32", false), span: Span { lo: BytePos(10), hi: BytePos(13), ctxt: #0 } }), Alone)])), Alone), (Delimited(DelimSpan { open: Span { lo: BytePos(15), hi: BytePos(16), ctxt: #0 }, close: Span { lo: BytePos(20), hi: BytePos(21), ctxt: #0 } }, Brace, TokenStream([(Token(Token { kind: Ident("b", false), span: Span { lo: BytePos(17), hi: BytePos(18), ctxt: #0 } }), Joint), (Token(Token { kind: Semi, span: Span { lo: BytePos(18), hi: BytePos(19), ctxt: #0 } }), Alone)])), Alone)])`,
 right: `TokenStream([(Token(Token { kind: Ident("fn", false), span: Span { lo: BytePos(0), hi: BytePos(2), ctxt: #0 } }), Alone), (Token(Token { kind: Ident("a", false), span: Span { lo: BytePos(3), hi: BytePos(4), ctxt: #0 } }), Alone), (Delimited(DelimSpan { open: Span { lo: BytePos(5), hi: BytePos(6), ctxt: #0 }, close: Span { lo: BytePos(13), hi: BytePos(14), ctxt: #0 } }, Parenthesis, TokenStream([(Token(Token { kind: Ident("b", false), span: Span { lo: BytePos(6), hi: BytePos(7), ctxt: #0 } }), Alone), (Token(Token { kind: Colon, span: Span { lo: BytePos(8), hi: BytePos(9), ctxt: #0 } }), Alone), (Token(Token { kind: Ident("i32", false), span: Span { lo: BytePos(10), hi: BytePos(13), ctxt: #0 } }), Alone)])), Alone), (Delimited(DelimSpan { open: Span { lo: BytePos(15), hi: BytePos(16), ctxt: #0 }, close: Span { lo: BytePos(20), hi: BytePos(21), ctxt: #0 } }, Brace, TokenStream([(Token(Token { kind: Ident("b", false), span: Span { lo: BytePos(17), hi: BytePos(18), ctxt: #0 } }), Alone), (Token(Token { kind: Semi, span: Span { lo: BytePos(18), hi: BytePos(19), ctxt: #0 } }), Alone)])), Alone)])`', compiler/rustc_expand/src/parse/tests.rs:139:9

---- tokenstream::tests::test_to_from_bijection stdout ----
thread 'tokenstream::tests::test_to_from_bijection' panicked at 'assertion failed: `(left == right)`
thread 'tokenstream::tests::test_to_from_bijection' panicked at 'assertion failed: `(left == right)`
  left: `TokenStream([(Token(Token { kind: Ident("foo", false), span: Span { lo: BytePos(0), hi: BytePos(3), ctxt: #0 } }), Joint), (Token(Token { kind: ModSep, span: Span { lo: BytePos(3), hi: BytePos(5), ctxt: #0 } }), Alone), (Token(Token { kind: Ident("bar", false), span: Span { lo: BytePos(5), hi: BytePos(8), ctxt: #0 } }), Alone), (Delimited(DelimSpan { open: Span { lo: BytePos(8), hi: BytePos(9), ctxt: #0 }, close: Span { lo: BytePos(12), hi: BytePos(13), ctxt: #0 } }, Parenthesis, TokenStream([(Token(Token { kind: Ident("baz", false), span: Span { lo: BytePos(9), hi: BytePos(12), ctxt: #0 } }), Alone)])), Alone)])`,
 right: `TokenStream([(Token(Token { kind: Ident("foo", false), span: Span { lo: BytePos(0), hi: BytePos(3), ctxt: #0 } }), Alone), (Token(Token { kind: ModSep, span: Span { lo: BytePos(3), hi: BytePos(5), ctxt: #0 } }), Alone), (Token(Token { kind: Ident("bar", false), span: Span { lo: BytePos(5), hi: BytePos(8), ctxt: #0 } }), Alone), (Delimited(DelimSpan { open: Span { lo: BytePos(8), hi: BytePos(9), ctxt: #0 }, close: Span { lo: BytePos(12), hi: BytePos(13), ctxt: #0 } }, Parenthesis, TokenStream([(Token(Token { kind: Ident("baz", false), span: Span { lo: BytePos(9), hi: BytePos(12), ctxt: #0 } }), Alone)])), Alone)])`', compiler/rustc_expand/src/tokenstream/tests.rs:39:9


error: test failed, to rerun pass '-p rustc_expand --lib'
    parse::tests::string_to_tts_1
    tokenstream::tests::test_to_from_bijection

test result: FAILED. 45 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
