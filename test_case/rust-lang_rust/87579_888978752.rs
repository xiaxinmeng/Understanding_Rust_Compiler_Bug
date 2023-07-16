plain
    Checking toml v0.5.7
    Checking clippy_utils v0.1.56 (/checkout/src/tools/clippy/clippy_utils)
    Checking cargo_metadata v0.12.0
    Checking clippy_lints v0.1.56 (/checkout/src/tools/clippy/clippy_lints)
error: Prefer FxHashMap over HashMap, it has better performance
    |
    |
203 |     HashMap(Span, Ty<'tcx>, Cow<'static, str>, Cow<'static, str>),
    |     ^^^^^^^ help: use: `FxHashMap`
    |
    = note: `-D rustc::default-hash-types` implied by `-D warnings`
    = note: a `use rustc_data_structures::fx::FxHashMap` may be necessary

error: Prefer FxHashSet over HashSet, it has better performance
    |
    |
204 |     HashSet(Span, Ty<'tcx>, Cow<'static, str>),
    |     ^^^^^^^ help: use: `FxHashSet`
    |
    = note: a `use rustc_data_structures::fx::FxHashSet` may be necessary

error: Prefer FxHashMap over HashMap, it has better performance
    |
    |
229 |                 Some(ImplicitHasherType::HashMap(
    |                                          ^^^^^^^ help: use: `FxHashMap`
    |
    = note: a `use rustc_data_structures::fx::FxHashMap` may be necessary

error: Prefer FxHashSet over HashSet, it has better performance
    |
    |
236 |                 Some(ImplicitHasherType::HashSet(
    |                                          ^^^^^^^ help: use: `FxHashSet`
    |
    = note: a `use rustc_data_structures::fx::FxHashSet` may be necessary

error: Prefer FxHashMap over HashMap, it has better performance
    |
    |
251 |             ImplicitHasherType::HashMap(..) => "HashMap",
    |                                 ^^^^^^^ help: use: `FxHashMap`
    |
    = note: a `use rustc_data_structures::fx::FxHashMap` may be necessary

error: Prefer FxHashSet over HashSet, it has better performance
    |
    |
252 |             ImplicitHasherType::HashSet(..) => "HashSet",
    |                                 ^^^^^^^ help: use: `FxHashSet`
    |
    = note: a `use rustc_data_structures::fx::FxHashSet` may be necessary

error: Prefer FxHashMap over HashMap, it has better performance
    |
    |
258 |             ImplicitHasherType::HashMap(.., ref k, ref v) => format!("{}, {}", k, v),
    |                                 ^^^^^^^ help: use: `FxHashMap`
    |
    = note: a `use rustc_data_structures::fx::FxHashMap` may be necessary

error: Prefer FxHashSet over HashSet, it has better performance
    |
    |
259 |             ImplicitHasherType::HashSet(.., ref t) => format!("{}", t),
    |                                 ^^^^^^^ help: use: `FxHashSet`
    |
    = note: a `use rustc_data_structures::fx::FxHashSet` may be necessary

error: Prefer FxHashMap over HashMap, it has better performance
    |
    |
265 |             ImplicitHasherType::HashMap(_, ty, ..) | ImplicitHasherType::HashSet(_, ty, ..) => ty,
    |                                 ^^^^^^^ help: use: `FxHashMap`
    |
    = note: a `use rustc_data_structures::fx::FxHashMap` may be necessary

error: Prefer FxHashSet over HashSet, it has better performance
    |
    |
265 |             ImplicitHasherType::HashMap(_, ty, ..) | ImplicitHasherType::HashSet(_, ty, ..) => ty,
    |                                                                          ^^^^^^^ help: use: `FxHashSet`
    |
    = note: a `use rustc_data_structures::fx::FxHashSet` may be necessary

error: Prefer FxHashMap over HashMap, it has better performance
    |
    |
271 |             ImplicitHasherType::HashMap(span, ..) | ImplicitHasherType::HashSet(span, ..) => span,
    |                                 ^^^^^^^ help: use: `FxHashMap`
    |
    = note: a `use rustc_data_structures::fx::FxHashMap` may be necessary

error: Prefer FxHashSet over HashSet, it has better performance
    |
    |
271 |             ImplicitHasherType::HashMap(span, ..) | ImplicitHasherType::HashSet(span, ..) => span,
    |                                                                         ^^^^^^^ help: use: `FxHashSet`
    |
    = note: a `use rustc_data_structures::fx::FxHashSet` may be necessary
error: aborting due to 12 previous errors

error: could not compile `clippy_lints`

