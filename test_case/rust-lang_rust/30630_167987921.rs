 rust
                // to start, translate the argument patterns and collect the
                // argument types.
                let explicit_arg_decls =
                    explicit_arguments
                    .into_iter()
                    .map(|(ty, pat)| (ty, Some(pat)))
                    .chain(implicit_arguments.into_iter().map(|ty| (ty, None)))
                    .enumerate()
                    .map(|(index, (ty, pattern))| {
                        if let Some(pattern) = pattern {
                             let lvalue = Lvalue::Arg(index as u32);
                             let pattern = this.hir.irrefutable_pat(pattern);
                             unpack!(block = this.lvalue_into_pattern(block,
                                                                      argument_extent,
                                                                      pattern,
                                                                      &lvalue));
                        }
                        ArgDecl { ty: ty }
                    });
