
error: internal compiler error: /checkout/src/librustc/infer/region_constraints/mod.rs:685: cannot relate bound region: ReLateBound(DebruijnIndex { depth: 2 }, BrNamed(CrateNum(0):DefIndex(1:12), 'a(91))) <= ReSkolemized(0, BrNamed(CrateNum(0):DefIndex(1:12), 'a(91)))
  --> src/main.rs:34:5
   |
34 |     foo(Wrapper);
   |     ^^^
