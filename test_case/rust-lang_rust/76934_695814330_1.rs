diff
-	error: unresolved link to `path::to::nonexistent::module`
-	  --> $DIR/intra-link-errors.rs:7:6
+	error: unresolved link to ``
+	  --> $DIR/intra-link-errors.rs:5:6
3	   |
-	LL | /// [path::to::nonexistent::module]
-	   |      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no item named `path` in `intra_link_errors`
+	LL | /// [<invalid syntax>]
+	   |      ^^^^^^^^^^^^^^^^ no item named `` in `intra_link_errors`
6	   |
7	note: the lint level is defined here
8	  --> $DIR/intra-link-errors.rs:1:9

9	   |
10	LL | #![deny(broken_intra_doc_links)]
11	   |         ^^^^^^^^^^^^^^^^^^^^^^
+	   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
12	
+	error: unresolved link to `path::to::nonexistent::module`
+	  --> $DIR/intra-link-errors.rs:7:6
+	   |
+	LL | /// [path::to::nonexistent::module]
+	   |      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no item named `path` in `intra_link_errors`
+	
13	error: unresolved link to `path::to::nonexistent::macro`
14	  --> $DIR/intra-link-errors.rs:11:6
15	   |

112	   |      this link resolves to the macro `m`, which is not in the value namespace
113	   |      help: to link to the macro, add an exclamation mark: `m!`
114	
-	error: aborting due to 16 previous errors
+	error: aborting due to 17 previous errors
116	
117
