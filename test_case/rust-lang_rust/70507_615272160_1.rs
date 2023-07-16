
error: internal compiler error: PromoteTemps: MIR had errors
 --> src/lib.rs:6:1
  |
6 | / fn test() {
7 | |     Foo.foo();
8 | | }
  | |_^

error: internal compiler error: broken MIR in DefId(0:8 ~ playground[d1c7]::test[0]) ("return type"): bad type [type error]
 --> src/lib.rs:6:1
  |
6 | / fn test() {
7 | |     Foo.foo();
8 | | }
  | |_^

error: internal compiler error: broken MIR in DefId(0:8 ~ playground[d1c7]::test[0]) (LocalDecl { mutability: Mut, local_info: Other, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, source_info: SourceInfo { span: src/lib.rs:6:1: 8:2, scope: scope[0] } }): bad type [type error]
 --> src/lib.rs:6:1
  |
6 | / fn test() {
7 | |     Foo.foo();
8 | | }
  | |_^
