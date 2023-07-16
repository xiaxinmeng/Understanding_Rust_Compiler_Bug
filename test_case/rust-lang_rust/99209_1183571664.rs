diff
diff --git a/src/librustdoc/html/templates/page.html b/src/librustdoc/html/templates/page.html
index dfb3e4e6a2c..db45253ef69 100644
--- a/src/librustdoc/html/templates/page.html
+++ b/src/librustdoc/html/templates/page.html
@@ -39,7 +39,7 @@
     {%- else if page.css_class == "source" -%}
     <script defer src="{{static_root_path|safe}}source-script{{page.resource_suffix}}.js"></script> {#- -#}
     <script defer src="{{page.root_path|safe}}source-files{{page.resource_suffix}}.js"></script> {#- -#}
-    {%- else -%}
+    {%- else if page.css_class != "mod" -%}
     <script defer src="sidebar-items{{page.resource_suffix}}.js"></script> {#- -#}
     {%- endif -%}
     <script defer src="{{static_root_path|safe}}main{{page.resource_suffix}}.js"></script> {#- -#}
