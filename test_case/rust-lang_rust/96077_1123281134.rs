
error[E0221]: ambiguous associated type `Item` in bounds of `I`
   --> library/core/src/array/mod.rs:770:77
    |
770 | unsafe fn collect_into_array_unchecked<I, const N: usize>(iter: &mut I) -> [I::Item; N]
    |                                                                             ^^^^^^^ ambiguous associated type `Item`
    |
   ::: library/core/src/iter/traits/iterator.rs:65:5
    |
65  |     type Item;
    |     ----------
    |     |
    |     ambiguous `Item` from `Iterator<>`
    |     ambiguous `Item` from `Iterator<>`
    |
help: use fully qualified syntax to disambiguate
    |
770 | unsafe fn collect_into_array_unchecked<I, const N: usize>(iter: &mut I) -> [<I as Iterator<>>::Item; N]
    |                                                                             ~~~~~~~~~~~~~~~~~~~
help: use fully qualified syntax to disambiguate
    |
770 | unsafe fn collect_into_array_unchecked<I, const N: usize>(iter: &mut I) -> [<I as Iterator<>>::Item; N]
    |                                                                             ~~~~~~~~~~~~~~~~~~~

error[E0221]: ambiguous associated type `Item` in bounds of `A`
   --> library/core/src/iter/adapters/zip.rs:165:52
    |
