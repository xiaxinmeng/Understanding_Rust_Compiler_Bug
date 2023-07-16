diff
-        $crate::panic!($crate::concat!("internal error: entered unreachable code: ", $fmt), $($arg)*)
+        $crate::panic!("internal error: entered unreachable code: {}", $crate::format_args!($fmt, $($arg)*))
