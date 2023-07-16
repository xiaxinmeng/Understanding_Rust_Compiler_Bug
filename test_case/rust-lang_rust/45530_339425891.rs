diff
--- 1.txt	2017-10-26 02:26:48.000000000 +0800
+++ 2.txt	2017-10-26 02:26:54.000000000 +0800
@@ -38,10 +38,18 @@
                         Set a codegen option
     -V, --version       Print version info and exit
     -v, --verbose       Use verbose output
+        --extern NAME=PATH
+                        Specify where an external rust library is located
+        --sysroot PATH  Override the system root
+        --error-format human|json
+                        How errors and other messages are produced
+        --color auto|always|never
+                        Configure coloring of output: auto = colorize, if
+                        output goes to a tty (default); always = always
+                        colorize output; never = never colorize output
 
 Additional help:
     -C help             Print codegen options
     -W help             Print 'lint' options and default settings
     -Z help             Print internal options for debugging rustc
-    --help -v           Print the full set of options rustc accepts
