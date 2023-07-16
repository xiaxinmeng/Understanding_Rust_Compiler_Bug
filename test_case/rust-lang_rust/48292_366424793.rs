
error[E0277]: the trait bound `[u8]: core::marker::Sized` is not satisfied in `path::Path`
    --> libstd/path.rs:1414:6
     |
1414 | impl FromStr for Path {
     |      ^^^^^^^ `[u8]` does not have a constant size known at compile-time
     |
     = help: within `path::Path`, the trait `core::marker::Sized` is not implemented for `[u8]`
     = note: required because it appears within the type `path::Path`

error[E0277]: the trait bound `[u8]: core::marker::Sized` is not satisfied in `path::Path`
    --> libstd/path.rs:1417:5
     |
1417 | /     fn from_str(s: &str) -> Result<Self, Self::Err> {
1418 | |         Ok(Path::new(s))
1419 | |     }
     | |_____^ `[u8]` does not have a constant size known at compile-time
     |
     = help: within `path::Path`, the trait `core::marker::Sized` is not implemented for `[u8]`
     = note: required because it appears within the type `path::Path`
     = note: required by `core::result::Result`
