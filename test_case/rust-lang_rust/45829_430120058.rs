rust
           if let (Ok(snippet), false) = (cm.span_to_snippet(binding.span),
                                           binding.is_renamed_extern_crate()) {
