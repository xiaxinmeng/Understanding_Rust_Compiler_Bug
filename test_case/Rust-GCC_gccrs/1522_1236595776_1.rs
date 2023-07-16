
;; Function const_eval_oeis_a006884::test (_ZN23const_eval_oeis_a0068844test17h64d1733dc533ef5cE, funcdef_no=0, decl_uid=74, cgraph_uid=1, symbol_order=0)

Folding predicate 1 != 0 to 1
Removing basic block 4
Removing basic block 5
Removing basic block 6
Removing basic block 7
Merging blocks 2 and 3
Merging blocks 2 and 8
__attribute__((cdecl))
u64 const_eval_oeis_a006884::test ()
{
  u64 n;

  <bb 2> :
  gimple_return <113383>

}



;; Function const_eval_oeis_a006884::test_1 (_ZN23const_eval_oeis_a0068846test_117h64d1733dc533ef5cE, funcdef_no=1, decl_uid=97, cgraph_uid=2, symbol_order=1)

__attribute__((cdecl))
u64 const_eval_oeis_a006884::test_1 ()
{
  u64 _1;

  <bb 2> :
  gimple_call <const_eval_oeis_a006884::test, _1>
  gimple_return <_1>

}
