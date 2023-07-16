
DEBUG:rustc_typeck::check::dropck: check_safety_of_destructor_if_necessary typ: FingerTree<i32> scope: Remainder(BlockRemainder { block: 37, first_statement_index: 0 })
DEBUG:rustc_typeck::check::dropck: iterate_over_potentially_unsafe_regions_in_type typ: FingerTree<i32> scope: Remainder(BlockRemainder { block: 37, first_statement_index: 0 }) opt_dtor: None
DEBUG:rustc_typeck::check::dropck: typ: FingerTree<i32> has no dtor, and thus is uninteresting
DEBUG:rustc_typeck::check::dropck: iterate_over_potentially_unsafe_regions_in_type  typ: i32 scope: Remainder(BlockRemainder { block: 37, first_statement_index: 0 }) opt_dtor: None
DEBUG:rustc_typeck::check::dropck: typ: i32 has no dtor, and thus is uninteresting
DEBUG:rustc_typeck::check::dropck: iterate_over_potentially_unsafe_regions_in_type  typ: Box<FingerTree<Node<i32>>> scope: Remainder(BlockRemainder { block: 37, first_statement_index: 0 }) opt_dtor: None
DEBUG:rustc_typeck::check::dropck: typ: Box<FingerTree<Node<i32>>> has no dtor, and thus is uninteresting
DEBUG:rustc_typeck::check::dropck: iterate_over_potentially_unsafe_regions_in_type  typ: FingerTree<Node<i32>> scope: Remainder(BlockRemainder { block: 37, first_statement_index: 0 }) opt_dtor: None
DEBUG:rustc_typeck::check::dropck: typ: FingerTree<Node<i32>> has no dtor, and thus is uninteresting
DEBUG:rustc_typeck::check::dropck: iterate_over_potentially_unsafe_regions_in_type   typ: Node<i32> scope: Remainder(BlockRemainder { block: 37, first_statement_index: 0 }) opt_dtor: None
DEBUG:rustc_typeck::check::dropck: typ: Node<i32> has no dtor, and thus is uninteresting
DEBUG:rustc_typeck::check::dropck: iterate_over_potentially_unsafe_regions_in_type   typ: Box<FingerTree<Node<Node<i32>>>> scope: Remainder(BlockRemainder { block: 37, first_statement_index: 0 }) opt_dtor: None
DEBUG:rustc_typeck::check::dropck: typ: Box<FingerTree<Node<Node<i32>>>> has no dtor, and thus is uninteresting
DEBUG:rustc_typeck::check::dropck: iterate_over_potentially_unsafe_regions_in_type   typ: FingerTree<Node<Node<i32>>> scope: Remainder(BlockRemainder { block: 37, first_statement_index: 0 }) opt_dtor: None
DEBUG:rustc_typeck::check::dropck: typ: FingerTree<Node<Node<i32>>> has no dtor, and thus is uninteresting
DEBUG:rustc_typeck::check::dropck: iterate_over_potentially_unsafe_regions_in_type    typ: Node<Node<i32>> scope: Remainder(BlockRemainder { block: 37, first_statement_index: 0 }) opt_dtor: None
DEBUG:rustc_typeck::check::dropck: typ: Node<Node<i32>> has no dtor, and thus is uninteresting
DEBUG:rustc_typeck::check::dropck: iterate_over_potentially_unsafe_regions_in_type    typ: Box<FingerTree<Node<Node<Node<i32>>>>> scope: Remainder(BlockRemainder { block: 37, first_statement_index: 0 }) opt_dtor: None
DEBUG:rustc_typeck::check::dropck: typ: Box<FingerTree<Node<Node<Node<i32>>>>> has no dtor, and thus is uninteresting
DEBUG:rustc_typeck::check::dropck: iterate_over_potentially_unsafe_regions_in_type    typ: FingerTree<Node<Node<Node<i32>>>> scope: Remainder(BlockRemainder { block: 37, first_statement_index: 0 }) opt_dtor: None
DEBUG:rustc_typeck::check::dropck: typ: FingerTree<Node<Node<Node<i32>>>> has no dtor, and thus is uninteresting
DEBUG:rustc_typeck::check::dropck: iterate_over_potentially_unsafe_regions_in_type     typ: Node<Node<Node<i32>>> scope: Remainder(BlockRemainder { block: 37, first_statement_index: 0 }) opt_dtor: None
DEBUG:rustc_typeck::check::dropck: typ: Node<Node<Node<i32>>> has no dtor, and thus is uninteresting
DEBUG:rustc_typeck::check::dropck: iterate_over_potentially_unsafe_regions_in_type     typ: Box<FingerTree<Node<Node<Node<Node<i32>>>>>> scope: Remainder(BlockRemainder { block: 37, first_statement_index: 0 }) opt_dtor: None
DEBUG:rustc_typeck::check::dropck: typ: Box<FingerTree<Node<Node<Node<Node<i32>>>>>> has no dtor, and thus is uninteresting
DEBUG:rustc_typeck::check::dropck: iterate_over_potentially_unsafe_regions_in_type     typ: FingerTree<Node<Node<Node<Node<i32>>>>> scope: Remainder(BlockRemainder { block: 37, first_statement_index: 0 }) opt_dtor: None
DEBUG:rustc_typeck::check::dropck: typ: FingerTree<Node<Node<Node<Node<i32>>>>> has no dtor, and thus is uninteresting
DEBUG:rustc_typeck::check::dropck: iterate_over_potentially_unsafe_regions_in_type      typ: Node<Node<Node<Node<i32>>>> scope: Remainder(BlockRemainder { block: 37, first_statement_index: 0 }) opt_dtor: None
DEBUG:rustc_typeck::check::dropck: typ: Node<Node<Node<Node<i32>>>> has no dtor, and thus is uninteresting
DEBUG:rustc_typeck::check::dropck: iterate_over_potentially_unsafe_regions_in_type      typ: Box<FingerTree<Node<Node<Node<Node<Node<i32>>>>>>> scope: Remainder(BlockRemainder { block: 37, first_statement_index: 0 }) opt_dtor: None
DEBUG:rustc_typeck::check::dropck: typ: Box<FingerTree<Node<Node<Node<Node<Node<i32>>>>>>> has no dtor, and thus is uninteresting
DEBUG:rustc_typeck::check::dropck: iterate_over_potentially_unsafe_regions_in_type      typ: FingerTree<Node<Node<Node<Node<Node<i32>>>>>> scope: Remainder(BlockRemainder { block: 37, first_statement_index: 0 }) opt_dtor: None
DEBUG:rustc_typeck::check::dropck: typ: FingerTree<Node<Node<Node<Node<Node<i32>>>>>> has no dtor, and thus is uninteresting
DEBUG:rustc_typeck::check::dropck: iterate_over_potentially_unsafe_regions_in_type       typ: Node<Node<Node<Node<Node<i32>>>>> scope: Remainder(BlockRemainder { block: 37, first_statement_index: 0 }) opt_dtor: None
DEBUG:rustc_typeck::check::dropck: typ: Node<Node<Node<Node<Node<i32>>>>> has no dtor, and thus is uninteresting
DEBUG:rustc_typeck::check::dropck: iterate_over_potentially_unsafe_regions_in_type       typ: Box<FingerTree<Node<Node<Node<Node<Node<Node<i32>>>>>>>> scope: Remainder(BlockRemainder { block: 37, first_statement_index: 0 }) opt_dtor: None
DEBUG:rustc_typeck::check::dropck: typ: Box<FingerTree<Node<Node<Node<Node<Node<Node<i32>>>>>>>> has no dtor, and thus is uninteresting
DEBUG:rustc_typeck::check::dropck: iterate_over_potentially_unsafe_regions_in_type       typ: FingerTree<Node<Node<Node<Node<Node<Node<i32>>>>>>> scope: Remainder(BlockRemainder { block: 37, first_statement_index: 0 }) opt_dtor: None
DEBUG:rustc_typeck::check::dropck: typ: FingerTree<Node<Node<Node<Node<Node<Node<i32>>>>>>> has no dtor, and thus is uninteresting
DEBUG:rustc_typeck::check::dropck: iterate_over_potentially_unsafe_regions_in_type        typ: Node<Node<Node<Node<Node<Node<i32>>>>>> scope: Remainder(BlockRemainder { block: 37, first_statement_index: 0 }) opt_dtor: None
DEBUG:rustc_typeck::check::dropck: typ: Node<Node<Node<Node<Node<Node<i32>>>>>> has no dtor, and thus is uninteresting
DEBUG:rustc_typeck::check::dropck: iterate_over_potentially_unsafe_regions_in_type        typ: Box<FingerTree<Node<Node<Node<Node<Node<Node<Node<i32>>>>>>>>> scope: Remainder(BlockRemainder { block: 37, first_statement_index: 0 }) opt_dtor: None
DEBUG:rustc_typeck::check::dropck: typ: Box<FingerTree<Node<Node<Node<Node<Node<Node<Node<i32>>>>>>>>> has no dtor, and thus is uninteresting
DEBUG:rustc_typeck::check::dropck: iterate_over_potentially_unsafe_regions_in_type        typ: FingerTree<Node<Node<Node<Node<Node<Node<Node<i32>>>>>>>> scope: Remainder(BlockRemainder { block: 37, first_statement_index: 0 }) opt_dtor: None
DEBUG:rustc_typeck::check::dropck: typ: FingerTree<Node<Node<Node<Node<Node<Node<Node<i32>>>>>>>> has no dtor, and thus is uninteresting
DEBUG:rustc_typeck::check::dropck: iterate_over_potentially_unsafe_regions_in_type         typ: Node<Node<Node<Node<Node<Node<Node<i32>>>>>>> scope: Remainder(BlockRemainder { block: 37, first_statement_index: 0 }) opt_dtor: None
DEBUG:rustc_typeck::check::dropck: typ: Node<Node<Node<Node<Node<Node<Node<i32>>>>>>> has no dtor, and thus is uninteresting
DEBUG:rustc_typeck::check::dropck: iterate_over_potentially_unsafe_regions_in_type         typ: Box<FingerTree<Node<Node<Node<Node<Node<Node<Node<Node<i32>>>>>>>>>> scope: Remainder(BlockRemainder { block: 37, first_statement_index: 0 }) opt_dtor: None
DEBUG:rustc_typeck::check::dropck: typ: Box<FingerTree<Node<Node<Node<Node<Node<Node<Node<Node<i32>>>>>>>>>> has no dtor, and thus is uninteresting
DEBUG:rustc_typeck::check::dropck: iterate_over_potentially_unsafe_regions_in_type         typ: FingerTree<Node<Node<Node<Node<Node<Node<Node<Node<i32>>>>>>>>> scope: Remainder(BlockRemainder { block: 37, first_statement_index: 0 }) opt_dtor: None
DEBUG:rustc_typeck::check::dropck: typ: FingerTree<Node<Node<Node<Node<Node<Node<Node<Node<i32>>>>>>>>> has no dtor, and thus is uninteresting
DEBUG:rustc_typeck::check::dropck: iterate_over_potentially_unsafe_regions_in_type          typ: Node<Node<Node<Node<Node<Node<Node<Node<i32>>>>>>>> scope: Remainder(BlockRemainder { block: 37, first_statement_index: 0 }) opt_dtor: None
DEBUG:rustc_typeck::check::dropck: typ: Node<Node<Node<Node<Node<Node<Node<Node<i32>>>>>>>> has no dtor, and thus is uninteresting
DEBUG:rustc_typeck::check::dropck: iterate_over_potentially_unsafe_regions_in_type          typ: Box<FingerTree<Node<Node<Node<Node<Node<Node<Node<Node<Node<i32>>>>>>>>>>> scope: Remainder(BlockRemainder { block: 37, first_statement_index: 0 }) opt_dtor: None
DEBUG:rustc_typeck::check::dropck: typ: Box<FingerTree<Node<Node<Node<Node<Node<Node<Node<Node<Node<i32>>>>>>>>>>> has no dtor, and thus is uninteresting
DEBUG:rustc_typeck::check::dropck: iterate_over_potentially_unsafe_regions_in_type          typ: FingerTree<Node<Node<Node<Node<Node<Node<Node<Node<Node<i32>>>>>>>>>> scope: Remainder(BlockRemainder { block: 37, first_statement_index: 0 }) opt_dtor: None
DEBUG:rustc_typeck::check::dropck: typ: FingerTree<Node<Node<Node<Node<Node<Node<Node<Node<Node<i32>>>>>>>>>> has no dtor, and thus is uninteresting
DEBUG:rustc_typeck::check::dropck: iterate_over_potentially_unsafe_regions_in_type           typ: Node<Node<Node<Node<Node<Node<Node<Node<Node<i32>>>>>>>>> scope: Remainder(BlockRemainder { block: 37, first_statement_index: 0 }) opt_dtor: None
DEBUG:rustc_typeck::check::dropck: typ: Node<Node<Node<Node<Node<Node<Node<Node<Node<i32>>>>>>>>> has no dtor, and thus is uninteresting
DEBUG:rustc_typeck::check::dropck: iterate_over_potentially_unsafe_regions_in_type           typ: Box<FingerTree<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<i32>>>>>>>>>>>> scope: Remainder(BlockRemainder { block: 37, first_statement_index: 0 }) opt_dtor: None
DEBUG:rustc_typeck::check::dropck: typ: Box<FingerTree<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<i32>>>>>>>>>>>> has no dtor, and thus is uninteresting
DEBUG:rustc_typeck::check::dropck: iterate_over_potentially_unsafe_regions_in_type           typ: FingerTree<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<i32>>>>>>>>>>> scope: Remainder(BlockRemainder { block: 37, first_statement_index: 0 }) opt_dtor: None
DEBUG:rustc_typeck::check::dropck: typ: FingerTree<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<i32>>>>>>>>>>> has no dtor, and thus is uninteresting
DEBUG:rustc_typeck::check::dropck: iterate_over_potentially_unsafe_regions_in_type            typ: Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<i32>>>>>>>>>> scope: Remainder(BlockRemainder { block: 37, first_statement_index: 0 }) opt_dtor: None
DEBUG:rustc_typeck::check::dropck: typ: Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<i32>>>>>>>>>> has no dtor, and thus is uninteresting
DEBUG:rustc_typeck::check::dropck: iterate_over_potentially_unsafe_regions_in_type            typ: Box<FingerTree<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<i32>>>>>>>>>>>>> scope: Remainder(BlockRemainder { block: 37, first_statement_index: 0 }) opt_dtor: None
DEBUG:rustc_typeck::check::dropck: typ: Box<FingerTree<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<i32>>>>>>>>>>>>> has no dtor, and thus is uninteresting
DEBUG:rustc_typeck::check::dropck: iterate_over_potentially_unsafe_regions_in_type            typ: FingerTree<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<i32>>>>>>>>>>>> scope: Remainder(BlockRemainder { block: 37, first_statement_index: 0 }) opt_dtor: None
DEBUG:rustc_typeck::check::dropck: typ: FingerTree<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<i32>>>>>>>>>>>> has no dtor, and thus is uninteresting
DEBUG:rustc_typeck::check::dropck: iterate_over_potentially_unsafe_regions_in_type             typ: Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<i32>>>>>>>>>>> scope: Remainder(BlockRemainder { block: 37, first_statement_index: 0 }) opt_dtor: None
DEBUG:rustc_typeck::check::dropck: typ: Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<Node<i32>>>>>>>>>>> has no dtor, and thus is uninteresting
