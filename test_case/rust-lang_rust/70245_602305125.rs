rust
// `true` if fails compilation
struct DiagnosticBuilder<const IS_ERROR: bool> {...}

impl DiagnosticBuilder<true> {
  pub fn emit() {
    // like today
  }
}

impl DiagnosticBuilder<false> {
  pub fn emit() -> Proof { ... } // returns proof
}

impl DiagnosticBuild<IS_ERROR> {
  // all the others remain generic
}
