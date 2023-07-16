
---- [ui] ui/suggestions/issue-64252-self-type.rs stdout ----
diff of stderr:

-	error: expected one of `:`, `@`, or `|`, found `<`
-	  --> $DIR/issue-64252-self-type.rs:4:15
+	error: `"Box"` is not a keyword
+	  --> $DIR/issue-64252-self-type.rs:4:12
3	   |
4	LL | pub fn foo(Box<Self>) { }
-	   |               ^ expected one of `:`, `@`, or `|`
+	   |            ^^^ try `"box"` instead
6	   |
7	   = note: anonymous parameters are removed in the 2018 edition (see RFC 1685)
8	help: if this is a `self` type, give it a parameter name

14	LL | pub fn foo(_: Box<Self>) { }
15	   |            ^^^^^^
16	
-	error: expected one of `:`, `@`, or `|`, found `<`
-	  --> $DIR/issue-64252-self-type.rs:10:15
+	error: `"Box"` is not a keyword
+	  --> $DIR/issue-64252-self-type.rs:10:12
19	   |
20	LL |     fn bar(Box<Self>) { }
-	   |               ^ expected one of `:`, `@`, or `|`
+	   |            ^^^ try `"box"` instead
22	   |
23	   = note: anonymous parameters are removed in the 2018 edition (see RFC 1685)
24	help: if this is a `self` type, give it a parameter name
