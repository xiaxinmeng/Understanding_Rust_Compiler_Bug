diff
diff --git a/src/main.rs b/src/main.rs
index 40425ba..0ceabdc 100644
--- a/src/main.rs
+++ b/src/main.rs
@@ -303,6 +303,13 @@ fn publish(pkg: &Package, commit: &str, vers: &semver::Version) {
                         }
                     }
                     new_table.insert("version".to_string(), toml::Value::String(vers.to_string()));
+                    if !has_package {
+                        new_table.insert(
+                            "package".to_string(),
+                            toml::Value::String(format!("{}-{}", PREFIX, name)),
+                        );
+                        has_package = true;
+                    }
                     let key_name = if has_package {
                         name.clone()
                     } else {

