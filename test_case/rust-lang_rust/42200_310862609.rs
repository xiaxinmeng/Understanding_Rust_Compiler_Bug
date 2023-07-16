diff
 impl fmt::Debug for Command {
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
-        write!(f, "{:?}", self.program)?;
-        for arg in &self.args {
-            write!(f, " {:?}", arg)?;
-        }
-        Ok(())
+        f.debug_struct("Command")
+            .field("program", &self.program)
+            .field("args", &self.args)
+            .field("env", &self.env)
+            .field("argv", &self.argv)
+            .field("envp", &self.envp)
+            .field("cwd", &self.cwd)
+            .field("uid", &self.uid)
+            .field("gid", &self.gid)
+            .field("saw_nul", &self.saw_nul)
+            .field("stdin", &self.stdin)
+            .field("stdout", &self.stdout)
+            .field("stderr", &self.stderr)
+            .finish()
     }
 }
