rust
let mut input_stream_iter = input.into_iter()
let input_span = match (input_stream_iter.next(), input_stream_iter.last()) {
    (Some(first_token), Some(last_token)) => first_token
        .span()
        .join(last_token.span())
        .expect("the two spans are from the same file"),
    (Some(first_token), None) => first_token.span(),
    _ => return Ok(...),
};

let Some(source_text) = input_span.source_text() else {
    return Err(...);
};
