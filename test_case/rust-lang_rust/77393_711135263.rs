rust
        self.encode_ident_span(def_id, ...);
        record!(self.tables.visibility[def_id] <-
            ty::Visibility::from_hir(...));
        record!(self.tables.attributes[def_id] <- ...);
        self.encode_stability(def_id);
        self.encode_const_stability(def_id);
        self.encode_deprecation(def_id);
        self.encode_item_type_when(def_id, ...);
        self.encode_inherent_implementations(def_id);
        self.encode_function_signature_when(
            def_id,
            matches!(...),
        );
        self.encode_generics(def_id);
        self.encode_variances_when(def_id, matches!(...));
        self.encode_explicit_predicates(def_id);
        self.encode_generics_explicit_predicates_inferred_outlives_when(def_id, ...);
