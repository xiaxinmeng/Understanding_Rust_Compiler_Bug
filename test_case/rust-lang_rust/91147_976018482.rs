rust
/// foo.
{
  #[cfg(debug_assertions)]
  pub const FOO: () = ();
  #[cfg(not(debug_assertions))]
  pub const FOO: () = ();
}
