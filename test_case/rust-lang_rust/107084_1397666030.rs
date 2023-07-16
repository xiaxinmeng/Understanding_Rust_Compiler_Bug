plain
 finished in 0.509 seconds
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 219 tests
.....F.................................................i................................ 88/219
.........................F....................i..............F........................F. 176/219
failures:
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [mir-opt] tests/mir-opt/address_of.rs stdout ----
---- [mir-opt] tests/mir-opt/address_of.rs stdout ----
2 
3 | User Type Annotations
4 | 0: user_ty: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Ty(General(U0)) }], value: Ty(RawPtr(TypeAndMut { ty: Bound(DebruijnIndex(0), BoundTy { var: 0, kind: Anon }), mutbl: Not })) }, span: $DIR/address_of.rs:7:5: 7:18, inferred_ty: RawPtr(TypeAndMut { ty: Array(Int(I32), Const { ty: Uint(Usize), kind: Value(Leaf(0x000000000000000a)) }), mutbl: Not })
- | 1: user_ty: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Region(U0) }], value: Ty(RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[a28e]::marker::Send)), [])], ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrAnon(0, None) }), Dyn), mutbl: Not })) }, span: $DIR/address_of.rs:9:5: 9:25, inferred_ty: RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[a28e]::marker::Send)), [])], ReErased, Dyn), mutbl: Not })
+ | 1: user_ty: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Region(U0) }], value: Ty(RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[5618]::marker::Send)), [])], ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrAnon(0, None) }), Dyn), mutbl: Not })) }, span: $DIR/address_of.rs:9:5: 9:25, inferred_ty: RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[5618]::marker::Send)), [])], ReErased, Dyn), mutbl: Not })
6 | 2: user_ty: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Ty(General(U0)) }], value: Ty(RawPtr(TypeAndMut { ty: Bound(DebruijnIndex(0), BoundTy { var: 0, kind: Anon }), mutbl: Not })) }, span: $DIR/address_of.rs:13:12: 13:20, inferred_ty: RawPtr(TypeAndMut { ty: Array(Int(I32), Const { ty: Uint(Usize), kind: Value(Leaf(0x000000000000000a)) }), mutbl: Not })
7 | 3: user_ty: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Ty(General(U0)) }], value: Ty(RawPtr(TypeAndMut { ty: Bound(DebruijnIndex(0), BoundTy { var: 0, kind: Anon }), mutbl: Not })) }, span: $DIR/address_of.rs:13:12: 13:20, inferred_ty: RawPtr(TypeAndMut { ty: Array(Int(I32), Const { ty: Uint(Usize), kind: Value(Leaf(0x000000000000000a)) }), mutbl: Not })
8 | 4: user_ty: Canonical { max_universe: U0, variables: [], value: Ty(RawPtr(TypeAndMut { ty: Array(Int(I32), Const { ty: Uint(Usize), kind: Value(Leaf(0x000000000000000a)) }), mutbl: Not })) }, span: $DIR/address_of.rs:14:12: 14:28, inferred_ty: RawPtr(TypeAndMut { ty: Array(Int(I32), Const { ty: Uint(Usize), kind: Value(Leaf(0x000000000000000a)) }), mutbl: Not })

9 | 5: user_ty: Canonical { max_universe: U0, variables: [], value: Ty(RawPtr(TypeAndMut { ty: Array(Int(I32), Const { ty: Uint(Usize), kind: Value(Leaf(0x000000000000000a)) }), mutbl: Not })) }, span: $DIR/address_of.rs:14:12: 14:28, inferred_ty: RawPtr(TypeAndMut { ty: Array(Int(I32), Const { ty: Uint(Usize), kind: Value(Leaf(0x000000000000000a)) }), mutbl: Not })
- | 6: user_ty: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Region(U0) }], value: Ty(RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[a28e]::marker::Send)), [])], ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrAnon(0, None) }), Dyn), mutbl: Not })) }, span: $DIR/address_of.rs:15:12: 15:27, inferred_ty: RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[a28e]::marker::Send)), [])], ReErased, Dyn), mutbl: Not })
- | 7: user_ty: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Region(U0) }], value: Ty(RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[a28e]::marker::Send)), [])], ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrAnon(0, None) }), Dyn), mutbl: Not })) }, span: $DIR/address_of.rs:15:12: 15:27, inferred_ty: RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[a28e]::marker::Send)), [])], ReErased, Dyn), mutbl: Not })
+ | 6: user_ty: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Region(U0) }], value: Ty(RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[5618]::marker::Send)), [])], ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrAnon(0, None) }), Dyn), mutbl: Not })) }, span: $DIR/address_of.rs:15:12: 15:27, inferred_ty: RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[5618]::marker::Send)), [])], ReErased, Dyn), mutbl: Not })
+ | 7: user_ty: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Region(U0) }], value: Ty(RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[5618]::marker::Send)), [])], ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrAnon(0, None) }), Dyn), mutbl: Not })) }, span: $DIR/address_of.rs:15:12: 15:27, inferred_ty: RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[5618]::marker::Send)), [])], ReErased, Dyn), mutbl: Not })
12 | 8: user_ty: Canonical { max_universe: U0, variables: [], value: Ty(RawPtr(TypeAndMut { ty: Slice(Int(I32)), mutbl: Not })) }, span: $DIR/address_of.rs:16:12: 16:24, inferred_ty: RawPtr(TypeAndMut { ty: Slice(Int(I32)), mutbl: Not })
13 | 9: user_ty: Canonical { max_universe: U0, variables: [], value: Ty(RawPtr(TypeAndMut { ty: Slice(Int(I32)), mutbl: Not })) }, span: $DIR/address_of.rs:16:12: 16:24, inferred_ty: RawPtr(TypeAndMut { ty: Slice(Int(I32)), mutbl: Not })
14 | 10: user_ty: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Ty(General(U0)) }], value: Ty(RawPtr(TypeAndMut { ty: Bound(DebruijnIndex(0), BoundTy { var: 0, kind: Anon }), mutbl: Not })) }, span: $DIR/address_of.rs:18:5: 18:18, inferred_ty: RawPtr(TypeAndMut { ty: Array(Int(I32), Const { ty: Uint(Usize), kind: Value(Leaf(0x000000000000000a)) }), mutbl: Not })

