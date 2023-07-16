
Node::PathSegment(seg) => {
    if seg.res != Res::Err {
        seg.res
    } else {
        let parent_node = self.tcx.hir().get_parent_node(hir_id);
        self.get_path_res(parent_node)
    }   
}   
