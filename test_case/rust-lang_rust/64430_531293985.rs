rust
        if let Some(mut err) = self.report_method_error(
            span,
            rcvr_t,
            segment.ident,
            SelfSource::MethodCall(rcvr),
            error,
            Some(args),
        ) {
            if let ty::Adt(..) = rcvr_t.sty {
                // Try alternative arbitrary self types that could fulfill this call.
                // FIXME: probe for all types that *could* be arbitrary self-types, not
                // just this whitelist.
                let box_rcvr_t = self.tcx.mk_box(rcvr_t);
                try_alt_rcvr(&mut err, box_rcvr_t);
                let pin_rcvr_t = self.tcx.mk_lang_item(
                    rcvr_t,
                    lang_items::PinTypeLangItem,
                );
                try_alt_rcvr(&mut err, pin_rcvr_t);
                let arc_rcvr_t = self.tcx.mk_lang_item(rcvr_t, lang_items::Arc);
                try_alt_rcvr(&mut err, arc_rcvr_t);
                let rc_rcvr_t = self.tcx.mk_lang_item(rcvr_t, lang_items::Rc);
                try_alt_rcvr(&mut err, rc_rcvr_t);
            }
            err.emit();
        }