- | 11: user_ty: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Region(U0) }], value: Ty(RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[a28e]::marker::Send)), [])], ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrAnon(0, None) }), Dyn), mutbl: Not })) }, span: $DIR/address_of.rs:20:5: 20:25, inferred_ty: RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[a28e]::marker::Send)), [])], ReErased, Dyn), mutbl: Not })
+ | 11: user_ty: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Region(U0) }], value: Ty(RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[5618]::marker::Send)), [])], ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrAnon(0, None) }), Dyn), mutbl: Not })) }, span: $DIR/address_of.rs:20:5: 20:25, inferred_ty: RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[5618]::marker::Send)), [])], ReErased, Dyn), mutbl: Not })
16 | 12: user_ty: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Ty(General(U0)) }], value: Ty(RawPtr(TypeAndMut { ty: Bound(DebruijnIndex(0), BoundTy { var: 0, kind: Anon }), mutbl: Not })) }, span: $DIR/address_of.rs:23:12: 23:20, inferred_ty: RawPtr(TypeAndMut { ty: Array(Int(I32), Const { ty: Uint(Usize), kind: Value(Leaf(0x000000000000000a)) }), mutbl: Not })
17 | 13: user_ty: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Ty(General(U0)) }], value: Ty(RawPtr(TypeAndMut { ty: Bound(DebruijnIndex(0), BoundTy { var: 0, kind: Anon }), mutbl: Not })) }, span: $DIR/address_of.rs:23:12: 23:20, inferred_ty: RawPtr(TypeAndMut { ty: Array(Int(I32), Const { ty: Uint(Usize), kind: Value(Leaf(0x000000000000000a)) }), mutbl: Not })
18 | 14: user_ty: Canonical { max_universe: U0, variables: [], value: Ty(RawPtr(TypeAndMut { ty: Array(Int(I32), Const { ty: Uint(Usize), kind: Value(Leaf(0x000000000000000a)) }), mutbl: Not })) }, span: $DIR/address_of.rs:24:12: 24:28, inferred_ty: RawPtr(TypeAndMut { ty: Array(Int(I32), Const { ty: Uint(Usize), kind: Value(Leaf(0x000000000000000a)) }), mutbl: Not })

