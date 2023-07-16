 diff
commit 0b215c3286c0dc7c2b2569c65b08c07c7e0d302d
Author: Adrien Tétar <adri-from-59 at hotmail period fr>
Date:   Sun Feb 2 09:56:02 2014 +0100

    etc/tidy: pass triple information to SNAP

diff --git a/mk/tests.mk b/mk/tests.mk
index d50f987..e44c046 100644
--- a/mk/tests.mk
+++ b/mk/tests.mk
@@ -255,13 +255,13 @@ tidy:
        | grep '^$(S)src/llvm' -v \
        | grep '^$(S)src/gyp' -v \
        | xargs -n 10 $(CFG_PYTHON) $(S)src/etc/tidy.py
-       $(Q)find $(S)src/etc -name '*.py' \
+       $(CFG_BUILD) $(Q)find $(S)src/etc -name '*.py' \
        | xargs -n 10 $(CFG_PYTHON) $(S)src/etc/tidy.py
-       $(Q)echo $(ALL_CS) \
+       $(CFG_BUILD) $(Q)echo $(ALL_CS) \
        | xargs -n 10 $(CFG_PYTHON) $(S)src/etc/tidy.py
-       $(Q)echo $(ALL_HS) \
+       $(CFG_BUILD) $(Q)echo $(ALL_HS) \
        | xargs -n 10 $(CFG_PYTHON) $(S)src/etc/tidy.py
-       $(Q)find $(S)src -type f -perm +111 \
+       $(CFG_BUILD) $(Q)find $(S)src -type f -perm +111 \
            -not -name '*.rs' -and -not -name '*.py' \
            -and -not -name '*.sh' \
        | grep '^$(S)src/llvm' -v \
diff --git a/src/etc/tidy.py b/src/etc/tidy.py
index 3364ddc..1a5d2ac 100644
--- a/src/etc/tidy.py
+++ b/src/etc/tidy.py
@@ -33,8 +33,8 @@ def do_license_check(name, contents):
     if not check_license(name, contents):
         report_error_name_no(name, 1, "incorrect license")

-
-file_names = [s for s in sys.argv[1:] if (not s.endswith("_gen.rs"))
+triple = sys.argv[1]
+file_names = [s for s in sys.argv[2:] if (not s.endswith("_gen.rs"))
                                      and (not ".#" in s)]

 current_name = ""
@@ -58,7 +58,7 @@ try:
             match = re.match(r'^.*//\s*SNAP\s+(\w+)', line)
             if match:
                 hsh = match.group(1)
-                a, b, c, phash = snapshot.determine_curr_snapshot_info()
+                a, b, c, phash = snapshot.determine_curr_snapshot_info(triple)
                 if not phash.startswith(hsh):
                     report_err("Snapshot out of date: " + line)
             else:
