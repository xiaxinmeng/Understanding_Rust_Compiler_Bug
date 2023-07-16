rust
#[rustc_layout(debug)]
struct Edges<'a, E>(Cow<'a, [E]>) where [E]: ToOwned;
