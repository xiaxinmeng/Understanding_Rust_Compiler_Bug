rust
                r#"<link rel="stylesheet" type="text/css" href="{}.css"{}>"#,
                Escape(&format!("{}{}{}", static_root_path, t, page.resource_suffix)),
                if t == "light" { "" } else { " disabled" }
