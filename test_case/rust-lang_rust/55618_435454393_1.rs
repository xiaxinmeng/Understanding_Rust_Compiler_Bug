rust
use some_crate::Thing;

fn foo(x: Bar) -> Baz { ... }

#[cfg(test)]
mod test {
    // use super::*;
    // ^-- you don't have to do this anymore.

    ...
}
