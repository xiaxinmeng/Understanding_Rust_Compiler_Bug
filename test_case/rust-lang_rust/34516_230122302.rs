 rust
enum InnerAttributeParsePolicy {
  Permitted,
  // the string specifies the reason in the form of
  // a recommended error message, perhaps
  NotPermitted(Option<&str>)
}
