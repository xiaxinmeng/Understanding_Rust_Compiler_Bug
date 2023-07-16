diff
diff --git a/src/libsyntax/tokenstream.rs b/src/libsyntax/tokenstream.rs
index f5d2d6f18e..db6b701b83 100644
--- a/src/libsyntax/tokenstream.rs
+++ b/src/libsyntax/tokenstream.rs
@@ -255,11 +255,19 @@ impl TokenStream {
             0 => TokenStream::empty(),
             1 => streams.pop().unwrap(),
             _ => {
-                let mut vec = vec![];
+                let tree_count = streams.iter()
+                    .map(|ts| match &ts.0 { None => 0, Some(s) => s.len() })
+                    .sum();
+                let mut vec = Vec::with_capacity(tree_count);
                 for stream in streams {
                     match stream.0 {
                         None => {},
-                        Some(stream2) => vec.extend(stream2.iter().cloned()),
+                        Some(stream2) => {
+                            match Lrc::try_unwrap(stream2) {
+                                Ok(trees) => vec.extend(trees.into_iter()),
+                                Err(stream2) => vec.extend(stream2.iter().cloned()),
+                            }
+                        }
                     }
                 }
                 TokenStream::new(vec)
