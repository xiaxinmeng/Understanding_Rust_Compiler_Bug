rust
            Subcommand::Setup { ref path } => (Kind::Setup, if let Some(p) = path { std::slice::from_ref(p) } else { &[] }),
