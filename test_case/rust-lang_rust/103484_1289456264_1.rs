rust")).collect();
+        !blocks.is_empty() && blocks.iter().all(|line| line.contains(",ignore"))
     }

     /// Checks the doc style of the lint.
