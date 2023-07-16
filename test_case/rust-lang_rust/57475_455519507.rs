diff
diff --git a/src/test/ui/try-block/try-block-bad-type.stderr b/src/test/ui/try-block/try-block-bad-type.stderr
index c2f9e9b52b..4e964f4b83 100644
--- a/src/test/ui/try-block/try-block-bad-type.stderr
+++ b/src/test/ui/try-block/try-block-bad-type.stderr
@@ -5,9 +5,9 @@ LL |         Err("")?; //~ ERROR the trait bound `i32: std::convert::From<&str>`
    |         ^^^^^^^^ the trait `std::convert::From<&str>` is not implemented for `i32`
    |
    = help: the following implementations were found:
-             <i32 as std::convert::From<core::num::NonZeroI32>>
+             <i32 as std::convert::From<bool>>
              <i32 as std::convert::From<i16>>
-             <i32 as std::convert::From<i8>>
+             <i32 as std::convert::From<u16>>
              <i32 as std::convert::From<u8>>
            and 2 others
    = note: required by `std::convert::From::from`