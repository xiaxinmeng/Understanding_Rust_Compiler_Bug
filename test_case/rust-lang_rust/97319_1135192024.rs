
  let mut disr_vals: Vec<Discr<'tcx>> = Vec::with_capacity(vs.len());
  for ((_, discr), v) in iter::zip(def.discriminants(tcx), vs) {
      // Check for duplicate discriminant values
      if let Some(i) = disr_vals.iter().position(|&x| x.val == discr.val) {
          let variant_did = def.variant(VariantIdx::new(i)).def_id;
          let variant_i_hir_id = tcx.hir().local_def_id_to_hir_id(variant_did.expect_local());
          let variant_i = tcx.hir().expect_variant(variant_i_hir_id);
          let i_span = match variant_i.disr_expr {
              Some(ref expr) => tcx.hir().span(expr.hir_id),
              None => tcx.def_span(variant_did),
          };
          let span = match v.disr_expr {
              Some(ref expr) => tcx.hir().span(expr.hir_id),
              None => v.span,
          };
          let display_discr = display_discriminant_value(tcx, v, discr.val);
          let display_discr_i = display_discriminant_value(tcx, variant_i, disr_vals[i].val);
                                                                                                                  
          struct_span_err!(                                                  
              tcx.sess,                              
              span,                                                   
              E0081,
              "discriminant value `{}` already exists",
              discr.val,
          )
          .span_label(i_span, format!("first use of {display_discr_i}"))
          .span_label(span, format!("enum already has {display_discr}"))
          .emit();
      }
      disr_vals.push(discr);
  }

  check_representable(tcx, sp, def_id);
  check_transparent(tcx, sp, def);
