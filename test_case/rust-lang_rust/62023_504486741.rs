diff
diff --git a/src/tools/publish_toolstate.py b/src/tools/publish_toolstate.py
index 9e7c18b7f5..c7bfd8a1ab 100755
--- a/src/tools/publish_toolstate.py
+++ b/src/tools/publish_toolstate.py
@@ -164,6 +164,8 @@ def update_latest(
                         tool, new, MAINTAINERS.get(tool, ''),
                         relevant_pr_number, relevant_pr_user, pr_reviewer,
                     )
+                except urllib2.HTTPError as e:
+                    print("HTTPError: {0} {1}".format(e, e.read()))
                 except IOError as e:
                     # network errors will simply end up not creating an issue, but that's better
                     # than failing the entire build job
