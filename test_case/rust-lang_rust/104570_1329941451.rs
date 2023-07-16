diff
--- b	2022-11-28 16:53:46.718160978 -0800
+++ a	2022-11-28 16:53:36.398616799 -0800
@@ -20,6 +20,7 @@
     "ZERO_AR_DATE=1"
   ],
   "link-env-remove": [
+    "MACOSX_DEPLOYMENT_TARGET",
     "IPHONEOS_DEPLOYMENT_TARGET"
   ],
   "linker-is-gnu": false,
@@ -33,6 +34,14 @@
       "x86_64",
       "-m64"
     ],
+    "ld": [
+      "-arch",
+      "x86_64",
+      "-platform_version",
+      "macos",
+      "10.7",
+      "10.7"
+    ],
     "ld64.lld": [
       "-arch",
       "x86_64",
@@ -44,7 +53,12 @@
   },
   "split-debuginfo": "packed",
   "stack-probes": {
-    "kind": "call"
+    "kind": "inline-or-call",
+    "min-llvm-version-for-inline": [
+      16,
+      0,
+      0
+    ]
   },
   "supported-sanitizers": [
     "address",
