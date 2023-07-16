rust
macro_rules! foo {
    (
        bar: [
            $(
                {
                    baz: [$($crab:ident),*],
                    cor: [$($turtle:ident),*]$(,)*
//                                           ^^^^^ (1)
                    $(gar: [$(urchin:ident),*])*   $(,)*
//                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^   ^^^^^ (3)
//                                 (2)
                }
            ),*
        ]
    ) => {};
}

fn main() {
    foo!(
        bar: [
            {
                baz: [pink, fluffy],
                cor: [],
//                     ^ (4)
            },
            {
                baz: [nyan, cat],
                cor: [cute, funny, friendo],
//                                         ^ (5)
            }
        ]
    );
}