19 | 15: user_ty: Canonical { max_universe: U0, variables: [], value: Ty(RawPtr(TypeAndMut { ty: Array(Int(I32), Const { ty: Uint(Usize), kind: Value(Leaf(0x000000000000000a)) }), mutbl: Not })) }, span: $DIR/address_of.rs:24:12: 24:28, inferred_ty: RawPtr(TypeAndMut { ty: Array(Int(I32), Const { ty: Uint(Usize), kind: Value(Leaf(0x000000000000000a)) }), mutbl: Not })
- | 16: user_ty: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Region(U0) }], value: Ty(RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[a28e]::marker::Send)), [])], ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrAnon(0, None) }), Dyn), mutbl: Not })) }, span: $DIR/address_of.rs:25:12: 25:27, inferred_ty: RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[a28e]::marker::Send)), [])], ReErased, Dyn), mutbl: Not })
- | 17: user_ty: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Region(U0) }], value: Ty(RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[a28e]::marker::Send)), [])], ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrAnon(0, None) }), Dyn), mutbl: Not })) }, span: $DIR/address_of.rs:25:12: 25:27, inferred_ty: RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[a28e]::marker::Send)), [])], ReErased, Dyn), mutbl: Not })
+ | 16: user_ty: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Region(U0) }], value: Ty(RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[5618]::marker::Send)), [])], ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrAnon(0, None) }), Dyn), mutbl: Not })) }, span: $DIR/address_of.rs:25:12: 25:27, inferred_ty: RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[5618]::marker::Send)), [])], ReErased, Dyn), mutbl: Not })
+ | 17: user_ty: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Region(U0) }], value: Ty(RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[5618]::marker::Send)), [])], ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrAnon(0, None) }), Dyn), mutbl: Not })) }, span: $DIR/address_of.rs:25:12: 25:27, inferred_ty: RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[5618]::marker::Send)), [])], ReErased, Dyn), mutbl: Not })
22 | 18: user_ty: Canonical { max_universe: U0, variables: [], value: Ty(RawPtr(TypeAndMut { ty: Slice(Int(I32)), mutbl: Not })) }, span: $DIR/address_of.rs:26:12: 26:24, inferred_ty: RawPtr(TypeAndMut { ty: Slice(Int(I32)), mutbl: Not })
23 | 19: user_ty: Canonical { max_universe: U0, variables: [], value: Ty(RawPtr(TypeAndMut { ty: Slice(Int(I32)), mutbl: Not })) }, span: $DIR/address_of.rs:26:12: 26:24, inferred_ty: RawPtr(TypeAndMut { ty: Slice(Int(I32)), mutbl: Not })
24 | 20: user_ty: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Ty(General(U0)) }], value: Ty(RawPtr(TypeAndMut { ty: Bound(DebruijnIndex(0), BoundTy { var: 0, kind: Anon }), mutbl: Mut })) }, span: $DIR/address_of.rs:28:5: 28:16, inferred_ty: RawPtr(TypeAndMut { ty: Array(Int(I32), Const { ty: Uint(Usize), kind: Value(Leaf(0x000000000000000a)) }), mutbl: Mut })

- | 21: user_ty: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Region(U0) }], value: Ty(RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[a28e]::marker::Send)), [])], ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrAnon(0, None) }), Dyn), mutbl: Mut })) }, span: $DIR/address_of.rs:30:5: 30:23, inferred_ty: RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[a28e]::marker::Send)), [])], ReErased, Dyn), mutbl: Mut })
+ | 21: user_ty: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Region(U0) }], value: Ty(RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[5618]::marker::Send)), [])], ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrAnon(0, None) }), Dyn), mutbl: Mut })) }, span: $DIR/address_of.rs:30:5: 30:23, inferred_ty: RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[5618]::marker::Send)), [])], ReErased, Dyn), mutbl: Mut })
26 | 22: user_ty: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Ty(General(U0)) }], value: Ty(RawPtr(TypeAndMut { ty: Bound(DebruijnIndex(0), BoundTy { var: 0, kind: Anon }), mutbl: Mut })) }, span: $DIR/address_of.rs:33:12: 33:18, inferred_ty: RawPtr(TypeAndMut { ty: Array(Int(I32), Const { ty: Uint(Usize), kind: Value(Leaf(0x000000000000000a)) }), mutbl: Mut })
27 | 23: user_ty: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Ty(General(U0)) }], value: Ty(RawPtr(TypeAndMut { ty: Bound(DebruijnIndex(0), BoundTy { var: 0, kind: Anon }), mutbl: Mut })) }, span: $DIR/address_of.rs:33:12: 33:18, inferred_ty: RawPtr(TypeAndMut { ty: Array(Int(I32), Const { ty: Uint(Usize), kind: Value(Leaf(0x000000000000000a)) }), mutbl: Mut })
28 | 24: user_ty: Canonical { max_universe: U0, variables: [], value: Ty(RawPtr(TypeAndMut { ty: Array(Int(I32), Const { ty: Uint(Usize), kind: Value(Leaf(0x000000000000000a)) }), mutbl: Mut })) }, span: $DIR/address_of.rs:34:12: 34:26, inferred_ty: RawPtr(TypeAndMut { ty: Array(Int(I32), Const { ty: Uint(Usize), kind: Value(Leaf(0x000000000000000a)) }), mutbl: Mut })

