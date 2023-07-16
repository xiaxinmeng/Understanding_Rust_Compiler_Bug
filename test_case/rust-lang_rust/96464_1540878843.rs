
--- stderr -------------------------------
  /b/s/w/ir/cache/builder/src/third_party/rust_src/src/tests/codegen/vec-shrink-panik.rs:28:12: error: CHECK: expected string not found in input
   // CHECK: filter
             ^
  /b/s/w/ir/cache/builder/src/third_party/rust_src/src/build/x86_64-unknown-linux-gnu/test/codegen/vec-shrink-panik.new/vec-shrink-panik.ll:95:32: note: scanning from here
  define { ptr, i64 } @issue71861(ptr noalias nocapture noundef dereferenceable(24) %vec) unnamed_addr #1 personality ptr @rust_eh_personality {
                                 ^
  /b/s/w/ir/cache/builder/src/third_party/rust_src/src/build/x86_64-unknown-linux-gnu/test/codegen/vec-shrink-panik.new/vec-shrink-panik.ll:95:47: note: possible intended match here
  define { ptr, i64 } @issue71861(ptr noalias nocapture noundef dereferenceable(24) %vec) unnamed_addr #1 personality ptr @rust_eh_personality {
                                                ^
  /b/s/w/ir/cache/builder/src/third_party/rust_src/src/tests/codegen/vec-shrink-panik.rs:43:12: error: CHECK: expected string not found in input
   // CHECK: filter
             ^
  /b/s/w/ir/cache/builder/src/third_party/rust_src/src/build/x86_64-unknown-linux-gnu/test/codegen/vec-shrink-panik.new/vec-shrink-panik.ll:158:32: note: scanning from here
  define { ptr, i64 } @issue75636(ptr noalias noundef nonnull readonly align 8 %iter.0, i64 noundef %iter.1) unnamed_addr #1 personality ptr @rust_eh_personality {
                                 ^
  /b/s/w/ir/cache/builder/src/third_party/rust_src/src/build/x86_64-unknown-linux-gnu/test/codegen/vec-shrink-panik.new/vec-shrink-panik.ll:158:78: note: possible intended match here
  define { ptr, i64 } @issue75636(ptr noalias noundef nonnull readonly align 8 %iter.0, i64 noundef %iter.1) unnamed_addr #1 personality ptr @rust_eh_personality {
                                                                               ^
