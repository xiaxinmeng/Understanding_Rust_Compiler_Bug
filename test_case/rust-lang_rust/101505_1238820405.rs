
Node::PathSegment(seg) => match seg.res {
    Some(res) if res != Res::Err => res,
    _ => { 
        let parent_node = self.tcx.hir().get_parent_node(hir_id);
        self.get_path_res(parent_node)
    }
},  
