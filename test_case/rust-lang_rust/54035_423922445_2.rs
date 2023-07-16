rust
File::open(Path::new(&root)
            .join("Preferences.toml"))?.read_to_string(toml_str.as_ref());
