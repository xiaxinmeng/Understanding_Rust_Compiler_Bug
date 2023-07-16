rust
can_be_public = ["enum", "struct", "fn", "extern", "trait"];
if can_be_public.contains(&self.token.as_str()) {
    // ...
}
