diff
diff --git a/src/bootstrap/tool.rs b/src/bootstrap/tool.rs
index 8ebad95ea17..054c7412b84 100644
--- a/src/bootstrap/tool.rs
+++ b/src/bootstrap/tool.rs
@@ -254,6 +254,10 @@ pub fn prepare_tool_cargo(

     cargo.env("CFG_RELEASE_CHANNEL", &builder.config.channel);
     cargo.env("CFG_VERSION", builder.rust_version());
+    // CFG_RELEASE is needed by rustfmt (and possibly other tools) which
+    // import rustc-ap-rustc_attr which requires this to be set for the
+    // `#[cfg(version(...))]` attribute.
+    cargo.env("CFG_RELEASE", builder.rust_release());
     cargo.env("CFG_RELEASE_NUM", channel::CFG_RELEASE_NUM);

     let info = GitInfo::new(builder.config.ignore_git, &dir);
