 diff
diff --git a/src/liburl/lib.rs b/src/liburl/lib.rs
index b95fc3c..548ef46 100644
--- a/src/liburl/lib.rs
+++ b/src/liburl/lib.rs
@@ -21,6 +21,8 @@
        html_playground_url = "http://play.rust-lang.org/")]
 #![feature(default_type_params)]

+#![deprecated="This is being removed. Use rust-url instead. https://github.com/servo/rust-url"]
+
 use std::collections::HashMap;
 use std::fmt;
 use std::from_str::FromStr;
