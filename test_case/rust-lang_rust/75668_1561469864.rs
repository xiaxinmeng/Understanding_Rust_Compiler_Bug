rust
#[warn(warnings)]
mod outer {
  // Here, warnings are warnings.

  #[deny(warnings)]
  mod inner {
    // Here, they are errors.
  }
}
