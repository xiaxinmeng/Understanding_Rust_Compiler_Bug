rust
if ident.name.with(|n| n.chars().next().map_or(false, |c| c.is_ascii_uppercase())) {
    (format!("use of undeclared type `{}`", ident), None)
} else {
    (format!("use of undeclared crate or module `{}`", ident), None)
}