29 | 25: user_ty: Canonical { max_universe: U0, variables: [], value: Ty(RawPtr(TypeAndMut { ty: Array(Int(I32), Const { ty: Uint(Usize), kind: Value(Leaf(0x000000000000000a)) }), mutbl: Mut })) }, span: $DIR/address_of.rs:34:12: 34:26, inferred_ty: RawPtr(TypeAndMut { ty: Array(Int(I32), Const { ty: Uint(Usize), kind: Value(Leaf(0x000000000000000a)) }), mutbl: Mut })
- | 26: user_ty: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Region(U0) }], value: Ty(RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[a28e]::marker::Send)), [])], ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrAnon(0, None) }), Dyn), mutbl: Mut })) }, span: $DIR/address_of.rs:35:12: 35:25, inferred_ty: RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[a28e]::marker::Send)), [])], ReErased, Dyn), mutbl: Mut })
- | 27: user_ty: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Region(U0) }], value: Ty(RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[a28e]::marker::Send)), [])], ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrAnon(0, None) }), Dyn), mutbl: Mut })) }, span: $DIR/address_of.rs:35:12: 35:25, inferred_ty: RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[a28e]::marker::Send)), [])], ReErased, Dyn), mutbl: Mut })
+ | 26: user_ty: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Region(U0) }], value: Ty(RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[5618]::marker::Send)), [])], ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrAnon(0, None) }), Dyn), mutbl: Mut })) }, span: $DIR/address_of.rs:35:12: 35:25, inferred_ty: RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[5618]::marker::Send)), [])], ReErased, Dyn), mutbl: Mut })
+ | 27: user_ty: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Region(U0) }], value: Ty(RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[5618]::marker::Send)), [])], ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrAnon(0, None) }), Dyn), mutbl: Mut })) }, span: $DIR/address_of.rs:35:12: 35:25, inferred_ty: RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[5618]::marker::Send)), [])], ReErased, Dyn), mutbl: Mut })
32 | 28: user_ty: Canonical { max_universe: U0, variables: [], value: Ty(RawPtr(TypeAndMut { ty: Slice(Int(I32)), mutbl: Mut })) }, span: $DIR/address_of.rs:36:12: 36:22, inferred_ty: RawPtr(TypeAndMut { ty: Slice(Int(I32)), mutbl: Mut })
33 | 29: user_ty: Canonical { max_universe: U0, variables: [], value: Ty(RawPtr(TypeAndMut { ty: Slice(Int(I32)), mutbl: Mut })) }, span: $DIR/address_of.rs:36:12: 36:22, inferred_ty: RawPtr(TypeAndMut { ty: Slice(Int(I32)), mutbl: Mut })


40     let _5: RawPtr(TypeAndMut { ty: Array(Int(I32), Const { ty: Uint(Usize), kind: Value(Leaf(0x000000000000000a)) }), mutbl: Not }); // in scope 0 at $DIR/address_of.rs:+4:5: +4:18
41     let mut _6: RawPtr(TypeAndMut { ty: Array(Int(I32), Const { ty: Uint(Usize), kind: Value(Leaf(0x000000000000000a)) }), mutbl: Not }); // in scope 0 at $DIR/address_of.rs:+4:5: +4:18
42     let _7: RawPtr(TypeAndMut { ty: Array(Int(I32), Const { ty: Uint(Usize), kind: Value(Leaf(0x000000000000000a)) }), mutbl: Not }); // in scope 0 at $DIR/address_of.rs:+5:5: +5:26
-     let _8: RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[a28e]::marker::Send)), [])], ReErased, Dyn), mutbl: Not }); // in scope 0 at $DIR/address_of.rs:+6:5: +6:25
-     let mut _9: RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[a28e]::marker::Send)), [])], ReErased, Dyn), mutbl: Not }); // in scope 0 at $DIR/address_of.rs:+6:5: +6:25
+     let _8: RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[5618]::marker::Send)), [])], ReErased, Dyn), mutbl: Not }); // in scope 0 at $DIR/address_of.rs:+6:5: +6:25
+     let mut _9: RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[5618]::marker::Send)), [])], ReErased, Dyn), mutbl: Not }); // in scope 0 at $DIR/address_of.rs:+6:5: +6:25
45     let mut _10: RawPtr(TypeAndMut { ty: Array(Int(I32), Const { ty: Uint(Usize), kind: Value(Leaf(0x000000000000000a)) }), mutbl: Not }); // in scope 0 at $DIR/address_of.rs:+6:5: +6:6
46     let _11: RawPtr(TypeAndMut { ty: Slice(Int(I32)), mutbl: Not }); // in scope 0 at $DIR/address_of.rs:+7:5: +7:22
47     let mut _12: RawPtr(TypeAndMut { ty: Array(Int(I32), Const { ty: Uint(Usize), kind: Value(Leaf(0x000000000000000a)) }), mutbl: Not }); // in scope 0 at $DIR/address_of.rs:+7:5: +7:6

52     let _21: RawPtr(TypeAndMut { ty: Array(Int(I32), Const { ty: Uint(Usize), kind: Value(Leaf(0x000000000000000a)) }), mutbl: Not }); // in scope 0 at $DIR/address_of.rs:+15:5: +15:18
53     let mut _22: RawPtr(TypeAndMut { ty: Array(Int(I32), Const { ty: Uint(Usize), kind: Value(Leaf(0x000000000000000a)) }), mutbl: Not }); // in scope 0 at $DIR/address_of.rs:+15:5: +15:18
54     let _23: RawPtr(TypeAndMut { ty: Array(Int(I32), Const { ty: Uint(Usize), kind: Value(Leaf(0x000000000000000a)) }), mutbl: Not }); // in scope 0 at $DIR/address_of.rs:+16:5: +16:26
-     let _24: RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[a28e]::marker::Send)), [])], ReErased, Dyn), mutbl: Not }); // in scope 0 at $DIR/address_of.rs:+17:5: +17:25
-     let mut _25: RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[a28e]::marker::Send)), [])], ReErased, Dyn), mutbl: Not }); // in scope 0 at $DIR/address_of.rs:+17:5: +17:25
+     let _24: RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[5618]::marker::Send)), [])], ReErased, Dyn), mutbl: Not }); // in scope 0 at $DIR/address_of.rs:+17:5: +17:25
+     let mut _25: RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[5618]::marker::Send)), [])], ReErased, Dyn), mutbl: Not }); // in scope 0 at $DIR/address_of.rs:+17:5: +17:25
57     let mut _26: RawPtr(TypeAndMut { ty: Array(Int(I32), Const { ty: Uint(Usize), kind: Value(Leaf(0x000000000000000a)) }), mutbl: Not }); // in scope 0 at $DIR/address_of.rs:+17:5: +17:6
58     let _27: RawPtr(TypeAndMut { ty: Slice(Int(I32)), mutbl: Not }); // in scope 0 at $DIR/address_of.rs:+18:5: +18:22
59     let mut _28: RawPtr(TypeAndMut { ty: Array(Int(I32), Const { ty: Uint(Usize), kind: Value(Leaf(0x000000000000000a)) }), mutbl: Not }); // in scope 0 at $DIR/address_of.rs:+18:5: +18:6

