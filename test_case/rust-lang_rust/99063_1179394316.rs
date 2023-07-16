plain
    Checking clippy_utils v0.1.64 (/checkout/src/tools/clippy/clippy_utils)
    Checking getopts v0.2.21
   Compiling clippy v0.1.64 (/checkout/src/tools/clippy)
    Checking clap_lex v0.2.2
error[E0658]: the `#[expect]` attribute is an experimental feature
    |
    |
245 | #[expect(clippy::too_many_lines)] // Just a big match statement
    |
    = note: see issue #54503 <https://github.com/rust-lang/rust/issues/54503> for more information
    = help: add `#![feature(lint_reasons)]` to the crate attributes to enable


error[E0658]: the `#[expect]` attribute is an experimental feature
    |
    |
353 |     #[expect(clippy::cast_possible_wrap)]
    |
    = note: see issue #54503 <https://github.com/rust-lang/rust/issues/54503> for more information
    = help: add `#![feature(lint_reasons)]` to the crate attributes to enable


error[E0658]: the `#[expect]` attribute is an experimental feature
  --> src/tools/clippy/clippy_utils/src/eager_or_lazy.rs:99:1
   |
99 | #[expect(clippy::too_many_lines)]
   |
   = note: see issue #54503 <https://github.com/rust-lang/rust/issues/54503> for more information
   = help: add `#![feature(lint_reasons)]` to the crate attributes to enable


error[E0658]: the `#[expect]` attribute is an experimental feature
   --> src/tools/clippy/clippy_utils/src/hir_utils.rs:236:5
    |
236 |     #[expect(clippy::similar_names)]
    |
    = note: see issue #54503 <https://github.com/rust-lang/rust/issues/54503> for more information
    = help: add `#![feature(lint_reasons)]` to the crate attributes to enable


error[E0658]: the `#[expect]` attribute is an experimental feature
   --> src/tools/clippy/clippy_utils/src/hir_utils.rs:403:5
    |
403 |     #[expect(clippy::similar_names)]
    |
    = note: see issue #54503 <https://github.com/rust-lang/rust/issues/54503> for more information
    = help: add `#![feature(lint_reasons)]` to the crate attributes to enable


error[E0658]: the `#[expect]` attribute is an experimental feature
   --> src/tools/clippy/clippy_utils/src/hir_utils.rs:603:5
    |
603 |     #[expect(clippy::too_many_lines)]
    |
    = note: see issue #54503 <https://github.com/rust-lang/rust/issues/54503> for more information
    = help: add `#![feature(lint_reasons)]` to the crate attributes to enable


error[E0658]: the `#[expect]` attribute is an experimental feature
   |
   |
52 | #[expect(clippy::invalid_paths)] // internal lints do not know about all external crates
   |
   = note: see issue #54503 <https://github.com/rust-lang/rust/issues/54503> for more information
   = help: add `#![feature(lint_reasons)]` to the crate attributes to enable


error[E0658]: the `#[expect]` attribute is an experimental feature
   |
   |
54 | #[expect(clippy::invalid_paths)] // internal lints do not know about all external crates
   |
   = note: see issue #54503 <https://github.com/rust-lang/rust/issues/54503> for more information
   = help: add `#![feature(lint_reasons)]` to the crate attributes to enable


error[E0658]: the `#[expect]` attribute is an experimental feature
   |
   |
74 | #[expect(clippy::invalid_paths)] // internal lints do not know about all external crates
   |
   = note: see issue #54503 <https://github.com/rust-lang/rust/issues/54503> for more information
   = help: add `#![feature(lint_reasons)]` to the crate attributes to enable


error[E0658]: the `#[expect]` attribute is an experimental feature
    |
    |
127 | #[expect(clippy::invalid_paths)] // internal lints do not know about all external crates
    |
    = note: see issue #54503 <https://github.com/rust-lang/rust/issues/54503> for more information
    = help: add `#![feature(lint_reasons)]` to the crate attributes to enable


error[E0658]: the `#[expect]` attribute is an experimental feature
    |
    |
