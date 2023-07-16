
--- coregrind/m_stacktrace.c    (revision 12485)
+++ coregrind/m_stacktrace.c    (working copy)
@@ -112,10 +112,10 @@
    if (fp_min + 512 >= fp_max) {
       /* If the stack limits look bogus, don't poke around ... but
          don't bomb out either. */
-      if (sps) sps[0] = uregs.xsp;
-      if (fps) fps[0] = uregs.xbp;
-      ips[0] = uregs.xip;
-      return 1;
+      //if (sps) sps[0] = uregs.xsp;
+      //if (fps) fps[0] = uregs.xbp;
+      //ips[0] = uregs.xip;
+      //return 1;
    } 
 #  endif

@@ -272,7 +272,7 @@
       if (fps) fps[0] = uregs.xbp;
       ips[0] = uregs.xip;
       return 1;
-   } 
+   }
 #  endif

