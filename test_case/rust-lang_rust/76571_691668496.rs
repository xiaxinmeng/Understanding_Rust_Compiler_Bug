diff
@@ src/tools/linkchecker/main.rs: fn is_exception(file: &Path, link: &str) -> bool
 +        // calculated in `check` function is outside `build/<triple>/doc` dir.
 +        // So the `strip_prefix` method just returns the old absolute broken path.
 +        if file.ends_with("std/primitive.slice.html") {
-+            if link.ends_with("std/primitive.slice.html") {
++            if link.ends_with("primitive.slice.html") {
 +                return true;
 +            }
 +        }
