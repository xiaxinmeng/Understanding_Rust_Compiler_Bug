rust
pub fn extract_code_blocks<'md, I: Iterator<Item=Event<'md>>>(
    md_events: I)
    -> Result<Vec<CodeBlock>, Error>
