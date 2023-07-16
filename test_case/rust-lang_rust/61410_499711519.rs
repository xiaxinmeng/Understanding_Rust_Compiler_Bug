rust
let explicit = seg.args.iter().map(|args| args.args.iter()).flatten().any(|arg| {
    if let GenericArg::Type(_) = arg { true } else { false }
});
