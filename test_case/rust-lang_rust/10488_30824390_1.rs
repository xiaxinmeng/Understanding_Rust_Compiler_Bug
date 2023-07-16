 rust
fn some_func(pair: (~Foo, ~Bar)) {
    {
        let (ref x, ref _arg) = pair; // do not move ~Bar ...
        ...
    }
    {
        let (ref _ign, ref y) = pair; // ... and thus this is fine.
    }
}
