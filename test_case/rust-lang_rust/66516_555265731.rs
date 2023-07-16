
warning: a method with this name may be added to the standard library in the future
   --> src/global/extensions.rs:132:26
    |
132 |             Some(v) => v.flatten(),
    |                          ^^^^^^^
    |
    = note: #[warn(unstable_name_collisions)] on by default
    = warning: once this method is added to the standard library, the ambiguity may cause an error or change in behavior!
    = note: for more information, see issue #48919 <https://github.com/rust-lang/rust/issues/48919>
    = help: call with fully qualified syntax `global::extensions::OptionFlatten::flatten(...)` to keep using the current method
