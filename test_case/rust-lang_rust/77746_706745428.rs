
 If you're running `tidy`, try again with `--bless`. Or, if you just want to format code, run `./x.py fmt` instead.
     let config = Config::parse(&args);
 
     // check_version warnings are not printed during setup
-    let changelog_suggestion = if matches!(config.cmd, Subcommand::Setup {..}) {
-        None
-    } else {
-        check_version(&config)
-    };
+    let changelog_suggestion =
+        if matches!(config.cmd, Subcommand::Setup {..}) { None } else { check_version(&config) };
