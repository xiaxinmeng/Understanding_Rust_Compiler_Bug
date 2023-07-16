diff
  macro_rules! j {
-     (@ $v:tt) => {
+     ($v:tt) => {
          let _ = $v;
      };

      (# $v:tt) => {
-         j!(@ $v);
+         j!($v);
      };
  }
