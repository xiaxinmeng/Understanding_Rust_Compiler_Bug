 rust
        let struct_fields = cx.struct_fields.borrow();
        let mut results: SmallVector<&[field_ty]> = SmallVector::zero();
        each_super_struct(cx, did, |s| {
            match struct_fields.find(&s) {
                Some(fields) => results.push(fields.as_slice()),
                _ => {
                    cx.sess.bug(
                        format!("ID not mapped to struct fields: {}",
                                cx.map.node_to_string(did.node)).as_slice());
                }
            }
        });

        let len = results.as_slice().iter().map(|x| x.len()).sum();