140 | / macro_rules! zip_impl_general_defaults {
141 | |     () => {
142 | |         default fn new(a: A, b: B) -> Self {
143 | |             Zip {
...   |
165 | |         default fn next_back(&mut self) -> Option<(A::Item, B::Item)>
    | |                                                    ^^^^^^^ ambiguous associated type `Item`
...   |
194 | |     };
195 | | }
    | |_- in this expansion of `zip_impl_general_defaults!`
...
206 |       zip_impl_general_defaults! {}
    |       ----------------------------- in this macro invocation
    |
   ::: library/core/src/iter/traits/iterator.rs:65:5
    |
65  |       type Item;
    |       ----------
    |       |
    |       ambiguous `Item` from `Iterator<>`
    |       ambiguous `Item` from `Iterator<>`
    |
help: use fully qualified syntax to disambiguate
    |
165 |         default fn next_back(&mut self) -> Option<(<A as Iterator<>>::Item, B::Item)>
    |                                                    ~~~~~~~~~~~~~~~~~~~
help: use fully qualified syntax to disambiguate
    |
165 |         default fn next_back(&mut self) -> Option<(<A as Iterator<>>::Item, B::Item)>
    |                                                    ~~~~~~~~~~~~~~~~~~~

error[E0221]: ambiguous associated type `Item` in bounds of `B`
   --> library/core/src/iter/adapters/zip.rs:165:61
    |
140 | / macro_rules! zip_impl_general_defaults {
141 | |     () => {
142 | |         default fn new(a: A, b: B) -> Self {
143 | |             Zip {
...   |
165 | |         default fn next_back(&mut self) -> Option<(A::Item, B::Item)>
    | |                                                             ^^^^^^^ ambiguous associated type `Item`
...   |
194 | |     };
195 | | }
    | |_- in this expansion of `zip_impl_general_defaults!`
...
206 |       zip_impl_general_defaults! {}
    |       ----------------------------- in this macro invocation
    |
   ::: library/core/src/iter/traits/iterator.rs:65:5
    |
65  |       type Item;
    |       ----------
    |       |
    |       ambiguous `Item` from `Iterator<>`
    |       ambiguous `Item` from `Iterator<>`
    |
help: use fully qualified syntax to disambiguate
    |
165 |         default fn next_back(&mut self) -> Option<(A::Item, <B as Iterator<>>::Item)>
    |                                                             ~~~~~~~~~~~~~~~~~~~
help: use fully qualified syntax to disambiguate
    |
165 |         default fn next_back(&mut self) -> Option<(A::Item, <B as Iterator<>>::Item)>
    |                                                             ~~~~~~~~~~~~~~~~~~~

error[E0221]: ambiguous associated type `Item` in bounds of `A`
   --> library/core/src/iter/adapters/zip.rs:165:52
    |
140 | / macro_rules! zip_impl_general_defaults {
141 | |     () => {
142 | |         default fn new(a: A, b: B) -> Self {
143 | |             Zip {
...   |
165 | |         default fn next_back(&mut self) -> Option<(A::Item, B::Item)>
    | |                                                    ^^^^^^^ ambiguous associated type `Item`
...   |
194 | |     };
195 | | }
    | |_- in this expansion of `zip_impl_general_defaults!`
...
239 |       zip_impl_general_defaults! {}
    |       ----------------------------- in this macro invocation
    |
   ::: library/core/src/iter/traits/iterator.rs:65:5
    |
65  |       type Item;
    |       ----------
    |       |
    |       ambiguous `Item` from `Iterator<>`
    |       ambiguous `Item` from `Iterator<>`
    |
help: use fully qualified syntax to disambiguate
    |
165 |         default fn next_back(&mut self) -> Option<(<A as Iterator<>>::Item, B::Item)>
    |                                                    ~~~~~~~~~~~~~~~~~~~
help: use fully qualified syntax to disambiguate
    |
165 |         default fn next_back(&mut self) -> Option<(<A as Iterator<>>::Item, B::Item)>
    |                                                    ~~~~~~~~~~~~~~~~~~~

error[E0221]: ambiguous associated type `Item` in bounds of `B`
   --> library/core/src/iter/adapters/zip.rs:165:61
    |
140 | / macro_rules! zip_impl_general_defaults {
141 | |     () => {
142 | |         default fn new(a: A, b: B) -> Self {
143 | |             Zip {
...   |
165 | |         default fn next_back(&mut self) -> Option<(A::Item, B::Item)>
    | |                                                             ^^^^^^^ ambiguous associated type `Item`
...   |
194 | |     };
195 | | }
    | |_- in this expansion of `zip_impl_general_defaults!`
...
239 |       zip_impl_general_defaults! {}
    |       ----------------------------- in this macro invocation
    |
   ::: library/core/src/iter/traits/iterator.rs:65:5
    |
65  |       type Item;
    |       ----------
    |       |
    |       ambiguous `Item` from `Iterator<>`
    |       ambiguous `Item` from `Iterator<>`
    |
help: use fully qualified syntax to disambiguate
    |
165 |         default fn next_back(&mut self) -> Option<(A::Item, <B as Iterator<>>::Item)>
    |                                                             ~~~~~~~~~~~~~~~~~~~
help: use fully qualified syntax to disambiguate
    |
165 |         default fn next_back(&mut self) -> Option<(A::Item, <B as Iterator<>>::Item)>
    |                                                             ~~~~~~~~~~~~~~~~~~~

error[E0221]: ambiguous associated type `Item` in bounds of `A`
   --> library/core/src/iter/adapters/zip.rs:330:40
    |
330 |     fn next_back(&mut self) -> Option<(A::Item, B::Item)>
    |                                        ^^^^^^^ ambiguous associated type `Item`
    |
   ::: library/core/src/iter/traits/iterator.rs:65:5
    |
65  |     type Item;
    |     ----------
    |     |
    |     ambiguous `Item` from `Iterator<>`
    |     ambiguous `Item` from `Iterator<>`
    |
help: use fully qualified syntax to disambiguate
    |
330 |     fn next_back(&mut self) -> Option<(<A as Iterator<>>::Item, B::Item)>
    |                                        ~~~~~~~~~~~~~~~~~~~
help: use fully qualified syntax to disambiguate
    |
330 |     fn next_back(&mut self) -> Option<(<A as Iterator<>>::Item, B::Item)>
    |                                        ~~~~~~~~~~~~~~~~~~~

error[E0221]: ambiguous associated type `Item` in bounds of `B`
   --> library/core/src/iter/adapters/zip.rs:330:49
    |
330 |     fn next_back(&mut self) -> Option<(A::Item, B::Item)>
    |                                                 ^^^^^^^ ambiguous associated type `Item`
    |
   ::: library/core/src/iter/traits/iterator.rs:65:5
    |
65  |     type Item;
    |     ----------
    |     |
    |     ambiguous `Item` from `Iterator<>`
    |     ambiguous `Item` from `Iterator<>`
    |
help: use fully qualified syntax to disambiguate
    |
330 |     fn next_back(&mut self) -> Option<(A::Item, <B as Iterator<>>::Item)>
    |                                                 ~~~~~~~~~~~~~~~~~~~
help: use fully qualified syntax to disambiguate
    |
330 |     fn next_back(&mut self) -> Option<(A::Item, <B as Iterator<>>::Item)>
    |                                                 ~~~~~~~~~~~~~~~~~~~

error: internal compiler error: compiler/rustc_typeck/src/check/fn_ctxt/_impl.rs:196:37: MethodCallee {
                                    def_id: DefId(0:3401 ~ core[cd04]::ops::function::FnOnce::call_once),
                                    substs: [
                                        impl FnMut(A, B) -> T,
                                        (_, _),
                                        Param,
                                    ],
                                    sig: ([impl FnMut(A, B) -> T, (_, _)]; c_variadic: false)-><impl FnMut(A, B) -> T as ops::function::FnOnce>::Output<>,
                                }
                                Generics {
                                    parent: Some(
                                        DefId(0:3398 ~ core[cd04]::ops::function::FnOnce),
                                    ),
                                    parent_count: 2,
                                    params: [
                                        GenericParamDef {
                                            name: "<constness>",
                                            def_id: DefId(0:3401 ~ core[cd04]::ops::function::FnOnce::call_once),
                                            index: 3,
                                            pure_wrt_drop: false,
                                            kind: Constness,
                                        },
                                    ],
                                    param_def_id_to_index: {
                                        DefId(0:3401 ~ core[cd04]::ops::function::FnOnce::call_once): 3,
                                    },
                                    has_self: true,
                                    has_late_bound_regions: None,
                                }
                                i: 3
   --> library/core/src/ops/try_trait.rs:383:39
    |
383 |         move |a, b| NeverShortCircuit(f(a, b))
    |               
