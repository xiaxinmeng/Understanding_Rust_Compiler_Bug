
237         let substitutions;
238         match structure_of(fcx, pat.span, expected) {
239             ty::ty_class(cid, ref substs) => {
241                 substitutions = substs;
243             }
251         }
