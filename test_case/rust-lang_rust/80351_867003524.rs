
error: internal compiler error: compiler/rustc_trait_selection/src/traits/codegen.rs:78:17:
Encountered error `OutputTypeParameterMismatch(
    Binder(
        <[closure@ice.rs:10:56: 10:62] as std::ops::FnMut<(<u64 as Viewable<'_>>::View,)>>,
        [Region(BrAnon(0))]
    ),
    Binder(
        <[closure@ice.rs:10:56: 10:62] as std::ops::FnMut<(u64,)>>,
        []
    ),
    Sorts(ExpectedFound {
        expected: u64,
        found: <u64 as Viewable<'_>>::View
    })
)` selecting `Binder(<[closure@ice.rs:10:56: 10:62] as std::ops::FnMut<(u64,)>>, [])` during codegen
