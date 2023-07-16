rust
struct Example {}

fn example(input: &Example) -> impl Iterator<Item = &Example> {
    ::std::iter::once(input)
}
