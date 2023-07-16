
        struct $struct_name {
                $(
                        $(
                                $(#[$field_meta])* // yes, from the parent level
                                $field_name : $getter_ty
                        )?
                ),*
        }
