rust
/*[...]*/<T, O, F>/*[...]*/
where
    O: // omitted [...]
    F: ViewFn<T, for<'x> Output<'x> = &'x O>,
