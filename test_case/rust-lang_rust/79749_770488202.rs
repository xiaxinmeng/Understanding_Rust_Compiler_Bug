diff
diff --git a/src/librustdoc/html/markdown.rs b/src/librustdoc/html/markdown.rs
index 0e4c5410abe..d08ffde201f 100644
--- a/src/librustdoc/html/markdown.rs
+++ b/src/librustdoc/html/markdown.rs
@@ -1051,12 +1051,24 @@ fn markdown_summary_with_limit(md: &str, length_limit: usize) -> (String, bool)
     let mut s = String::with_capacity(md.len() * 3 / 2);
     let mut text_length = 0;
     let mut stopped_early = false;
+    let mut unclosed_tags = SmallVec::<[&str; 4]>::new();
 
-    fn push(s: &mut String, text_length: &mut usize, text: &str) {
+    fn push_text(s: &mut String, text_length: &mut usize, text: &str) {
         s.push_str(text);
         *text_length += text.len();
     };
 
+    fn push_open_tag(s: &mut String, open_tag: &str, close_tag: &str) {
+        s.push_str(open_tag);
+        unclosed_tags.push(close_tag);
+    }
+
+    fn push_close_tag(s: &mut String, expected_close_tag: &str) {
+        let close_tag = unclosed_tags.pop().expect("no unclosed tags left");
+        assert_eq!(close_tag, expected_close_tag);
+        s.push_str(close_tag);
+    }
+
     'outer: for event in Parser::new_ext(md, Options::ENABLE_STRIKETHROUGH) {
         match &event {
             Event::Text(text) => {
@@ -1066,7 +1078,7 @@ fn push(s: &mut String, text_length: &mut usize, text: &str) {
                         break 'outer;
                     }
 
-                    push(&mut s, &mut text_length, word);
+                    push_text(&mut s, &mut text_length, word);
                 }
             }
             Event::Code(code) => {
@@ -1076,18 +1088,18 @@ fn push(s: &mut String, text_length: &mut usize, text: &str) {
                 }
 
                 s.push_str("<code>");
-                push(&mut s, &mut text_length, code);
+                push_text(&mut s, &mut text_length, code);
                 s.push_str("</code>");
             }
             Event::Start(tag) => match tag {
-                Tag::Emphasis => s.push_str("<em>"),
-                Tag::Strong => s.push_str("<strong>"),
+                Tag::Emphasis => push_open_tag(&mut s, "<em>", "</em>"),
+                Tag::Strong => push_open_tag(&mut s, "<strong>", "</strong>"),
                 Tag::CodeBlock(..) => break,
                 _ => {}
             },
             Event::End(tag) => match tag {
-                Tag::Emphasis => s.push_str("</em>"),
-                Tag::Strong => s.push_str("</strong>"),
+                Tag::Emphasis => push_close_tag(&mut s, "</em>"),
+                Tag::Strong => push_close_tag(&mut s, "</strong>"),
                 Tag::Paragraph => break,
                 _ => {}
             },
@@ -1097,7 +1109,7 @@ fn push(s: &mut String, text_length: &mut usize, text: &str) {
                     break;
                 }
 
-                push(&mut s, &mut text_length, " ");
+                push_text(&mut s, &mut text_length, " ");
             }
             _ => {}
         }
