diff
@@ -288,7 +288,7 @@ macho_get_commands (struct backtrace_state *state, int descriptor,
 
       archs_total_size = arch_count * sizeof (struct fat_arch);
 
-      if (!backtrace_get_view (state, descriptor, sizeof (fat_header),
+      if (!backtrace_get_view (state, descriptor, sizeof (struct fat_header),
                                archs_total_size, error_callback,
                                data, &fat_archs_view))
