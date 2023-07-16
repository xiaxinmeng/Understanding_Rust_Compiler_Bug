diff
diff --git a/install-template.sh b/install-template.sh
index 02070c35dcd4..fc8aa7985256 100644
--- a/install-template.sh
+++ b/install-template.sh
@@ -866,7 +866,7 @@ fi
 valopt without "" "comma-separated list of components to not install"
 valopt components "" "comma-separated list of components to install"
 flag list-components "list available components"
-valopt sysconfdir "/etc" "install system configuration files"
+valopt sysconfdir "$CFG_DESTDIR_PREFIX/etc" "install system configuration files"
 valopt bindir "$CFG_DESTDIR_PREFIX/bin" "install binaries"
 valopt libdir "$CFG_DESTDIR_PREFIX/lib" "install libraries"
 valopt mandir "$CFG_DESTDIR_PREFIX/share/man" "install man pages in PATH"
