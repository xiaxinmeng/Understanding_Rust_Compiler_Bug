
/// Returns the path to the C++ compiler for the target specified, may panic
/// if no C++ compiler was configured for the target.
fn cxx(&self, target: &str) -> &Path {
    self.cxx[target].path()
}
