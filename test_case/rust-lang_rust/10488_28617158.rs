
fn some_func(pair: (~Foo, ~Bar)) {
    {
        let (ref x, _) = pair; // should this *really* free the second half of `pair`??
        ...
    }
    {
        let (_, ref y) = pair; // ...because then you couldn't do this latter...
    }
}
