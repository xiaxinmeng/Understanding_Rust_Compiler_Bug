diff
diff --git a/src/test/run-make/relocation-model/Makefile b/src/test/run-make/relocation-model/Makefile
index 485ecbb4b5..f9213f0af0 100644
--- a/src/test/run-make/relocation-model/Makefile
+++ b/src/test/run-make/relocation-model/Makefile
@@ -4,7 +4,7 @@ all: others
        $(RUSTC) -C relocation-model=dynamic-no-pic foo.rs
        $(call RUN,foo)
 
-       $(RUSTC) -C relocation-model=default foo.rs
+       $(RUSTC) -C relocation-model=default -C link-arg=-no-pie foo.rs
        $(call RUN,foo)
 
        $(RUSTC) -C relocation-model=dynamic-no-pic --crate-type=dylib foo.rs --emit=link,obj
@@ -14,6 +14,6 @@ ifdef IS_MSVC
 others:
 else
 others:
-       $(RUSTC) -C relocation-model=static foo.rs
+       $(RUSTC) -C relocation-model=static -C link-arg=-no-pie foo.rs
        $(call RUN,foo)
 endif
