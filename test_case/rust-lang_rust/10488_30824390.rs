 rust
fn some_func(pair: (~Foo, ~Bar)) {
    {
        let (ref x, _arg) = pair; // It sounds like this moves ~Bar into _arg ...
        ...
                                  // ... which means we drop ~Bar here ...
    }
    {
        let (_, ref y) = pair;    // ... and thus makes this illegal
    }
}
