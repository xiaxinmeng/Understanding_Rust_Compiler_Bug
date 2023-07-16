plain
    |
239 | impl str {
    |      ^^^
    |
    = help: consider moving this inherent impl into `core` if possible
help: alternatively add `#[rustc_allow_incoherent_impl]` to the relevant impl items
    |
    |
424 | /     fn to_lowercase_cold(&self, mut s: String) -> String {
425 | |         for (i, c) in self[..].char_indices() {
426 | |             if c == 'Σ' {
427 | |                 // Σ maps to σ, except at the end of a word where it maps to ς.
465 | |         }
466 | |     }
    | |_____^


error[E0599]: no method named `to_lowercase_cold` found for reference `&str` in the current scope
    |
    |
420 |         rest.to_lowercase_cold(to)
    |              ^^^^^^^^^^^^^^^^^ method not found in `&str`

error[E0599]: no method named `to_uppercase_cold` found for reference `&str` in the current scope
    |
    |
542 |         rest.to_uppercase_cold(to)
    |              ^^^^^^^^^^^^^^^^^ method not found in `&str`
error[E0599]: no method named `into_string` found for struct `Box<str>` in the current scope
    --> library/alloc/src/string.rs:2644:11
     |
2644 |           s.into_string()
2644 |           s.into_string()
     |             ^^^^^^^^^^^ method not found in `Box<str>`
     |
    ::: library/alloc/src/boxed.rs:180:1
     |
180  | / pub struct Box<
181  | |     T: ?Sized,
182  | |     #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global,
183  | | >(Unique<T>, A);
     | |________________- method `into_string` not found for this
Some errors have detailed explanations: E0390, E0599.
For more information about an error, try `rustc --explain E0390`.
error: could not compile `alloc` due to 4 previous errors
warning: build failed, waiting for other jobs to finish...
