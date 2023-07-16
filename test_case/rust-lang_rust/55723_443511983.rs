rust

 // The span starts in the `///`, so we don't have to account for the leading whitespace
            let code_dox_len = if line_offset <= 1 {
                doc_comment_padding
            } else {
                // The first `///`
                doc_comment_padding +
                    // Each subsequent leading whitespace and `///`
                    code_dox.lines().skip(1).take(line_offset - 1).fold(0, |sum, line| {
                        sum + doc_comment_padding + line.len() - line.trim_start().len()
                    })
            };
