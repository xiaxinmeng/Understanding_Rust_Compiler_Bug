console
ai: [ExpectedIdx(0), ExpectedIdx(1), ExpectedIdx(2), ExpectedIdx(3), ExpectedIdx(4), ExpectedIdx(5)]
ii: [ProvidedIdx(0), ProvidedIdx(1), ProvidedIdx(2), ProvidedIdx(3), ProvidedIdx(4), ProvidedIdx(5)]
================== find_issue: ==================
x|  0  1  2  3  4  5
-|  -  -  -  -  -  -
0|  0  0  0  0  1  1
1|  0  0  0  0  1  1
2|  1  1  0  0  0  0
3|  1  1  0  0  0  0
4|  0  0  1  1  0  0
5|  0  0  1  1  0  0
i: 0, is_input: true, is_arg: true, useless: false, unsatisfiable: false
i: 1, is_input: true, is_arg: true, useless: false, unsatisfiable: false
i: 2, is_input: true, is_arg: true, useless: false, unsatisfiable: false
i: 3, is_input: true, is_arg: true, useless: false, unsatisfiable: false
i: 4, is_input: true, is_arg: true, useless: false, unsatisfiable: false
i: 5, is_input: true, is_arg: true, useless: false, unsatisfiable: false
loop i: 0 j: 0 compat: [4, 5]
permutation_found: false
now permutation: [Some(None), None, None, None, None, None]
loop i: 1 j: 1 compat: [4, 5]
permutation_found: false
now permutation: [Some(None), Some(None), None, None, None, None]
loop i: 2 j: 2 compat: [0, 1]
permutation_found: false
now permutation: [Some(None), Some(None), Some(None), None, None, None]
loop i: 3 j: 3 compat: [0, 1]
permutation_found: false
now permutation: [Some(None), Some(None), Some(None), Some(None), None, None]
loop i: 4 j: 4 compat: [2, 3]
permutation_found: false
now permutation: [Some(None), Some(None), Some(None), Some(None), Some(None), None]
loop i: 5 j: 5 compat: [2, 3]
permutation_found: false
now permutation: [Some(None), Some(None), Some(None), Some(None), Some(None), Some(None)]
res: None
thread 'rustc' panicked at 'didn't eliminated any indice in this round', compiler/rustc_typeck/src/check/fn_ctxt/arg_matrix.rs:412:21
