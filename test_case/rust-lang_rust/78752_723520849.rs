
$ echo test > f
$ echo test 1 | git diff f -
fatal: option '-' must come before non-option arguments
$ git diff f <(echo test 1)
diff --git a/f b/f
deleted file mode 100644
index 9daeafb9864..00000000000
--- a/f
+++ /dev/null
@@ -1 +0,0 @@
-test
diff --git a/dev/fd/63 b/dev/fd/63
new file mode 120000
index 00000000000..0b022da6c30
--- /dev/null
+++ b/dev/fd/63
@@ -0,0 +1 @@
+pipe:[2557594]
\ No newline at end of file
$ echo test 1 > g
$ git diff f g
$ cat f
test
$ cat g
test 1
