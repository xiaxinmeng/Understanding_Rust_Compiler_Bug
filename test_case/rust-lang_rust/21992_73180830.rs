
$ git diff
diff --git a/mk/docs.mk b/mk/docs.mk
index 547ad96..71b2533 100644
--- a/mk/docs.mk
+++ b/mk/docs.mk
@@ -87,7 +87,7 @@ endif

 # Check for xelatex

-ifeq($(CFG_XELATEX),)
+ifeq ($(CFG_XELATEX),)
     CFG_LATEX := $(CFG_XELATEX)
     XELATEX = 1
   else
