
failures:

---- [ui] src/test/ui/type/issue-103271.rs stdout ----
diff of stderr:

7	   = note: the following trait bounds were not satisfied:
8	           `&[_]: ExactSizeIterator`
9	           which is required by `&mut &[_]: ExactSizeIterator`
-	   = help: items from traits can only be used if the trait is in scope
-	help: the following trait is implemented but not in scope; perhaps add a `use` for it:
-	   |
-	LL | use object::read::read_ref::ReadRef;
-	   |
15	help: the function `len` is implemented on `[_]`
16	   |
17	LL |     let length = <[_]>::len;
