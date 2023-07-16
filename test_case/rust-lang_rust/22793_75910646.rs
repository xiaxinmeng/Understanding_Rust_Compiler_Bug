
diff --git a/src/ui/curses.rs b/src/ui/curses.rs
index 8f879a1..efe97a1 100644
--- a/src/ui/curses.rs
+++ b/src/ui/curses.rs
@@ -164,14 +164,14 @@ impl CursesUI {
             panic!("Your TERM environment variable must end with -256color, sorry, stranger from the past. It is curable. Google it, fix it, try again.");
         }

-        if env::var_os("ESCDELAY").is_none() {
-         //   env::set_var("ESCDELAY", "25");
-        }

         unsafe {
             let _ = locale::setlocale(locale::LC_ALL, b"en_US.UTF-8".as_ptr() as *const i8);
         }

+        if env::var_os("ESCDELAY").is_none() {
+         //   env::set_var("ESCDELAY", "25");
+        }

         nc::initscr();
         nc::start_color();
