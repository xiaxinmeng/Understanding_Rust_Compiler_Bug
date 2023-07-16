rust
struct Example {}

fn example<'a>(input: &'a Example) -> impl Iterator<Item = &'a Example> {
    ::std::iter::once(input)
}