62     let _35: RawPtr(TypeAndMut { ty: Array(Int(I32), Const { ty: Uint(Usize), kind: Value(Leaf(0x000000000000000a)) }), mutbl: Mut }); // in scope 0 at $DIR/address_of.rs:+25:5: +25:16
63     let mut _36: RawPtr(TypeAndMut { ty: Array(Int(I32), Const { ty: Uint(Usize), kind: Value(Leaf(0x000000000000000a)) }), mutbl: Mut }); // in scope 0 at $DIR/address_of.rs:+25:5: +25:16
64     let _37: RawPtr(TypeAndMut { ty: Array(Int(I32), Const { ty: Uint(Usize), kind: Value(Leaf(0x000000000000000a)) }), mutbl: Mut }); // in scope 0 at $DIR/address_of.rs:+26:5: +26:24
-     let _38: RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[a28e]::marker::Send)), [])], ReErased, Dyn), mutbl: Mut }); // in scope 0 at $DIR/address_of.rs:+27:5: +27:23
-     let mut _39: RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[a28e]::marker::Send)), [])], ReErased, Dyn), mutbl: Mut }); // in scope 0 at $DIR/address_of.rs:+27:5: +27:23
+     let _38: RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[5618]::marker::Send)), [])], ReErased, Dyn), mutbl: Mut }); // in scope 0 at $DIR/address_of.rs:+27:5: +27:23
+     let mut _39: RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[5618]::marker::Send)), [])], ReErased, Dyn), mutbl: Mut }); // in scope 0 at $DIR/address_of.rs:+27:5: +27:23
67     let mut _40: RawPtr(TypeAndMut { ty: Array(Int(I32), Const { ty: Uint(Usize), kind: Value(Leaf(0x000000000000000a)) }), mutbl: Mut }); // in scope 0 at $DIR/address_of.rs:+27:5: +27:6
68     let _41: RawPtr(TypeAndMut { ty: Slice(Int(I32)), mutbl: Mut }); // in scope 0 at $DIR/address_of.rs:+28:5: +28:20
69     let mut _42: RawPtr(TypeAndMut { ty: Array(Int(I32), Const { ty: Uint(Usize), kind: Value(Leaf(0x000000000000000a)) }), mutbl: Mut }); // in scope 0 at $DIR/address_of.rs:+28:5: +28:6

