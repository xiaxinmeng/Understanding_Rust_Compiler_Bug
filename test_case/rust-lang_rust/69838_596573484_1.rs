
---- [pretty] pretty/issue-12590-a.rs stdout ----

error: pretty-printed source does not typecheck
status: exit code: 101
command: "/home/centril/programming/rust/rust-gamma/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "-" "-Zno-codegen" "--out-dir" "/home/centril/programming/rust/rust-gamma/build/x86_64-unknown-linux-gnu/test/pretty/issue-12590-a/issue-12590-a.pretty-out" "--target=x86_64-unknown-linux-gnu" "-L" "/home/centril/programming/rust/rust-gamma/build/x86_64-unknown-linux-gnu/test/pretty" "-L" "/home/centril/programming/rust/rust-gamma/build/x86_64-unknown-linux-gnu/test/pretty/issue-12590-a/auxiliary.pretty" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/home/centril/programming/rust/rust-gamma/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Ztreat-err-as-bug"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
[DEBUG rustc_expand::expand] expand_crate, module #0 = ModuleData { mod_path: [rust_out#0], directory: "<anon>" }
[DEBUG rustc_expand::expand] expand_crate, module #1 = ModuleData { mod_path: [rust_out#0], directory: "" }
[DEBUG rustc_expand::module] parse_external_mod(id=issue_12590_b#0, span=Span { lo: BytePos(248), hi: BytePos(266), ctxt: 
#0 }, ownership=Owned { relative: None }, path="", attrs=[Attribute { kind: Normal(AttrItem { path: Path { span: Span { lo: BytePos(221), hi: BytePos(225), ctxt: #0 }, segments: [PathSegment { ident: path#0, id: NodeId(4294967040), args: None }] }, args: Eq(Span { lo: BytePos(226), hi: BytePos(227), ctxt: #0 }, TokenStream([(Token(Token { kind: Literal(Lit { kind: Str, symbol: "issue-12590-b.rs", suffix: None }), span: Span { lo: BytePos(228), hi: BytePos(246), ctxt: #0 } }), NonJoint)])) }), id: AttrId(0), style: Outer, span: Span { lo: BytePos(219), hi: BytePos(247), ctxt: #0 } }]
[DEBUG rustc_expand::module] parse_external_mod, mp(ownership=Owned { relative: None }, path="issue-12590-b.rs")
error: couldn't read issue-12590-b.rs: No such file or directory (os error 2)
  --> <anon>:10:1
   |
10 | mod issue_12590_b;
   | ^^^^^^^^^^^^^^^^^^

thread 'rustc' panicked at 'aborting due to `-Z treat-err-as-bug=1`', src/librustc_errors/lib.rs:901:13
