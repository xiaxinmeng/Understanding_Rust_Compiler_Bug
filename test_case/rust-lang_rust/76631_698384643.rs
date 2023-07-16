diff
commit 7304b64d28415841c6d1b07501194471109f917e
Author: Joshua Nelson <jyn514@gmail.com>
Date:   Thu Sep 24 10:31:54 2020 -0400

    Note that distro maintainers will see the changelog warning
    
    My previous TODO was incorrect

diff --git a/src/bootstrap/bin/main.rs b/src/bootstrap/bin/main.rs
index 8290420ec06..669dd7a33de 100644
--- a/src/bootstrap/bin/main.rs
+++ b/src/bootstrap/bin/main.rs
@@ -15,7 +15,8 @@ fn main() {
 
     let changelog_suggestion = check_version(&config);
 
-    // TODO: `./configure` should show a warning about the changelog, not `x.py setup`
+    // NOTE: Since `./configure` generates a `config.toml`, distro maintainers will see the
+    // changelog warning, not the `x.py setup` message.
     let suggest_setup = !config.config.exists() && !matches!(config.cmd, Subcommand::Setup { .. });
     if suggest_setup {
         println!("warning: you have not made a `config.toml`");
