rs
 assert!(
            !self.builder.src.join(alias).exists(),
            "use `builder.path()` for real paths: {}",
            alias
        );
