
------------------------ROUND 1------------------------------
InferVisitor::visit_item(item=OwnerId { def_id: DefId(0:7 ~ issue_102966[6f77]::Kind) })
    (rebound/substituted = OutlivesPredicate(<KG as NodeGenerics<'kind>>::R, ReEarlyBound(0, 'kind)))
------------------------ROUND 2------------------------------
InferVisitor::visit_item(item=OwnerId { def_id: DefId(0:3 ~ issue_102966[6f77]::Node) })
    unsubstituted pred   OutlivesPredicate(<KG as NodeGenerics<'kind>>::R, ReEarlyBound(0, 'kind))
    (rebound/substituted OutlivesPredicate(<NG as NodeGenerics<'node>>::R, ReEarlyBound(0, 'node)))
InferVisitor::visit_item(item=OwnerId { def_id: DefId(0:26 ~ issue_102966[6f77]::Var) })
    unsubstituted pred   OutlivesPredicate(<NG as NodeGenerics<'node>>::R, ReEarlyBound(0, 'node))
    (rebound/substituted OutlivesPredicate(<RGen<R> as NodeGenerics<'var>>::R, ReEarlyBound(0, 'var)))
------------------------ROUND 3------------------------------
InferVisitor::visit_item(item=OwnerId { def_id: DefId(0:7 ~ issue_102966[6f77]::Kind) })
    unsubstituted pred   OutlivesPredicate(<RGen<R> as NodeGenerics<'var>>::R, ReEarlyBound(0, 'var))
    (rebound/substituted OutlivesPredicate(<RGen<<KG as NodeGenerics<'kind>>::R> as NodeGenerics<'kind>>::R, ReEarlyBound(0, 'kind)))
------------------------ROUND 4------------------------------
InferVisitor::visit_item(item=OwnerId { def_id: DefId(0:3 ~ issue_102966[6f77]::Node) })
    unsubstituted pred   OutlivesPredicate(<RGen<<KG as NodeGenerics<'kind>>::R> as NodeGenerics<'kind>>::R, ReEarlyBound(0, 'kind))
    (rebound/substituted OutlivesPredicate(<RGen<<NG as NodeGenerics<'node>>::R> as NodeGenerics<'node>>::R, ReEarlyBound(0, 'node)))
InferVisitor::visit_item(item=OwnerId { def_id: DefId(0:26 ~ issue_102966[6f77]::Var) })
    unsubstituted pred   OutlivesPredicate(<RGen<<NG as NodeGenerics<'node>>::R> as NodeGenerics<'node>>::R, ReEarlyBound(0, 'node))
    (rebound/substituted OutlivesPredicate(<RGen<<RGen<R> as NodeGenerics<'var>>::R> as NodeGenerics<'var>>::R, ReEarlyBound(0, 'var)))
