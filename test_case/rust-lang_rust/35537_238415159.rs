 rust
        let mut err = match warning_node_id {
            Some(_) => None,
            None => {
                let trait_str = self.item_path_str(trait_def_id);
                let mut db = struct_span_err!(
                            self.sess, span, E0038,
                            "the trait `{}` cannot be made into an object",
                            trait_str);
                db.span_label(span,
                            &format!("the trait `{}` cannot be made \
                            into an object", trait_str));
                Some(db)
            }
        };
