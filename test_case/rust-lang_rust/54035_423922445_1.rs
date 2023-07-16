rust
File::open(Path::new(&root)
            .join("Preferences.toml"))?.read_to_string(&mut toml_str);
