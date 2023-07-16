patch
diff --git a/gold/target-reloc.h b/gold/target-reloc.h
index 97e45da619..20a6fc0542 100644
--- a/gold/target-reloc.h
+++ b/gold/target-reloc.h
@@ -136,6 +136,8 @@ class Default_comdat_behavior
     if (Layout::is_debug_info_section(name))
       return CB_PRETEND;
     if (strcmp(name, ".eh_frame") == 0
+        || strncmp(name, ".stack_sizes", 12+1) == 0
+        //|| strncmp(name, ".gnu.build.attributes", 21) == 0  // FIXME: We should really be checking the section type for ST_NOTE... // NOTE: this line(and inspiration for the above .stack_sizes line) from: https://bugzilla.redhat.com/show_bug.cgi?id=1600431#c6 ) // decided to comment out this line since I cannot repro that bug that it supposedly fixes with GNU gold (GNU Binutils 2.31.1) 1.16 or with GNU gold (GNU Binutils 2.32) 1.16
 	|| strcmp(name, ".gcc_except_table") == 0)
       return CB_IGNORE;
     return CB_ERROR;
