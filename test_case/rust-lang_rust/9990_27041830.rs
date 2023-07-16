
diff --git a/configure b/configure
index 0f965fc..d8b28bb 100755
--- a/configure
+++ b/configure
@@ -616,6 +616,26 @@ do
 done
 CFG_TARGET=$V_TEMP

+# copy host-triples to target-triples so that hosts are a subset of targets
+# XXX: remove deprecated variables here
+V_TEMP=""
+for i in $CFG_HOST_TRIPLES $CFG_TARGET_TRIPLES;
+do
+   echo "$V_TEMP" | grep -qF $i || V_TEMP="$V_TEMP${V_TEMP:+ }$i"
+done
+CFG_TARGET_TRIPLES=$V_TEMP
+
+# XXX: Support for deprecated syntax, should be dropped.
+if [ ! -z "$CFG_BUILD_TRIPLE" ]; then
+    CFG_BUILD=${CFG_BUILD_TRIPLE}
+fi
+if [ ! -z "$CFG_HOST_TRIPLES" ]; then
+    CFG_HOST=${CFG_HOST_TRIPLES}
+fi
+if [ ! -z "$CFG_TARGET_TRIPLES" ]; then
+    CFG_TARGET=${CFG_TARGET_TRIPLES}
+fi
+
 # check target-specific tool-chains
 for i in $CFG_TARGET
 do
@@ -718,7 +738,7 @@ then
     CFG_LIBDIR=bin
 fi

-for h in $CFG_HOST_
+for h in $CFG_HOST
 do
     for t in $CFG_TARGET
     do
@@ -986,20 +1006,6 @@ putvar CFG_ANDROID_CROSS_PATH
 putvar CFG_MINGW32_CROSS_PATH
 putvar CFG_MANDIR

-# Support for deprecated syntax, should be dropped.
-putvar CFG_BUILD_TRIPLE
-putvar CFG_HOST_TRIPLES
-putvar CFG_TARGET_TRIPLES
-if [ ! -z "$CFG_BUILD_TRIPLE" ]; then
-    CFG_BUILD=${CFG_BUILD_TRIPLE}
-fi
-if [ ! -z "$CFG_HOST_TRIPLES" ]; then
-    CFG_HOST=${CFG_HOST_TRIPLES}
-fi
-if [ ! -z "$CFG_TARGET_TRIPLES" ]; then
-    CFG_TARGET=${CFG_TARGET_TRIPLES}
-fi
-
 if [ ! -z "$CFG_ENABLE_PAX_FLAGS" ]
 then
     putvar CFG_ENABLE_PAX_FLAGS
