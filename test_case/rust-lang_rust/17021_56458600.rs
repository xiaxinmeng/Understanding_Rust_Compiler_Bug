
        match ty::get(type_).sty {
            ty::ty_nil      |
            ty::ty_bot      |
            ty::ty_bool     |
(...snipped...)
            _ => {
                cx.sess().bug(format!("get_unique_type_id_of_type() - unexpected type: {}, {:?}",
                                      ppaux::ty_to_string(cx.tcx(), type_).as_slice(),
                                      ty::get(type_).sty).as_slice())
            }
        };
