
$ diff -us hir{1,2}
--- hir1    2016-01-14 12:06:39.000000000 -0500
+++ hir2    2016-01-14 12:06:47.000000000 -0500
@@ -16,8 +16,8 @@
                                             ),
                                             Spanned {
                                                 node: Ident {
-                                                    name: _result(81),
-                                                    unhygienic_name: _result(81)
+                                                    name: _result(83),
+                                                    unhygienic_name: _result(83)
                                                 },
                                                 span: Span { lo: BytePos(0), hi: BytePos(0), expn_id: ExpnId(4294967295) }
                                             },
@@ -192,8 +192,8 @@
                                                                     ),
                                                                     Spanned {
                                                                         node: Ident {
-                                                                            name: iter(72),
-                                                                            unhygienic_name: iter(72)
+                                                                            name: iter(82),
+                                                                            unhygienic_name: iter(82)
                                                                         },
                                                                         span: Span { lo: BytePos(0), hi: BytePos(0), expn_id: ExpnId(4294967295) }
                                                                     },
@@ -283,8 +283,8 @@
                                                                                                                 segments: [
                                                                                                                     PathSegment {
                                                                                                                         identifier: Ident {
-                                                                                                                            name: iter(72),
-                                                                                                                            unhygienic_name: iter(72)
+                                                                                                                            name: iter(82),
+                                                                                                                            unhygienic_name: iter(82)
                                                                                                                         },
                                                                                                                         parameters: AngleBracketedParameters(
                                                                                                                             AngleBracketedParameterData {
@@ -587,8 +587,8 @@
                             segments: [
                                 PathSegment {
                                     identifier: Ident {
-                                        name: _result(81),
-                                        unhygienic_name: _result(81)
+                                        name: _result(83),
+                                        unhygienic_name: _result(83)
                                     },
                                     parameters: AngleBracketedParameters(
                                         AngleBracketedParameterData {
@@ -613,4 +613,3 @@
     span: Span { lo: BytePos(0), hi: BytePos(0), expn_id: ExpnId(4294967295) },
     attrs: None
 [00m[37m}
-
