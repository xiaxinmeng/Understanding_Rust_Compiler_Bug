rust
/// match Try::into_result(<expr>) {
///     Ok(val) => #[allow(unreachable_code)] val,
///     Err(err) => #[allow(unreachable_code)]
///                 // If there is an enclosing `try {...}`:
///                 break 'catch_target Try::from_error(From::from(err)),
///                 // Otherwise:
///                 return Try::from_error(From::from(err)),
/// }
/// 