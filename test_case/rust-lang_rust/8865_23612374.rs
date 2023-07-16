
diff --git a/mk/target.mk b/mk/target.mk
index 0d798f4..ac7a1b8 100644
--- a/mk/target.mk
+++ b/mk/target.mk
@@ -32,7 +32,8 @@ define TARGET_STAGE_N

 $$(TLIB$(1)_T_$(2)_H_$(3))/libmorestack.a: \
        rt/$(2)/stage$(1)/arch/$$(HOST_$(2))/libmorestack.a \
-       | $$(TLIB$(1)_T_$(2)_H_$(3))/
+       | $$(TLIB$(1)_T_$(2)_H_$(3))/ \
+         $(HBIN0_H_$(CFG_BUILD_TRIPLE))/rustc$(X_$(CFG_BUILD_TRIPLE))
    @$$(call E, cp: $$@)
    $$(Q)cp $$< $$@

