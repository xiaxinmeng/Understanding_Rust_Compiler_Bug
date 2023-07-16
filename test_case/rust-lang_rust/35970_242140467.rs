 rust
    fn get_type_parameter_bounds(&self, span: Span, def_id: ast::NodeId)
                                 -> Result<Vec<ty::PolyTraitRef<'tcx>>, ErrorReported>;
