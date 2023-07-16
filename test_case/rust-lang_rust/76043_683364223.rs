
---- [ui] ui/suggestions/appropriate-type-param-turbofish.rs stdout ----
diff of stderr:

32	   |                                                ^^^^^^^^^^^^^^^^^^^^^^^
33	LL |         vec!["a", "b", "c"].into_iter().collect::<hashbrown::set::HashSet<_, _>>();
34	   |                                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-	LL |         vec!["a", "b", "c"].into_iter().collect::<std::borrow::Cow<'_, [_]>>();
+	LL |         vec!["a", "b", "c"].into_iter().collect::<std::borrow::Cow<'_, str>>();
36	   |                                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
37	     and 10 other candidates
38	

51	   |                                                ^^^^^^^^^^^^^^^^^^^^^^^
52	LL |         vec!['a', 'b', 'c'].into_iter().collect::<hashbrown::set::HashSet<_, _>>();
53	   |                                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-	LL |         vec!['a', 'b', 'c'].into_iter().collect::<std::borrow::Cow<'_, str>>();
+	LL |         vec!['a', 'b', 'c'].into_iter().collect::<std::borrow::Cow<'_, [_]>>();
55	   |                                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
56	     and 10 other candidates
57	

108	   |              ^^^^^^^^^^^^^^^^^^^^^
109	LL |         let _: hashbrown::set::HashSet<_, _> = vec!['a', 'b', 'c'].into_iter().collect();
110	   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-	LL |         let _: std::borrow::Cow<'_, str> = vec!['a', 'b', 'c'].into_iter().collect();
+	LL |         let _: std::borrow::Cow<'_, [_]> = vec!['a', 'b', 'c'].into_iter().collect();
112	   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^
113	     and 10 other candidates
