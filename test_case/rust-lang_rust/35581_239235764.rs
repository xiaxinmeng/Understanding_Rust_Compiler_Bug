
            (true, _) | (_, true) if binding.is_import() || old_binding.is_import() => {
                let mut e = struct_span_err!(self.session, span, E0254, "{}", msg);
                e.span_label(span, &"already imported");
                e
            },
