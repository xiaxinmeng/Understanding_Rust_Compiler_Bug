
        ExprBreak(ref opt_ident) => {
            // FIXME #6993: add fold_name to fold.... then cut out the
            // bogus Name->Ident->Name conversion.
            ExprBreak(opt_ident.map_move(|x| fld.fold_ident(Ident::new(x)).name))
        }
        ExprAgain(ref opt_ident) => {
            // FIXME #6993: add fold_name to fold....
            ExprAgain(opt_ident.map_move(|x| fld.fold_ident(Ident::new(x)).name))
        }