129 | #[expect(clippy::invalid_paths)] // internal lints do not know about all external crates
    |
    = note: see issue #54503 <https://github.com/rust-lang/rust/issues/54503> for more information
    = help: add `#![feature(lint_reasons)]` to the crate attributes to enable


error[E0658]: the `#[expect]` attribute is an experimental feature
    |
    |
131 | #[expect(clippy::invalid_paths)] // internal lints do not know about all external crates
    |
    = note: see issue #54503 <https://github.com/rust-lang/rust/issues/54503> for more information
    = help: add `#![feature(lint_reasons)]` to the crate attributes to enable


error[E0658]: the `#[expect]` attribute is an experimental feature
    |
    |
133 | #[expect(clippy::invalid_paths)] // internal lints do not know about all external crates
    |
    = note: see issue #54503 <https://github.com/rust-lang/rust/issues/54503> for more information
    = help: add `#![feature(lint_reasons)]` to the crate attributes to enable


error[E0658]: the `#[expect]` attribute is an experimental feature
    |
    |
135 | #[expect(clippy::invalid_paths)] // internal lints do not know about all external crates
    |
    = note: see issue #54503 <https://github.com/rust-lang/rust/issues/54503> for more information
    = help: add `#![feature(lint_reasons)]` to the crate attributes to enable


error[E0658]: the `#[expect]` attribute is an experimental feature
    |
    |
137 | #[expect(clippy::invalid_paths)] // internal lints do not know about all external crates
    |
    = note: see issue #54503 <https://github.com/rust-lang/rust/issues/54503> for more information
    = help: add `#![feature(lint_reasons)]` to the crate attributes to enable


error[E0658]: the `#[expect]` attribute is an experimental feature
    |
    |
182 | #[expect(clippy::invalid_paths)] // internal lints do not know about all external crates
    |
    = note: see issue #54503 <https://github.com/rust-lang/rust/issues/54503> for more information
    = help: add `#![feature(lint_reasons)]` to the crate attributes to enable


error[E0658]: the `#[expect]` attribute is an experimental feature
    |
    |
184 | #[expect(clippy::invalid_paths)] // internal lints do not know about all external crates
    |
    = note: see issue #54503 <https://github.com/rust-lang/rust/issues/54503> for more information
    = help: add `#![feature(lint_reasons)]` to the crate attributes to enable


error[E0658]: the `#[expect]` attribute is an experimental feature
    |
    |
140 | #[expect(clippy::needless_pass_by_value)]
    |
    = note: see issue #54503 <https://github.com/rust-lang/rust/issues/54503> for more information
    = help: add `#![feature(lint_reasons)]` to the crate attributes to enable


error[E0658]: the `#[expect]` attribute is an experimental feature
  --> src/tools/clippy/clippy_utils/src/sugg.rs:52:1
   |
52 | #[expect(clippy::wrong_self_convention)] // ok, because of the function `as_ty` method
   |
   = note: see issue #54503 <https://github.com/rust-lang/rust/issues/54503> for more information
   = help: add `#![feature(lint_reasons)]` to the crate attributes to enable


error[E0658]: the `#[expect]` attribute is an experimental feature
    |
    |
698 | #[expect(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    |
    = note: see issue #54503 <https://github.com/rust-lang/rust/issues/54503> for more information
    = help: add `#![feature(lint_reasons)]` to the crate attributes to enable


error[E0658]: the `#[expect]` attribute is an experimental feature
     |
     |
1563 | #[expect(clippy::cast_possible_wrap)]
     |
     = note: see issue #54503 <https://github.com/rust-lang/rust/issues/54503> for more information
     = help: add `#![feature(lint_reasons)]` to the crate attributes to enable


error[E0658]: the `#[expect]` attribute is an experimental feature
     |
     |
1570 | #[expect(clippy::cast_sign_loss)]
     |
     = note: see issue #54503 <https://github.com/rust-lang/rust/issues/54503> for more information
     = help: add `#![feature(lint_reasons)]` to the crate attributes to enable

