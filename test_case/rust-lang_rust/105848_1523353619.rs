plain
   |
10 | #![feature(is_some_and)]
   |            ^^^^^^^^^^^
   |
   = note: `-D stable-features` implied by `-D warnings`
error[E0658]: use of unstable library feature 'iter_intersperse': recently added
    --> src/librustdoc/clean/types.rs:2098:14
     |
     |
2098 |             .intersperse("::")
     |
     = note: see issue #79524 <https://github.com/rust-lang/rust/issues/79524> for more information
     = help: add `#![feature(iter_intersperse)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'iter_intersperse': recently added
   --> src/librustdoc/clean/utils.rs:304:71
    |
304 |         .chain(num_chars[num_start_index..].rchunks(chunk_size).rev().intersperse(&['_']).flatten())
    |
    = note: see issue #79524 <https://github.com/rust-lang/rust/issues/79524> for more information
    = help: add `#![feature(iter_intersperse)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'iter_intersperse': recently added
   --> src/librustdoc/core.rs:475:22
    |
475 |                     .intersperse("::")
    |
    = note: see issue #79524 <https://github.com/rust-lang/rust/issues/79524> for more information
    = help: add `#![feature(iter_intersperse)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'iter_intersperse': recently added
   --> src/librustdoc/doctest.rs:427:10
    |
427 |         .intersperse_with(|| "\n")
    |
    = note: see issue #79524 <https://github.com/rust-lang/rust/issues/79524> for more information
    = help: add `#![feature(iter_intersperse)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'iter_intersperse': recently added
   --> src/librustdoc/html/highlight.rs:946:37
    |
946 |         text_s = text_s.split("::").intersperse("::").fold(String::new(), |mut path, t| {
    |
    = note: see issue #79524 <https://github.com/rust-lang/rust/issues/79524> for more information
    = help: add `#![feature(iter_intersperse)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'iter_intersperse': recently added
   --> src/librustdoc/html/markdown.rs:263:26
    |
263 |         let text = lines.intersperse("\n".into()).collect::<String>();
    |
    = note: see issue #79524 <https://github.com/rust-lang/rust/issues/79524> for more information
    = help: add `#![feature(iter_intersperse)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'iter_intersperse': recently added
   --> src/librustdoc/html/markdown.rs:282:18
    |
282 |                 .intersperse("\n".into())
    |
    = note: see issue #79524 <https://github.com/rust-lang/rust/issues/79524> for more information
    = help: add `#![feature(iter_intersperse)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'iter_intersperse': recently added
   --> src/librustdoc/html/render/sidebar.rs:119:47
    |
119 |         cx.current.iter().map(|s| s.as_str()).intersperse("::").collect()
    |
    = note: see issue #79524 <https://github.com/rust-lang/rust/issues/79524> for more information
    = help: add `#![feature(iter_intersperse)]` to the crate attributes to enable

