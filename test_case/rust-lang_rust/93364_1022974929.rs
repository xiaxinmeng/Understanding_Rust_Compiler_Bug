rust
use std::fmt::{self, Display, Write};

fn join(values: impl IntoIterator<Item = impl Display>, separator: impl Display) -> Result<String, fmt::Error> {
    let mut iter = values.into_iter();
    let mut result = String::new();

    if let Some(first) = iter.next() {
        write!(result, "{}", first)?;

        drop(first);

        for value in iter {
            write!(result, "{}{}", separator, value)?;
        }
    }

    Ok(result)
}

fn main() {
    assert_eq!(join(["a", "bc", "defg"], ", ").unwrap(), "a, bc, defg");
}
