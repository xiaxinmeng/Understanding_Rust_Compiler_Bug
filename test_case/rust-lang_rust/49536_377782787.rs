diff
diff --git a/first-edition/src/error-handling.md b/first-edition/src/error-handling.md
index e2f3995c..ad8690ad 100644
--- a/first-edition/src/error-handling.md
+++ b/first-edition/src/error-handling.md
@@ -1340,7 +1340,8 @@ There's one little nit left: the `Box<Error>` type is *opaque*. If we
 return a `Box<Error>` to the caller, the caller can't (easily) inspect
 underlying error type. The situation is certainly better than `String`
 because the caller can call methods like
-[`description`](../../std/error/trait.Error.html#tymethod.description)
+[`to_string`](../../std/string/trait.ToString.html#tymethod.to_string)
+(through the [`Display`](../../std/fmt/trait.Display.html) trait)
 and [`cause`](../../std/error/trait.Error.html#method.cause), but the
 limitation remains: `Box<Error>` is opaque. (N.B. This isn't entirely
 true because Rust does have runtime reflection, which is useful in
