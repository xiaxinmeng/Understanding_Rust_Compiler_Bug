
This is because each test in the `tests` directory is an entirely separate crate, and so we need to import our library.
Meaning that every `#[cfg (test)]` will not be compiled in the main crate.
