diff
-    // Make sure that the linker/gcc really don't pull in anything, including
-    // default objects, libs, etc.
-    base.pre_link_args.push("-nostdlib".to_string());
