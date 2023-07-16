patch
diff --git a/day_2/galaxy_brain/src/lib.rs b/day_2/galaxy_brain/src/lib.rs
index 208c3b0..55e68e4 100644
--- a/day_2/galaxy_brain/src/lib.rs
+++ b/day_2/galaxy_brain/src/lib.rs
@@ -14,7 +14,7 @@ pub fn construct_galaxy_brain(_input: TokenStream) -> TokenStream {
 use packed_simd::u8x32;
 use ugly_array_decl::ugly_array_decl;

-const BYTES: [u8; 6750] = *include_bytes!("../../zesterer-input.txt");
+const BYTES: [u8; 7000] = *include_bytes!("../../zesterer-input.txt");
 const LINES: usize = 250;
 const ZEROS: u8x32 = u8x32::splat(0);
 const ONES: u8x32 = u8x32::new(1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
@@ -40,4 +40,4 @@ fn part_2_ff() -> String {
         }
     }
     unreachable!();
-}
\ No newline at end of file
+}
diff --git a/day_2/src/main.rs b/day_2/src/main.rs
index 9e2f244..957fab6 100644
--- a/day_2/src/main.rs
+++ b/day_2/src/main.rs
@@ -60,7 +60,7 @@ use std::hint::unreachable_unchecked;
 use packed_simd::u8x32;
 use ugly_array_decl::big_ugly_array_decl;

-const BYTES: [u8; 2700000] = *include_bytes!("../fixed-big-input.txt");
+const BYTES: [u8; 2800000] = *include_bytes!("../fixed-big-input.txt");
 const LINES: usize = 100_000;
 const ZEROS: u8x32 = u8x32::splat(0);
 const ONES: u8x32 = u8x32::new(1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
@@ -241,4 +241,4 @@ fn zesterer(b: &mut Bencher) {
 fn og_zesterer(b: &mut Bencher) {
     let i = include_bytes!("../ameo-input.txt");
     b.iter(|| black_box(original_zesterer(i)));
-}*/
\ No newline at end of file
+}*/
