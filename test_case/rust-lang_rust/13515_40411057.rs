
        let did = match self.tcx.def_map.borrow().find_copy(&path_id) {
            // `int` etc. (None doesn't seem to occur.)
            None | Some(ast::DefPrimTy(..)) => return false,
            Some(def) => def_id_of_def(def)
        };
