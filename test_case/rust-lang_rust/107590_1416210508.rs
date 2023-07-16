patch
     b.iter(|| {
-        format_shortest(black_box(&decoded), &mut buf);
+        format_shortest(black_box(&decoded), &mut buf)
     });
