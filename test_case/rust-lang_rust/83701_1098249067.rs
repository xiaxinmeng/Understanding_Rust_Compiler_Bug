rust
fn foo<T>(a: impl Bar, b: T) {}

// Awkward, comparing actual to formal generic args is weird
foo::<_, String>(some_bar, some_string);

fn baz<T>(a: T, b: impl Bar, c: T) {}

// Confusing, since T appears before and after impl Bar
baz::<String, _>(some_string, some_bar, some_string);

// Confusing, which comes first?
fn qux<T>(a: impl Bar<T>)
// More confusing
fn qux2<T>(a: impl Bar<T, impl Baz>)

// Confusing, does the return type come before or after argument usage.
// Non-ergonomic, will be common to specify `T`, very uncommon to specify the type for impl Context.
// Back compat hazard to change `impl Context` to `dyn Context` or to a concrete context type.
fn return_t<T>(c: &impl Context) -> T
