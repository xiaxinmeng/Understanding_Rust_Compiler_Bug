
         ResolutionError::FailedToResolve(msg) => {
            struct_span_err!(resolver.session, span, E0433, "failed to resolve. {}", msg)
            let mut err = struct_span_err!(resolver.session, span, E0433, "failed to resolve. {}", msg);
            err.span_label(span, &msg);
            err
         }
