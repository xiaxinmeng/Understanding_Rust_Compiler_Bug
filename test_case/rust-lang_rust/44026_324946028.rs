diff
diff --git a/src/etc/htmldocck.py b/src/etc/htmldocck.py
index a5449b748d..c8246bf727 100644
--- a/src/etc/htmldocck.py
+++ b/src/etc/htmldocck.py
@@ -110,11 +110,17 @@ import os.path
 import re
 import shlex
 from collections import namedtuple
-from HTMLParser import HTMLParser
+try:
+    from html.parser import HTMLParser
+except ImportError:
+    from HTMLParser import HTMLParser
 from xml.etree import cElementTree as ET
 
 # &larrb;/&rarrb; are not in HTML 4 but are in HTML 5
-from htmlentitydefs import entitydefs
+try:
+    from html.entities import entitydefs
+except ImportError:
+    from htmlentitydefs import entitydefs
 entitydefs['larrb'] = u'\u21e4'
 entitydefs['rarrb'] = u'\u21e5'
 entitydefs['nbsp'] = ' '
