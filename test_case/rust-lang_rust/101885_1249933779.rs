rust
// We need to do the conversion to detect errors, even though we
// discard the result.
_ = LitKind::from_token_lit(token_lit)?;
