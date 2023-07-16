
---- [ui] ui/suggestions/suggest-move-types.rs stdout ----
diff of stderr:

108	LL | struct Al<'a, T, M: OneWithLifetime<A=(), T, 'a>> {
109	   |                                           ^
110	   |
-	   = note: lifetime arguments must be provided before type arguments
+	   = note: type arguments must be provided before lifetime arguments ABC
112	
113	error[E0747]: type provided when a lifetime was expected
114	  --> $DIR/suggest-move-types.rs:48:71

116	LL | struct Bl<'a, 'b, 'c, T, U, V, M: ThreeWithLifetime<A=(), B=(), C=(), T, U, V, 'a, 'b, 'c>> {
117	   |                                                                       ^
118	   |
-	   = note: lifetime arguments must be provided before type arguments
+	   = note: type arguments must be provided before lifetime arguments ABC
120	
121	error[E0747]: lifetime provided when a type was expected
122	  --> $DIR/suggest-move-types.rs:65:56

124	LL | struct Cl<'a, 'b, 'c, T, U, V, M: ThreeWithLifetime<T, 'a, A=(), B=(), C=(), U, 'b, V, 'c>> {
125	   |                                                        ^^
126	   |
-	   = note: lifetime arguments must be provided before type arguments
+	   = note: lifetime arguments must be provided before type arguments ABC
128	
129	error[E0747]: lifetime provided when a type was expected
130	  --> $DIR/suggest-move-types.rs:82:56

132	LL | struct Dl<'a, 'b, 'c, T, U, V, M: ThreeWithLifetime<T, 'a, A=(), B=(), U, 'b, C=(), V, 'c>> {
133	   |                                                        ^^
134	   |
-	   = note: lifetime arguments must be provided before type arguments
+	   = note: lifetime arguments must be provided before type arguments ABC
136	
137	error: aborting due to 12 previous errors
138	


The actual stderr differed from the expected stderr.
