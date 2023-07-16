rust
OutputTypeParameterMismatch(
    Binder(<[closure@src/world.rs:10:5: 10:26] as std::ops::FnMut<(hecs::Entity, <hecs::query::FetchRead<bool> as hecs::Fetch<'_>>::Item)>>),
    Binder(<[closure@src/world.rs:10:5: 10:26] as std::ops::FnMut<(hecs::Entity, &bool)>>),
    Sorts(ExpectedFound { expected: &bool, found: <hecs::query::FetchRead<bool> as hecs::Fetch<'_>>::Item })
)
