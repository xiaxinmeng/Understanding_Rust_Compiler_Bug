
 let maybe_intern: &'self Option<&Intern> = do arc_interner.write |interner| {
                        interner.str_to_intern.find(&burrowed.to_str())
                    };
