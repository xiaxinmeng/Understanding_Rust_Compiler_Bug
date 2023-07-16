
-               $(foreach base,$$(CTEST_SRC_BASE_$(4)), \
-               --src-base $$(S)src/test/$(base))/ \
+        $$(foreach base,$$(CTEST_SRC_BASE_$(4)), \
+          --src-base $$(S)src/test/$$(base))/ \
