 rust
#[deny(no_lint_reason)];

#[allow(no_lint_reason)]
mod foo {
  #[warn(unused_unsafe)];
}
