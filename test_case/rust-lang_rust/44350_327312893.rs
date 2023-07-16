rust
fn check_attributes(attrs1: &HashMap<String, String>, attrs2: &HashMap<String, String>) -> bool {
    /// For strings that match "something-N", returns "something", else returns the whole string
    fn extract_attr(value: &str) -> &str {
        let mut iter = value.rsplitn(2, '-');
        if let (Some(n), Some(tag)) = (iter.next(), iter.next()) {
            if n.parse::<usize>().is_ok() {
                tag
            } else {
                value
            }
        } else {
            value
        }
    }

    if let (Some(id1), Some(id2)) = (attrs1.get("id"), attrs2.get("id")) {
         let tag1 = extract_attr(id1);
         let tag2 = extract_attr(id2);

        tag1 == tag2
    } else {
        !attrs1.contains_key("id") && !attrs2.contains_key("id")
    }
}
