rust
let loc = if let Some(first_line) = annotated_file.lines.first() {
    let col = if let Some(first_annotation) = first_line.annotations.first() {
        format!(":{}", first_annotation.start_col + 1)
    } else {
        String::new()
    };
    format!(
        "{}:{}{}",
        sm.filename_for_diagnostics(&annotated_file.file.name),
        sm.doctest_offset_line(&annotated_file.file.name, first_line.line_index),
        col
    )
} else {
    format!("{}", sm.filename_for_diagnostics(&annotated_file.file.name))
};
