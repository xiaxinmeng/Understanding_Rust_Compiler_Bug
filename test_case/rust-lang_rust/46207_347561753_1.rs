diff
diff --git a/src/test/run-make/cdylib-fewer-symbols/Makefile b/src/test/run-make/cdylib-fewer-symbols/Makefile
index 954ee792460a..929d5571194b 100644
--- a/src/test/run-make/cdylib-fewer-symbols/Makefile
+++ b/src/test/run-make/cdylib-fewer-symbols/Makefile
@@ -9,9 +9,5 @@ all:
 else
 all:
 	$(RUSTC) foo.rs
-	nm -g "$(call DYLIB,foo)"
-	nm -g "$(call DYLIB,foo)" | grep -vq __rdl_
-	nm -g "$(call DYLIB,foo)" | grep -vq __rde_
-	nm -g "$(call DYLIB,foo)" | grep -vq __rg_
-	nm -g "$(call DYLIB,foo)" | grep -vq __rust_
+	nm -g "$(call DYLIB,foo)" | $(CGREP) -v __rdl_ __rde_ __rg_ __rust_
