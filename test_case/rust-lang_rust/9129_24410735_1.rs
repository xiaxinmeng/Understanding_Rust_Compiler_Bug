
        ExprBreak(ref opt_ident) => {
            debug!("fold ExprBreak input: %?", opt_ident);
            let ret = ExprBreak(opt_ident.map_move(|x| {
                        // let i1 = Ident::new(x);
                        let i2 = fld.fold_ident(Ident::new(x)); // *** 1
                        i2.name
                    }));
            debug!("fold ExprBreak output: %?", ret);
            ret
        }
        ExprAgain(ref opt_ident) => {
            debug!("fold ExprAgain input: %?", opt_ident);
            let ret = ExprAgain(opt_ident.map_move(|x| {
                        let i1 = Ident::new(x);
                        let i2 = fld.fold_ident(i1); // *** 2
                        i2.name
                    }));
            debug!("fold ExprAgain input: %?", ret);
            ret
        }