80                 let _16: RawPtr(TypeAndMut { ty: Array(Int(I32), Const { ty: Uint(Usize), kind: Value(Leaf(0x000000000000000a)) }), mutbl: Not }) as UserTypeProjection { base: UserType(4), projs: [] }; // in scope 3 at $DIR/address_of.rs:+11:9: +11:10
81                 scope 4 {
82                     debug p => _16;      // in scope 4 at $DIR/address_of.rs:+11:9: +11:10
-                     let _17: RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[a28e]::marker::Send)), [])], ReErased, Dyn), mutbl: Not }) as UserTypeProjection { base: UserType(6), projs: [] }; // in scope 4 at $DIR/address_of.rs:+12:9: +12:10
+                     let _17: RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[5618]::marker::Send)), [])], ReErased, Dyn), mutbl: Not }) as UserTypeProjection { base: UserType(6), projs: [] }; // in scope 4 at $DIR/address_of.rs:+12:9: +12:10
84                     scope 5 {
85                         debug p => _17;  // in scope 5 at $DIR/address_of.rs:+12:9: +12:10
86                         let _19: RawPtr(TypeAndMut { ty: Slice(Int(I32)), mutbl: Not }) as UserTypeProjection { base: UserType(8), projs: [] }; // in scope 5 at $DIR/address_of.rs:+13:9: +13:10

92                                 let _30: RawPtr(TypeAndMut { ty: Array(Int(I32), Const { ty: Uint(Usize), kind: Value(Leaf(0x000000000000000a)) }), mutbl: Not }) as UserTypeProjection { base: UserType(14), projs: [] }; // in scope 7 at $DIR/address_of.rs:+21:9: +21:10
93                                 scope 8 {
94                                     debug p => _30; // in scope 8 at $DIR/address_of.rs:+21:9: +21:10
-                                     let _31: RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[a28e]::marker::Send)), [])], ReErased, Dyn), mutbl: Not }) as UserTypeProjection { base: UserType(16), projs: [] }; // in scope 8 at $DIR/address_of.rs:+22:9: +22:10
+                                     let _31: RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[5618]::marker::Send)), [])], ReErased, Dyn), mutbl: Not }) as UserTypeProjection { base: UserType(16), projs: [] }; // in scope 8 at $DIR/address_of.rs:+22:9: +22:10
96                                     scope 9 {
97                                         debug p => _31; // in scope 9 at $DIR/address_of.rs:+22:9: +22:10
98                                         let _33: RawPtr(TypeAndMut { ty: Slice(Int(I32)), mutbl: Not }) as UserTypeProjection { base: UserType(18), projs: [] }; // in scope 9 at $DIR/address_of.rs:+23:9: +23:10

104                                                 let _44: RawPtr(TypeAndMut { ty: Array(Int(I32), Const { ty: Uint(Usize), kind: Value(Leaf(0x000000000000000a)) }), mutbl: Mut }) as UserTypeProjection { base: UserType(24), projs: [] }; // in scope 11 at $DIR/address_of.rs:+31:9: +31:10
105                                                 scope 12 {
106                                                     debug p => _44; // in scope 12 at $DIR/address_of.rs:+31:9: +31:10
-                                                     let _45: RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[a28e]::marker::Send)), [])], ReErased, Dyn), mutbl: Mut }) as UserTypeProjection { base: UserType(26), projs: [] }; // in scope 12 at $DIR/address_of.rs:+32:9: +32:10
+                                                     let _45: RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[5618]::marker::Send)), [])], ReErased, Dyn), mutbl: Mut }) as UserTypeProjection { base: UserType(26), projs: [] }; // in scope 12 at $DIR/address_of.rs:+32:9: +32:10
108                                                     scope 13 {
109                                                         debug p => _45; // in scope 13 at $DIR/address_of.rs:+32:9: +32:10
110                                                         let _47: RawPtr(TypeAndMut { ty: Slice(Int(I32)), mutbl: Mut }) as UserTypeProjection { base: UserType(28), projs: [] }; // in scope 13 at $DIR/address_of.rs:+33:9: +33:10

150         StorageLive(_9);                 // scope 2 at $DIR/address_of.rs:+6:5: +6:25
151         StorageLive(_10);                // scope 2 at $DIR/address_of.rs:+6:5: +6:6
152         _10 = &raw const (*_1);          // scope 2 at $DIR/address_of.rs:+6:5: +6:6
-         _9 = move _10 as RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[a28e]::marker::Send)), [])], ReErased, Dyn), mutbl: Not }) (Pointer(Unsize)); // scope 2 at $DIR/address_of.rs:+6:5: +6:6
+         _9 = move _10 as RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[5618]::marker::Send)), [])], ReErased, Dyn), mutbl: Not }) (Pointer(Unsize)); // scope 2 at $DIR/address_of.rs:+6:5: +6:6
154         StorageDead(_10);                // scope 2 at $DIR/address_of.rs:+6:5: +6:6
155         AscribeUserType(_9, o, UserTypeProjection { base: UserType(1), projs: [] }); // scope 2 at $DIR/address_of.rs:+6:5: +6:25
156         _8 = _9;                         // scope 2 at $DIR/address_of.rs:+6:5: +6:25

179         StorageLive(_17);                // scope 4 at $DIR/address_of.rs:+12:9: +12:10
180         StorageLive(_18);                // scope 4 at $DIR/address_of.rs:+12:30: +12:31
181         _18 = &raw const (*_1);          // scope 4 at $DIR/address_of.rs:+12:30: +12:31
-         _17 = move _18 as RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[a28e]::marker::Send)), [])], ReErased, Dyn), mutbl: Not }) (Pointer(Unsize)); // scope 4 at $DIR/address_of.rs:+12:30: +12:31
+         _17 = move _18 as RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[5618]::marker::Send)), [])], ReErased, Dyn), mutbl: Not }) (Pointer(Unsize)); // scope 4 at $DIR/address_of.rs:+12:30: +12:31
183         StorageDead(_18);                // scope 4 at $DIR/address_of.rs:+12:30: +12:31
184         FakeRead(ForLet(None), _17);     // scope 4 at $DIR/address_of.rs:+12:9: +12:10
185         AscribeUserType(_17, o, UserTypeProjection { base: UserType(7), projs: [] }); // scope 4 at $DIR/address_of.rs:+12:12: +12:27

