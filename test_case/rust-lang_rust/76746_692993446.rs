diff
         fn tokens_to_string(tokens: &[TokenType]) -> String {
             let mut i = tokens.iter();
             // This might be a sign we need a connect method on `Iterator`.
-            let b = i.next().map_or(String::new(), |t| t.to_string());
-            i.enumerate().fold(b, |mut b, (i, a)| {
+            let mut b = i.next().map_or(String::new(), |t| t.to_string());
+            i.enumerate().for_each(|(i, a)| {
                 if tokens.len() > 2 && i == tokens.len() - 2 {
                     b.push_str(", or ");
                 } else if tokens.len() == 2 && i == tokens.len() - 2 {
                     b.push_str(" or ");
                 } else {
                     b.push_str(", ");
                 }
                 b.push_str(&a.to_string());
-                b
-            })
+            });
+            b
         }

         let mut expected = edible
