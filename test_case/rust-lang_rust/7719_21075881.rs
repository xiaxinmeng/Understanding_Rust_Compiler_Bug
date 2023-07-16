
@@ -163,7 +164,8 @@ impl Program {
             None => {}
         }

-        do self.newvars.consume |name, var| {
+        let newvars = util::replace(&mut self.newvars, HashMap::new());
+        for newvars.consume_iter().advance |(name, var)| {
             self.local_vars.insert(name, var);
         }