204         StorageLive(_25);                // scope 6 at $DIR/address_of.rs:+17:5: +17:25
205         StorageLive(_26);                // scope 6 at $DIR/address_of.rs:+17:5: +17:6
206         _26 = &raw const (*_3);          // scope 6 at $DIR/address_of.rs:+17:5: +17:6
-         _25 = move _26 as RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[a28e]::marker::Send)), [])], ReErased, Dyn), mutbl: Not }) (Pointer(Unsize)); // scope 6 at $DIR/address_of.rs:+17:5: +17:6
+         _25 = move _26 as RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[5618]::marker::Send)), [])], ReErased, Dyn), mutbl: Not }) (Pointer(Unsize)); // scope 6 at $DIR/address_of.rs:+17:5: +17:6
208         StorageDead(_26);                // scope 6 at $DIR/address_of.rs:+17:5: +17:6
209         AscribeUserType(_25, o, UserTypeProjection { base: UserType(11), projs: [] }); // scope 6 at $DIR/address_of.rs:+17:5: +17:25
210         _24 = _25;                       // scope 6 at $DIR/address_of.rs:+17:5: +17:25

227         StorageLive(_31);                // scope 8 at $DIR/address_of.rs:+22:9: +22:10
228         StorageLive(_32);                // scope 8 at $DIR/address_of.rs:+22:30: +22:31
229         _32 = &raw const (*_3);          // scope 8 at $DIR/address_of.rs:+22:30: +22:31
-         _31 = move _32 as RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[a28e]::marker::Send)), [])], ReErased, Dyn), mutbl: Not }) (Pointer(Unsize)); // scope 8 at $DIR/address_of.rs:+22:30: +22:31
+         _31 = move _32 as RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[5618]::marker::Send)), [])], ReErased, Dyn), mutbl: Not }) (Pointer(Unsize)); // scope 8 at $DIR/address_of.rs:+22:30: +22:31
231         StorageDead(_32);                // scope 8 at $DIR/address_of.rs:+22:30: +22:31
232         FakeRead(ForLet(None), _31);     // scope 8 at $DIR/address_of.rs:+22:9: +22:10
233         AscribeUserType(_31, o, UserTypeProjection { base: UserType(17), projs: [] }); // scope 8 at $DIR/address_of.rs:+22:12: +22:27

252         StorageLive(_39);                // scope 10 at $DIR/address_of.rs:+27:5: +27:23
253         StorageLive(_40);                // scope 10 at $DIR/address_of.rs:+27:5: +27:6
254         _40 = &raw mut (*_3);            // scope 10 at $DIR/address_of.rs:+27:5: +27:6
-         _39 = move _40 as RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[a28e]::marker::Send)), [])], ReErased, Dyn), mutbl: Mut }) (Pointer(Unsize)); // scope 10 at $DIR/address_of.rs:+27:5: +27:6
+         _39 = move _40 as RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[5618]::marker::Send)), [])], ReErased, Dyn), mutbl: Mut }) (Pointer(Unsize)); // scope 10 at $DIR/address_of.rs:+27:5: +27:6
256         StorageDead(_40);                // scope 10 at $DIR/address_of.rs:+27:5: +27:6
257         AscribeUserType(_39, o, UserTypeProjection { base: UserType(21), projs: [] }); // scope 10 at $DIR/address_of.rs:+27:5: +27:23
258         _38 = _39;                       // scope 10 at $DIR/address_of.rs:+27:5: +27:23

275         StorageLive(_45);                // scope 12 at $DIR/address_of.rs:+32:9: +32:10
276         StorageLive(_46);                // scope 12 at $DIR/address_of.rs:+32:28: +32:29
277         _46 = &raw mut (*_3);            // scope 12 at $DIR/address_of.rs:+32:28: +32:29
-         _45 = move _46 as RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[a28e]::marker::Send)), [])], ReErased, Dyn), mutbl: Mut }) (Pointer(Unsize)); // scope 12 at $DIR/address_of.rs:+32:28: +32:29
+         _45 = move _46 as RawPtr(TypeAndMut { ty: Dynamic([Binder(AutoTrait(DefId(2:2662 ~ core[5618]::marker::Send)), [])], ReErased, Dyn), mutbl: Mut }) (Pointer(Unsize)); // scope 12 at $DIR/address_of.rs:+32:28: +32:29
279         StorageDead(_46);                // scope 12 at $DIR/address_of.rs:+32:28: +32:29
280         FakeRead(ForLet(None), _45);     // scope 12 at $DIR/address_of.rs:+32:9: +32:10
281         AscribeUserType(_45, o, UserTypeProjection { base: UserType(27), projs: [] }); // scope 12 at $DIR/address_of.rs:+32:12: +32:25

thread '[mir-opt] tests/mir-opt/address_of.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/address_of.address_of_reborrow.SimplifyCfg-initial.after.mir', src/tools/compiletest/src/runtest.rs:3463:21

