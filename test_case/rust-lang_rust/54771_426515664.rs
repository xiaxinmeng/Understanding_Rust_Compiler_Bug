
DEBUG 2018-10-03T09:41:07Z: rustc_typeck::check: >> typechecking: expr=expr(21: { 1u8; }) expected=ExpectHasType(_)
DEBUG 2018-10-03T09:41:07Z: rustc_typeck::check: >> typechecking: expr=expr(16: 1u8) expected=NoExpectation
DEBUG 2018-10-03T09:41:07Z: rustc_typeck::check: write_ty(HirId { owner: DefIndex(0:5), local_id: ItemLocalId(1) }, u8) in fcx 0x7ffef324d930
DEBUG 2018-10-03T09:41:07Z: rustc_typeck::check: type of expr 1u8 (id=16) is...
DEBUG 2018-10-03T09:41:07Z: rustc_typeck::check: ... u8, expected is NoExpectation
/// STOP!!!
DEBUG 2018-10-03T09:41:07Z: rustc_typeck::check: write_ty(HirId { owner: DefIndex(0:5), local_id: ItemLocalId(3) }, ()) in fcx 0x7ffef324d930
DEBUG 2018-10-03T09:41:07Z: rustc_typeck::check: write_ty(HirId { owner: DefIndex(0:5), local_id: ItemLocalId(4) }, ()) in fcx 0x7ffef324d930
DEBUG 2018-10-03T09:41:07Z: rustc_typeck::check: type of expr { 1u8; } (id=21) is...
DEBUG 2018-10-03T09:41:07Z: rustc_typeck::check: ... (), expected is ExpectHasType(_)
D
