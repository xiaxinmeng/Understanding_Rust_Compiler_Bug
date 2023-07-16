
error: internal compiler error: broken MIR in DefId(0:6 ~ issue_66205_struct_dup[317d]::fact[0]) (CanonicalUserTypeAnnotation { user_ty: Canonical { max_universe: U0, variables: [], value: TypeOf(DefId(0:4 ~ issue_66205_struct_dup[317d]::MyStruct[0]::{{constructor}}[0]), UserSubsts { substs: [Const { ty: usize, val: Unevaluated(DefId(0:8 ~ issue_66205_struct_dup[317d]::fact[0]::{{constant}}[0]), [Const { ty: usize, val: Param(N/#0) }], None) }], user_self_ty: None }) }, span: /home/programming/rust/src/test/ui/const-generics/issues/issue-66205-struct-dup.rs:9:5: 9:26, inferred_ty: MyStruct<{ N - 1 }> }): bad user type AscribeUserType(MyStruct<{ N - 1 }>, DefId(0:4 ~ issue_66205_struct_dup[317d]::MyStruct[0]::{{constructor}}[0]) UserSubsts { substs: [Const { ty: usize, val: Unevaluated(DefId(0:8 ~ issue_66205_struct_dup[317d]::fact[0]::{{constant}}[0]), [Const { ty: usize, val: Param(N/#0) }], None) }], user_self_ty: None }): NoSolution

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:360:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
