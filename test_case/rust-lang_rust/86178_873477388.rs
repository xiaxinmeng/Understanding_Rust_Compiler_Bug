
// pretend this has once_cell or something
static HTML_IDS: &[&str] = parse_json(include_str!("src/librustdoc/html_ids.json"));
write_str!(buffer, HTML_IDS[html_id::TRAIT_IMPLEMENTATIONS_LIST]);