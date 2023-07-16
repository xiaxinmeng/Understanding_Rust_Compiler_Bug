
/// This increases the strong reference count on the `Arc<T>` associated with
/// the provided pointer.
/// 
/// # Safety
///
/// The pointer must have been obtained through `Arc::into_raw`, and the
/// associated `Arc` instance must be valid (i.e. the strong count must be at
/// least 1) for the duration of this method.
unsafe fn increment_strong_count(ptr: *const T);
unsafe fn decrement_strong_count(ptr: *const T);
