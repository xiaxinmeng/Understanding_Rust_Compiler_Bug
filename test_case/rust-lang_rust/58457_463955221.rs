rust
/// An allocator handle that can be re-created from just its type
#[unstable(issue = "0", reason = "temporarily restricts public APIs until \
                                  some compiler implementation limitations are resolved")]
pub unsafe trait StatelessAlloc: Alloc {
    fn get_handle() -> Self;
}
