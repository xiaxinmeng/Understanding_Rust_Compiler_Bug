rust
                            match (decl.mutability, is_local_mutation_allowed) {
                                (Mutability::Not, LocalMutationIsAllowed::No)
                                | (Mutability::Not, LocalMutationIsAllowed::ExceptUpvars) => {
                                    Err(place)
                                }
                                (Mutability::Not, LocalMutationIsAllowed::Yes)
                                | (Mutability::Mut, _) => {
                                    let _ = self.is_mutable(&proj.base, is_local_mutation_allowed)?;
                                    Ok((place, is_local_mutation_allowed))
                                }
                            }
