
[src/bootstrap/flags.rs:577] split(&matches.opt_strs("host")).into_iter().map(|x|
                                                     TargetSelection::from_user(&x)).collect::<Vec<_>>() = [
    TargetSelection {
        triple: "",
        file: None,
    },
]
