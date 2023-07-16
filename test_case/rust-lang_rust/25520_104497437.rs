 diff
--- a/compiler/rustc.vim
+++ b/compiler/rustc.vim
@@ -22,6 +22,8 @@ else
 endif

 CompilerSet errorformat=
+                       \note:\ %m\\\,\ defined\ at\ %f:%l:%c:\ %*\\d:%*\\d,
+                       \note:\ %m:\ %f:%l:%c:\ %*\\d:%*\\d,
                        \%f:%l:%c:\ %t%*[^:]:\ %m,
                        \%f:%l:%c:\ %*\\d:%*\\d\ %t%*[^:]:\ %m,
                        \%-G%f:%l\ %s,
