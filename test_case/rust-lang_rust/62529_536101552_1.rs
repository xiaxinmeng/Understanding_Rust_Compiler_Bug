rust
OutputTypeParameterMismatch(
    Binder(<[closure@src/main.rs:21:13] as Fn<(<u8 as Trait<'_>>::Out)>>),
    Binder(<[closure@src/main.rs:21:13] as Fn<(u8)>>),
    Sorts(ExpectedFound {
        expected: u8,
        found: <u8 as Trait<'_>>::Out
    })
)
