
                           err.multipart_suggestion(
                                message,
                                vec![
                                    (trait_path_segment.ident.span.shrink_to_lo(), format!("<{} as ", self.tcx.def_path(impl_def_id).to_string_no_crate_verbose())),
                                   // (trait_path_segment.ident.span.shrink_to_hi(), format!(">"))
                                    (generic_arg.span_ext.shrink_to_hi(), format!(">")),
                                ],
                                Applicability::MaybeIncorrect
                            );

