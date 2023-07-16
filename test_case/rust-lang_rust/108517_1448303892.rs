diff
diff --git a/src/bootstrap/clean.rs b/src/bootstrap/clean.rs
index 468efc1114c4..7ebd0a8f2706 100644
--- a/src/bootstrap/clean.rs
+++ b/src/bootstrap/clean.rs
@@ -62,6 +62,7 @@ fn run(self, builder: &Builder<'_>) -> Self::Output {
                 let target = compiler.host;
                 let mut cargo = builder.bare_cargo(compiler, $mode, target, "clean");
                 for krate in &*self.crates {
+                    cargo.arg("-p");
                     cargo.arg(krate);
                 }
