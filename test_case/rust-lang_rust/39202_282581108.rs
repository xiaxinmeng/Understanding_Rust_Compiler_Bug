
error: no field `map` on type `rustc::ty::TyCtxt<'_, '_, '_>`

   --> /checkout/src/librustc_lint/unused.rs:194:36

    |

194 |             let parent_id = cx.tcx.map.get_parent_node(id);

    |                                    ^^^ unknown field

error: no field `map` on type `rustc::ty::TyCtxt<'_, '_, '_>`

   --> /checkout/src/librustc_lint/unused.rs:201:30

    |

201 |                 })) = cx.tcx.map.find(parent_id) {

    |                              ^^^ unknown field

error: no field `map` on type `rustc::ty::TyCtxt<'_, '_, '_>`

   --> /checkout/src/librustc_lint/unused.rs:219:41

    |

219 |                     db.span_note(cx.tcx.map.span(id),

    |                                         ^^^ unknown field

