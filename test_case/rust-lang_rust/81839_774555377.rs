shell
git clone https://github.com/teloxide/teloxide
cd teloxide/
git checkout fe6429c2790502ae09b04d2659f33e4495f6d314
cargo build --manifest-path examples/sqlite_remember_bot/Cargo.toml
git apply - <<EOF
diff --git a/examples/sqlite_remember_bot/src/transitions.rs b/examples/sqlite_remember_bot/src/transitions.rs
index dcc78db..c8f1777 100644
--- a/examples/sqlite_remember_bot/src/transitions.rs
+++ b/examples/sqlite_remember_bot/src/transitions.rs
@@ -21,6 +21,15 @@ async fn have_number(
     ans: String,
 ) -> TransitionOut<Dialogue> {
     let num = state.number;
+    match ans {
+        "abc" => {
+            match num {
+                1 => { cx.answer_str("hi"); }
+                _ => cx.answer_str("hi")
+            }
+        }
+        _ => ()
+    };
 
     if ans.starts_with("/get") {
         cx.answer_str(format!("Here is your number: {}", num)).await?;
EOF
cargo build --manifest-path examples/sqlite_remember_bot/Cargo.toml
