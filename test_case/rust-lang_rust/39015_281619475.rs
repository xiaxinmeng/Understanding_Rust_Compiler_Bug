
--- a/src/test/run-make/issue-24445/Makefile
+++ b/src/test/run-make/issue-24445/Makefile
@@ -3,9 +3,9 @@
 ifeq ($(UNAME),Linux)
 all:
 	$(RUSTC) foo.rs
-	$(CC) foo.c -lfoo -L $(TMPDIR) -Wl,--gc-sections -lpthread -o $(TMPDIR)/foo
+	$(CC) foo.c -lfoo -L $(TMPDIR) -Wl,--gc-sections -lpthread -ldl -o $(TMPDIR)/foo
 	$(call RUN,foo)
-	$(CC) foo.c -lfoo -L $(TMPDIR) -Wl,--gc-sections -lpthread -pie -fPIC -o $(TMPDIR)/foo
+	$(CC) foo.c -lfoo -L $(TMPDIR) -Wl,--gc-sections -lpthread -ldl -pie -fPIC -o $(TMPDIR)/foo
 	$(call RUN,foo)
 else
 all:
