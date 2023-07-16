
            ptr::copy_nonoverlapping(
                self.node.edge_at(self.idx + 1),
                MaybeUninit::slice_as_mut_ptr(&mut new_node.edges),
                new_len + 1,
            );
