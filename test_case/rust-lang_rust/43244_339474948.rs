rust
        vec![
            "",
            "something",
            a_variable,
            function_call(),
            "etc",
        ]
            .reject(|i| { i.is_empty() })
            .join("/")
