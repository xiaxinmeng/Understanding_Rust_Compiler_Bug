diff
1 file changed, 4 insertions(+), 3 deletions(-)
src/test/ui/update-references.sh | 7 ++++---

modified   src/test/ui/update-references.sh
@@ -36,10 +36,11 @@ shopt -s nullglob
 while [[ "$1" != "" ]]; do
     for EXT in "stderr" "stdout"; do
         for OUT_NAME in $BUILD_DIR/${1%.rs}.*$EXT; do
+            OUT_DIR=`dirname "$1"`
             OUT_BASE=`basename "$OUT_NAME"`
-            if ! (diff $OUT_NAME $MYDIR/$OUT_BASE >& /dev/null); then
-                echo updating $MYDIR/$OUT_BASE
-                cp $OUT_NAME $MYDIR
+            if ! (diff $OUT_NAME $MYDIR/$OUT_DIR/$OUT_BASE >& /dev/null); then
+                echo updating $MYDIR/$OUT_DIR/$OUT_BASE
+                cp $OUT_NAME $MYDIR/$OUT_DIR
             fi
         done
     done
