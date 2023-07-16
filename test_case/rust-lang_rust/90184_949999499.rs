diff
diff --git a/src/generator.rs b/src/generator.rs
index bfe962d..79f1329 100644
--- a/src/generator.rs
+++ b/src/generator.rs
@@ -610,14 +610,14 @@ mod tests {
     #[tokio::test]
     async fn growth_rate() {
         // If the test hangs, it's probably because the buffer in generate_domains_test is too small
-        for number_of_tokens in vec![5u8, 10u8] {
+        for number_of_tokens in &[5u8, 10u8] {
             let start_char = 0x41u8; // 'A'

             let tokens: HashSet<String> = HashSet::from_iter(
-                (0..number_of_tokens).map(|i| char::from(start_char + i).to_string()),
+                (0..*number_of_tokens).map(|i| char::from(start_char + i).to_string()),
             );

-            let number_of_tokens = usize::from(number_of_tokens);
+            let number_of_tokens = usize::from(*number_of_tokens);
             assert_eq!(number_of_tokens, tokens.len()); // Sanity check

             let domains = generate_domains_test(&tokens, "example.com").await;
