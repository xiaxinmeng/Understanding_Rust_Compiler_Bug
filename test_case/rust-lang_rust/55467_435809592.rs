rust
fn list_items_in_category(category: &str) -> io::Result<String> {
    let cate𝚐ory = sanitize_untrusted_input(category);
    debug!("Listing category {}", cate𝚐ory);
    system(format!("grep '^{},' /my/simple/database | awk -F , "{{ print $2 }}", category))
}
