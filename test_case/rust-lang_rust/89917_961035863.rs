diff
diff --git a/coregrind/m_demangle/demangle.c b/coregrind/m_demangle/demangle.c
index 16161da2a..997e6a56b 100644
--- a/coregrind/m_demangle/demangle.c
+++ b/coregrind/m_demangle/demangle.c
@@ -119,7 +119,8 @@ void VG_(demangle) ( Bool do_cxx_demangling, Bool do_z_demangling,
 
    /* Possibly undo (1) */
    if (do_cxx_demangling && VG_(clo_demangle)
-       && orig != NULL && orig[0] == '_' && orig[1] == 'Z') {
+       && orig != NULL && orig[0] == '_' && (orig[1] == 'Z'
+       || orig[1] == 'R')) {
       /* !!! vvv STATIC vvv !!! */
       static HChar* demangled = NULL;
       /* !!! ^^^ STATIC ^^^ !!! */
