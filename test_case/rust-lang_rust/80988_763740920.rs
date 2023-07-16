diff
  #[macro_export]
  macro_rules! const_assert {
      ($condition:expr) => {
-         #[deny(const_err)]
-         const _: usize = 0 - !$condition as usize;
+         const _: () = {
+             const CONDITION: bool = $condition;
+             #[forbid(const_err)]
+             const _: usize = 0 - !CONDITION as usize;
+         };
      };
  }