---- [mir-opt] tests/mir-opt/fn_ptr_shim.rs stdout ----
---- [mir-opt] tests/mir-opt/fn_ptr_shim.rs stdout ----
1 // MIR for `std::ops::Fn::call` before AddMovesForPackedDrops
2 
3 fn std::ops::Fn::call(_1: *const fn(), _2: ()) -> <fn() as FnOnce<()>>::Output {
-     let mut _0: Alias(Projection, AliasTy { substs: [FnPtr(Binder(([]; c_variadic: false)->Tuple([]), [])), Tuple([])], def_id: DefId(2:2940 ~ core[a28e]::ops::function::FnOnce::Output) }); // return place in scope 0 at $SRC_DIR/core/src/ops/function.rs:+0:5: +0:67
+     let mut _0: Alias(Projection, AliasTy { substs: [FnPtr(Binder(([]; c_variadic: false)->Tuple([]), [])), Tuple([])], def_id: DefId(2:2940 ~ core[5618]::ops::function::FnOnce::Output) }); // return place in scope 0 at $SRC_DIR/core/src/ops/function.rs:+0:5: +0:67
6     bb0: {
6     bb0: {
7         _0 = move (*_1)() -> bb1;        // scope 0 at $SRC_DIR/core/src/ops/function.rs:+0:5: +0:67

thread '[mir-opt] tests/mir-opt/fn_ptr_shim.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/fn_ptr_shim.core.ops-function-Fn-call.AddMovesForPackedDrops.before.mir', src/tools/compiletest/src/runtest.rs:3463:21
---- [mir-opt] tests/mir-opt/issue_91633.rs stdout ----
---- [mir-opt] tests/mir-opt/issue_91633.rs stdout ----
3 fn hey(_1: &[T]) -> () {
4     debug it => _1;                      // in scope 0 at $DIR/issue_91633.rs:+0:12: +0:14
5     let mut _0: Tuple([]);               // return place in scope 0 at $DIR/issue_91633.rs:+1:2: +1:2
-     let mut _2: Ref(ReErased, Alias(Projection, AliasTy { substs: [Slice(Param(T/#0)), Uint(Usize)], def_id: DefId(2:2989 ~ core[a28e]::ops::index::Index::Output) }), Not); // in scope 0 at $DIR/issue_91633.rs:+4:14: +4:20
-     let _3: Ref(ReErased, Alias(Projection, AliasTy { substs: [Slice(Param(T/#0)), Uint(Usize)], def_id: DefId(2:2989 ~ core[a28e]::ops::index::Index::Output) }), Not); // in scope 0 at $DIR/issue_91633.rs:+4:15: +4:20
+     let mut _2: Ref(ReErased, Alias(Projection, AliasTy { substs: [Slice(Param(T/#0)), Uint(Usize)], def_id: DefId(2:2989 ~ core[5618]::ops::index::Index::Output) }), Not); // in scope 0 at $DIR/issue_91633.rs:+4:14: +4:20
+     let _3: Ref(ReErased, Alias(Projection, AliasTy { substs: [Slice(Param(T/#0)), Uint(Usize)], def_id: DefId(2:2989 ~ core[5618]::ops::index::Index::Output) }), Not); // in scope 0 at $DIR/issue_91633.rs:+4:15: +4:20
8     let mut _4: Ref(ReErased, Slice(Param(T/#0)), Not); // in scope 0 at $DIR/issue_91633.rs:+4:15: +4:17
9     scope 1 {


thread '[mir-opt] tests/mir-opt/issue_91633.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/issue_91633.hey.built.after.mir', src/tools/compiletest/src/runtest.rs:3463:21
---- [mir-opt] tests/mir-opt/lower_intrinsics.rs stdout ----
3   
4   fn non_const() -> usize {
4   fn non_const() -> usize {
5       let mut _0: Uint(Usize);             // return place in scope 0 at $DIR/lower_intrinsics.rs:+0:26: +0:31
-       let _1: FnDef(DefId(2:1271 ~ core[a28e]::intrinsics::{extern#0}::size_of), [Param(T/#0)]); // in scope 0 at $DIR/lower_intrinsics.rs:+2:9: +2:18
-       let mut _2: FnDef(DefId(2:1271 ~ core[a28e]::intrinsics::{extern#0}::size_of), [Param(T/#0)]); // in scope 0 at $DIR/lower_intrinsics.rs:+3:5: +3:14
+       let _1: FnDef(DefId(2:1271 ~ core[5618]::intrinsics::{extern#0}::size_of), [Param(T/#0)]); // in scope 0 at $DIR/lower_intrinsics.rs:+2:9: +2:18
+       let mut _2: FnDef(DefId(2:1271 ~ core[5618]::intrinsics::{extern#0}::size_of), [Param(T/#0)]); // in scope 0 at $DIR/lower_intrinsics.rs:+3:5: +3:14
8       scope 1 {
9           debug size_of_t => _1;           // in scope 1 at $DIR/lower_intrinsics.rs:+2:9: +2:18


thread '[mir-opt] tests/mir-opt/lower_intrinsics.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/lower_intrinsics.non_const.LowerIntrinsics.diff', src/tools/compiletest/src/runtest.rs:3463:21

failures:
    [mir-opt] tests/mir-opt/address_of.rs
    [mir-opt] tests/mir-opt/fn_ptr_shim.rs
