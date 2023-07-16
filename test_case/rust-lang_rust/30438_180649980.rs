
 838: DEBUG:rustc::middle::infer::region_inference: expansion: 
    constraint=ConstrainRegSubVar(ReFree(CodeExtent(7/CallSiteScope { fn_id: 15, body_id: 32 }), 
    BrNamed(DefId { krate: 0, node: DefIndex(11) => silly::'y }, 'y(76))), '_#11r) 
    origin=Subtype(TypeTrace(ExprAssignable(issue-30438-d.rs:11:5: 11:7)))
 839: DEBUG:rustc::middle::infer::region_inference: 
    expand_node(ReFree(CodeExtent(7/CallSiteScope{ fn_id: 15, body_id: 32 }),
    BrNamed(DefId { krate: 0, node: DefIndex(11) => silly::'y }, 'y(76))),
     '_#11r == Value(ReScope(CodeExtent(18/Misc(38)))))
 840: DEBUG:rustc::middle::infer::region_inference: given
