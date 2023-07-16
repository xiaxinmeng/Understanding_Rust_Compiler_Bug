diff
--- a/src/test/codegen/repr-transparent.rs
+++ b/src/test/codegen/repr-transparent.rs
@@ -145,7 +145,7 @@ pub struct Rgb8 { r: u8, g: u8, b: u8 }
 pub struct Rgb8Wrap(Rgb8);
 
 // NB: closing parenthesis is missing because sometimes the argument has a name and sometimes not
-// CHECK: define i32 @test_Rgb8Wrap(i32
+// CHECK: define i24 @test_Rgb8Wrap(i32
 #[no_mangle]
 #[cfg(all(target_arch="x86_64", target_os="linux"))]
 pub extern fn test_Rgb8Wrap(_: Rgb8Wrap) -> Rgb8Wrap { loop {} }
