diff
+    // To prevent some nasty linking errors, link in libgcc_s here.
+    base.pre_link_args.push("-lgcc_s".to_string());
