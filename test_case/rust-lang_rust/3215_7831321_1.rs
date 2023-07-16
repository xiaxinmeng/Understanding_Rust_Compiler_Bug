
280         for fields.each |field| {
281             match field_map.find(field.ident) {
282                 some(index) => {
284                     let field_type = ty::lookup_field_type(tcx,
285                                                            class_id,
286                                                            class_field.id,
287                                                            substitutions);
