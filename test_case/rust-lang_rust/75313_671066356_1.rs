rust
fn captures<'a>(
    re: &Regex,
    text: &'a str,
) -> Option<Collect<impl Iterator<Item = Option<Match<'a>>>>>
