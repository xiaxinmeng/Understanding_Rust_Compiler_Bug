rust
        let is_write_macro = sugg_span.ctxt().outer_expn_data().kind
            == ExpnKind::Macro(MacroKind::Bang, Symbol::intern("write"));
        let mut err;
        let is_write_fmt = item_name.name == Symbol::intern("write_fmt");
        if is_write_fmt && is_write_macro && args.is_some() {
            let helper = tcx.sess.source_map().span_to_snippet(args.unwrap().0.span).unwrap();
            let writer_note;
            match args.unwrap().0.kind {
                ExprKind::Lit(_) => {
                    writer_note = Some("you might want to insert a writer in front of this format string")
                }
                _ => writer_note = None,
            };
            err = struct_span_err!(
                tcx.sess,
                args.unwrap().0.span,
                E0599,
                "cannot write into '{}'",
                ty_str_reported,
            );
            err.note(format!(
                "{} must implement 'io::Write', 'fmt::Write' or have a 'write_fmt' method",
                helper
            ));
            if writer_note.is_some() {
                err.help(writer_note.unwrap());
            }
        }
