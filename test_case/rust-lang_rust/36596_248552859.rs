 rust
        // FIXME: do we want to use a different error code for each origin?
        let mut diag = struct_span_err!(
            self.tcx.sess, trace.origin.span(), E0308,
            "{}", trace.origin.as_failure_str()
        );
