rust
/// # OS-specific behaviors
///
/// An `Instant` is a wrapper around system-specific types. It may behave
/// differently depending upon the underlying operating system. Additionally
/// the inconsistency in internal type attributes and representation results in
/// differences in the size of duration than can be added without the
/// calculation overflowing. For example, the following snippet works fine on
/// Linux but panics on macOS:
///
/// 