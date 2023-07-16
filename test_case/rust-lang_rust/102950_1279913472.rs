
error: unclosed HTML tag `T`
  --> src/tools/miri/src/concurrency/weak_memory.rs:20:16
   |
20 | //! std::atomic<T> API). It is therefore possible for this implementation to generate behaviours never observable when the
   |                ^^^
   |
   = note: `-D rustdoc::invalid-html-tags` implied by `-D warnings`
help: try marking as source code
   |
20 | //! `std::atomic<T>` API). It is therefore possible for this implementation to generate behaviours never observable when the
   |     +              +

error: could not document `miri`
