rust
if !self.short_message {
    // remember where we are in the output buffer for easy reference
    let buffer_msg_line_offset = buffer.num_lines();

    buffer.prepend(buffer_msg_line_offset, "--> ", Style::LineNumber);
    buffer.append(
        buffer_msg_line_offset,
        &format!(
            "{}:{}:{}",
            sm.filename_for_diagnostics(&loc.file.name),
            sm.doctest_offset_line(&loc.file.name, loc.line),
            loc.col.0 + 1,
        ),
        Style::LineAndColumn,
    );
    for _ in 0..max_line_num_len {
        buffer.prepend(buffer_msg_line_offset, " ", Style::NoStyle);
    }
} else {
    buffer.prepend(
        0,
        &format!(
            "{}:{}:{}: ",
            sm.filename_for_diagnostics(&loc.file.name),
            sm.doctest_offset_line(&loc.file.name, loc.line),
            loc.col.0 + 1,
        ),
        Style::LineAndColumn,
    );
}
