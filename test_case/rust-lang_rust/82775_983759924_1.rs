diff
-assert_matches!(rx.next(), Some(Msgs::Specific { a, b: B(b), .. }) => {
+assert_matches!(rx.next(), Some(Msgs::Specific { a, b: B(b), .. }) if ({
     assert_eq!(a, b);
     let container = transform($container);
     if container.is_empty() {
         assert!(/* … */);
     } else {
         assert!(/* … */);
     }
-});
+    true
+}));
