
---- [ui] ui/str/str-mut-idx.rs stdout ----
diff of stderr:

39	   |
40	   = help: the trait `SliceIndex<str>` is not implemented for `{integer}`
41	   = note: you can use `.chars().nth()` or `.bytes().nth()`
-	           For more information, see chapter 8 in The Book: <https://doc.rust-lang.org/book/ch08-02-strings.html#indexing-into-strings>
+	           for more information, see chapter 8 in The Book: <https://doc.rust-lang.org/book/ch08-02-strings.html#indexing-into-strings>
43	
44	error[E0277]: the type `str` cannot be indexed by `{integer}`
45	  --> $DIR/str-mut-idx.rs:11:25

49	   |
50	   = help: the trait `SliceIndex<str>` is not implemented for `{integer}`
51	   = note: you can use `.chars().nth()` or `.bytes().nth()`
-	           For more information, see chapter 8 in The Book: <https://doc.rust-lang.org/book/ch08-02-strings.html#indexing-into-strings>
+	           for more information, see chapter 8 in The Book: <https://doc.rust-lang.org/book/ch08-02-strings.html#indexing-into-strings>
53	
54	error[E0277]: the type `str` cannot be indexed by `char`
55	  --> $DIR/str-mut-idx.rs:13:5
