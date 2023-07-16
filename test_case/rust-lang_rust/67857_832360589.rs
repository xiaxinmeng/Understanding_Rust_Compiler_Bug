rust
>             for new_diag in &buffer.messages {
>                 diag.warn(&new_diag.message());
>                 for (span, label) in new_diag.span.labels {
>                     diag.span_note(span, label);
>                 }
>                 diag.suggestions.extend(new_diag.suggestions.clone());
>             }
> 