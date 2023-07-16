plain

---- [ui] src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs stdout ----
diff of stderr:

16 LL |     ( $$ $a:ident ) => {
18 
18 
- note: `$$` and meta-variable expressions are not allowed inside macro parameter definitions
+ note: `$$`, `$crate`, and meta-variable expressions are not allowed inside macro parameter definitions
21    |
21    |
22 LL |     ( $$ $a:ident ) => {
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
23    |        ^
24 
+ error: unexpected token: $crate
+   --> $DIR/syntax-errors.rs:59:7
+   --> $DIR/syntax-errors.rs:59:7
+    |
+ LL |     ( $crate $a:ident ) => {
+ 
+ 
+ note: `$$`, `$crate`, and meta-variable expressions are not allowed inside macro parameter definitions
+    |
+    |
+ LL |     ( $crate $a:ident ) => {
+ 
25 error: unexpected token: a
-   --> $DIR/syntax-errors.rs:60:19
+   --> $DIR/syntax-errors.rs:66:19
+   --> $DIR/syntax-errors.rs:66:19
27    |
28 LL |         ${count() a b c}

30    |
31 note: meta-variable expression must not have trailing tokens
-   --> $DIR/syntax-errors.rs:60:19
-   --> $DIR/syntax-errors.rs:60:19
+   --> $DIR/syntax-errors.rs:66:19
33    |
34 LL |         ${count() a b c}

36 
37 error: unexpected token: a
-   --> $DIR/syntax-errors.rs:63:19
-   --> $DIR/syntax-errors.rs:63:19
+   --> $DIR/syntax-errors.rs:69:19
39    |
40 LL |         ${count(i a b c)}

42    |
43 note: meta-variable expression must not have trailing tokens
-   --> $DIR/syntax-errors.rs:63:19
-   --> $DIR/syntax-errors.rs:63:19
+   --> $DIR/syntax-errors.rs:69:19
45    |
46 LL |         ${count(i a b c)}

48 
49 error: unexpected token: a
-   --> $DIR/syntax-errors.rs:65:22
-   --> $DIR/syntax-errors.rs:65:22
+   --> $DIR/syntax-errors.rs:71:22
51    |
52 LL |         ${count(i, 1 a b c)}

54    |
55 note: meta-variable expression must not have trailing tokens
-   --> $DIR/syntax-errors.rs:65:22
-   --> $DIR/syntax-errors.rs:65:22
+   --> $DIR/syntax-errors.rs:71:22
57    |
58 LL |         ${count(i, 1 a b c)}

60 
61 error: unexpected token: a
-   --> $DIR/syntax-errors.rs:67:20
-   --> $DIR/syntax-errors.rs:67:20
+   --> $DIR/syntax-errors.rs:73:20
63    |
64 LL |         ${count(i) a b c}

66    |
67 note: meta-variable expression must not have trailing tokens
-   --> $DIR/syntax-errors.rs:67:20
-   --> $DIR/syntax-errors.rs:67:20
+   --> $DIR/syntax-errors.rs:73:20
69    |
70 LL |         ${count(i) a b c}

72 
73 error: unexpected token: a
-   --> $DIR/syntax-errors.rs:70:21
-   --> $DIR/syntax-errors.rs:70:21
+   --> $DIR/syntax-errors.rs:76:21
75    |
76 LL |         ${ignore(i) a b c}

78    |
79 note: meta-variable expression must not have trailing tokens
-   --> $DIR/syntax-errors.rs:70:21
-   --> $DIR/syntax-errors.rs:70:21
+   --> $DIR/syntax-errors.rs:76:21
81    |
82 LL |         ${ignore(i) a b c}

84 
85 error: unexpected token: a
-   --> $DIR/syntax-errors.rs:72:20
-   --> $DIR/syntax-errors.rs:72:20
+   --> $DIR/syntax-errors.rs:78:20
87    |
88 LL |         ${ignore(i a b c)}

90    |
91 note: meta-variable expression must not have trailing tokens
-   --> $DIR/syntax-errors.rs:72:20
-   --> $DIR/syntax-errors.rs:72:20
+   --> $DIR/syntax-errors.rs:78:20
93    |
94 LL |         ${ignore(i a b c)}

96 
97 error: unexpected token: a
-   --> $DIR/syntax-errors.rs:75:19
-   --> $DIR/syntax-errors.rs:75:19
+   --> $DIR/syntax-errors.rs:81:19
99    |
100 LL |         ${index() a b c}

102    |
103 note: meta-variable expression must not have trailing tokens
-   --> $DIR/syntax-errors.rs:75:19
-   --> $DIR/syntax-errors.rs:75:19
+   --> $DIR/syntax-errors.rs:81:19
105    |
106 LL |         ${index() a b c}

108 
109 error: unexpected token: a
-   --> $DIR/syntax-errors.rs:77:19
-   --> $DIR/syntax-errors.rs:77:19
+   --> $DIR/syntax-errors.rs:83:19
111    |
112 LL |         ${index(1 a b c)}

114    |
115 note: meta-variable expression must not have trailing tokens
-   --> $DIR/syntax-errors.rs:77:19
-   --> $DIR/syntax-errors.rs:77:19
+   --> $DIR/syntax-errors.rs:83:19
117    |
118 LL |         ${index(1 a b c)}

120 
121 error: unexpected token: a
-   --> $DIR/syntax-errors.rs:80:19
-   --> $DIR/syntax-errors.rs:80:19
+   --> $DIR/syntax-errors.rs:86:19
123    |
124 LL |         ${index() a b c}

126    |
127 note: meta-variable expression must not have trailing tokens
-   --> $DIR/syntax-errors.rs:80:19
-   --> $DIR/syntax-errors.rs:80:19
+   --> $DIR/syntax-errors.rs:86:19
129    |
130 LL |         ${index() a b c}

132 
133 error: unexpected token: a
-   --> $DIR/syntax-errors.rs:82:19
-   --> $DIR/syntax-errors.rs:82:19
+   --> $DIR/syntax-errors.rs:88:19
135    |
136 LL |         ${index(1 a b c)}

138    |
139 note: meta-variable expression must not have trailing tokens
-   --> $DIR/syntax-errors.rs:82:19
-   --> $DIR/syntax-errors.rs:82:19
+   --> $DIR/syntax-errors.rs:88:19
141    |
142 LL |         ${index(1 a b c)}

144 
145 error: meta-variable expression depth must be a literal
-   --> $DIR/syntax-errors.rs:89:33
-   --> $DIR/syntax-errors.rs:89:33
+   --> $DIR/syntax-errors.rs:95:33
147    |
148 LL |     ( $( $i:ident ),* ) => { ${ index(IDX) } };

150 
151 error: unexpected token: {
-   --> $DIR/syntax-errors.rs:95:8
-   --> $DIR/syntax-errors.rs:95:8
+   --> $DIR/syntax-errors.rs:101:8
153    |
154 LL |     ( ${ length() } ) => {

156 
156 
- note: `$$` and meta-variable expressions are not allowed inside macro parameter definitions
-   --> $DIR/syntax-errors.rs:95:8
+ note: `$$`, `$crate`, and meta-variable expressions are not allowed inside macro parameter definitions
159    |
159    |
160 LL |     ( ${ length() } ) => {

162 
162 
163 error: expected one of: `*`, `+`, or `?`
-   --> $DIR/syntax-errors.rs:95:8
165    |
165    |
166 LL |     ( ${ length() } ) => {

168 
169 error: expected identifier
-   --> $DIR/syntax-errors.rs:102:33
-   --> $DIR/syntax-errors.rs:102:33
+   --> $DIR/syntax-errors.rs:108:33
171    |
172 LL |     ( $( $i:ident ),* ) => { ${ ignore() } };

174 
175 error: only unsuffixes integer literals are supported in meta-variable expressions
-   --> $DIR/syntax-errors.rs:108:33
-   --> $DIR/syntax-errors.rs:108:33
+   --> $DIR/syntax-errors.rs:114:33
177    |
178 LL |     ( $( $i:ident ),* ) => { ${ index(1u32) } };

180 
181 error: meta-variable expression parameter must be wrapped in parentheses
-   --> $DIR/syntax-errors.rs:114:33
-   --> $DIR/syntax-errors.rs:114:33
+   --> $DIR/syntax-errors.rs:120:33
183    |
184 LL |     ( $( $i:ident ),* ) => { ${ count{i} } };

186 
187 error: expected identifier
-   --> $DIR/syntax-errors.rs:120:31
-   --> $DIR/syntax-errors.rs:120:31
+   --> $DIR/syntax-errors.rs:126:31
189    |
190 LL |     ( $( $i:ident ),* ) => { ${ {} } };

192 
193 error: unrecognized meta-variable expression
-   --> $DIR/syntax-errors.rs:140:33
-   --> $DIR/syntax-errors.rs:140:33
+   --> $DIR/syntax-errors.rs:146:33
195    |
196 LL |     ( $( $i:ident ),* ) => { ${ aaaaaaaaaaaaaa(i) } };
197    |                                 ^^^^^^^^^^^^^^ help: supported expressions are count, ignore, index and length
231    |                                    ^^
232 
233 error: expected expression, found `$`
-   --> $DIR/syntax-errors.rs:60:9
-   --> $DIR/syntax-errors.rs:60:9
+   --> $DIR/syntax-errors.rs:66:9
235    |
236 LL |         ${count() a b c}


242    = note: this error originates in the macro `extra_garbage_after_metavar` (in Nightly builds, run with -Z macro-backtrace for more info)
244 error: expected expression, found `$`
-   --> $DIR/syntax-errors.rs:89:30
+   --> $DIR/syntax-errors.rs:95:30
246    |
246    |
247 LL |     ( $( $i:ident ),* ) => { ${ index(IDX) } };


253    = note: this error originates in the macro `metavar_depth_is_not_literal` (in Nightly builds, run with -Z macro-backtrace for more info)
255 error: expected expression, found `$`
-   --> $DIR/syntax-errors.rs:102:30
+   --> $DIR/syntax-errors.rs:108:30
257    |
257    |
258 LL |     ( $( $i:ident ),* ) => { ${ ignore() } };


264    = note: this error originates in the macro `metavar_token_without_ident` (in Nightly builds, run with -Z macro-backtrace for more info)
266 error: expected expression, found `$`
-   --> $DIR/syntax-errors.rs:108:30
+   --> $DIR/syntax-errors.rs:114:30
268    |
268    |
269 LL |     ( $( $i:ident ),* ) => { ${ index(1u32) } };


275    = note: this error originates in the macro `metavar_with_literal_suffix` (in Nightly builds, run with -Z macro-backtrace for more info)
277 error: expected expression, found `$`
-   --> $DIR/syntax-errors.rs:114:30
+   --> $DIR/syntax-errors.rs:120:30
279    |
279    |
280 LL |     ( $( $i:ident ),* ) => { ${ count{i} } };


286    = note: this error originates in the macro `metavar_without_parens` (in Nightly builds, run with -Z macro-backtrace for more info)
288 error: expected expression, found `$`
-   --> $DIR/syntax-errors.rs:120:30
+   --> $DIR/syntax-errors.rs:126:30
290    |
290    |
291 LL |     ( $( $i:ident ),* ) => { ${ {} } };


297    = note: this error originates in the macro `open_brackets_without_tokens` (in Nightly builds, run with -Z macro-backtrace for more info)
299 error: variable `foo` is not recognized in meta-variable expression
-   --> $DIR/syntax-errors.rs:127:17
+   --> $DIR/syntax-errors.rs:133:17
301    |
301    |
302 LL |         ${count(foo)}

304 
304 
305 error: variable `bar` is not recognized in meta-variable expression
-   --> $DIR/syntax-errors.rs:134:18
307    |
307    |
308 LL |         ${ignore(bar)}

310 
311 error: expected expression, found `$`
-   --> $DIR/syntax-errors.rs:140:30
-   --> $DIR/syntax-errors.rs:140:30
+   --> $DIR/syntax-errors.rs:146:30
313    |
314 LL |     ( $( $i:ident ),* ) => { ${ aaaaaaaaaaaaaa(i) } };


375    = note: this error originates in the macro `no_curly__rhs_dollar__no_round` (in Nightly builds, run with -Z macro-backtrace for more info)
377 error[E0425]: cannot find value `a` in this scope
-   --> $DIR/syntax-errors.rs:153:37
+   --> $DIR/syntax-errors.rs:159:37
379    |
379    |
380 LL |     no_curly__rhs_dollar__no_round!(a);

382 
- error: aborting due to 40 previous errors
+ error: aborting due to 41 previous errors
---
To only update this specific test, also pass `--test-args macros/rfc-3086-metavar-expr/syntax-errors.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/rfc-3086-metavar-expr/syntax-errors" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/rfc-3086-metavar-expr/syntax-errors/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected identifier, found `$`
   |
   |
LL |     ( $( $i:ident ),* ) => { ${ count($i) } };
   |                                 ^^^^^ - help: try removing `$`
error: expected identifier, found `$`
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:23:26
   |
   |
LL |     ( $i:ident ) => { ${ count($i) } };
   |                          ^^^^^ - help: try removing `$`
error: unexpected token: $
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:53:8
   |
   |
LL |     ( $$ $a:ident ) => {


note: `$$`, `$crate`, and meta-variable expressions are not allowed inside macro parameter definitions
   |
   |
LL |     ( $$ $a:ident ) => {

error: unexpected token: $crate
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:59:7
   |
   |
LL |     ( $crate $a:ident ) => {


note: `$$`, `$crate`, and meta-variable expressions are not allowed inside macro parameter definitions
   |
   |
LL |     ( $crate $a:ident ) => {

error: unexpected token: a
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:66:19
   |
   |
LL |         ${count() a b c}
   |
note: meta-variable expression must not have trailing tokens
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:66:19
   |
   |
LL |         ${count() a b c}

error: unexpected token: a
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:69:19
   |
   |
LL |         ${count(i a b c)}
   |
note: meta-variable expression must not have trailing tokens
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:69:19
   |
   |
LL |         ${count(i a b c)}

error: unexpected token: a
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:71:22
   |
   |
LL |         ${count(i, 1 a b c)}
   |
note: meta-variable expression must not have trailing tokens
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:71:22
   |
   |
LL |         ${count(i, 1 a b c)}

error: unexpected token: a
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:73:20
   |
   |
LL |         ${count(i) a b c}
   |
note: meta-variable expression must not have trailing tokens
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:73:20
   |
   |
LL |         ${count(i) a b c}

error: unexpected token: a
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:76:21
   |
   |
LL |         ${ignore(i) a b c}
   |
note: meta-variable expression must not have trailing tokens
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:76:21
   |
   |
LL |         ${ignore(i) a b c}

error: unexpected token: a
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:78:20
   |
   |
LL |         ${ignore(i a b c)}
   |
note: meta-variable expression must not have trailing tokens
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:78:20
   |
   |
LL |         ${ignore(i a b c)}

error: unexpected token: a
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:81:19
   |
   |
LL |         ${index() a b c}
   |
note: meta-variable expression must not have trailing tokens
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:81:19
   |
   |
LL |         ${index() a b c}

error: unexpected token: a
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:83:19
   |
   |
LL |         ${index(1 a b c)}
   |
note: meta-variable expression must not have trailing tokens
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:83:19
   |
   |
LL |         ${index(1 a b c)}

error: unexpected token: a
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:86:19
   |
   |
LL |         ${index() a b c}
   |
note: meta-variable expression must not have trailing tokens
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:86:19
   |
   |
LL |         ${index() a b c}

error: unexpected token: a
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:88:19
   |
   |
LL |         ${index(1 a b c)}
   |
note: meta-variable expression must not have trailing tokens
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:88:19
   |
   |
LL |         ${index(1 a b c)}

error: meta-variable expression depth must be a literal
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:95:33
   |
   |
LL |     ( $( $i:ident ),* ) => { ${ index(IDX) } };

error: unexpected token: {
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:101:8
   |
   |
LL |     ( ${ length() } ) => {


note: `$$`, `$crate`, and meta-variable expressions are not allowed inside macro parameter definitions
   |
   |
LL |     ( ${ length() } ) => {


error: expected one of: `*`, `+`, or `?`
   |
   |
LL |     ( ${ length() } ) => {

error: expected identifier
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:108:33
   |
   |
LL |     ( $( $i:ident ),* ) => { ${ ignore() } };

error: only unsuffixes integer literals are supported in meta-variable expressions
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:114:33
   |
   |
LL |     ( $( $i:ident ),* ) => { ${ index(1u32) } };

error: meta-variable expression parameter must be wrapped in parentheses
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:120:33
   |
   |
LL |     ( $( $i:ident ),* ) => { ${ count{i} } };

error: expected identifier
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:126:31
   |
   |
LL |     ( $( $i:ident ),* ) => { ${ {} } };

error: unrecognized meta-variable expression
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:146:33
   |
   |
LL |     ( $( $i:ident ),* ) => { ${ aaaaaaaaaaaaaa(i) } };
   |                                 ^^^^^^^^^^^^^^ help: supported expressions are count, ignore, index and length

error: `count` can not be placed inside the inner-most repetition
   |
   |
LL |     ( $i:ident ) => { ${ count(i) } };

error: expected expression, found `$`
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:17:30
   |
   |
LL |     ( $( $i:ident ),* ) => { ${ count($i) } };
...
...
LL |     curly__rhs_dollar__round!(a, b, c);
   |
   |
   = note: this error originates in the macro `curly__rhs_dollar__round` (in Nightly builds, run with -Z macro-backtrace for more info)
error: expected expression, found `$`
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:23:23
   |
   |
LL |     ( $i:ident ) => { ${ count($i) } };
...
...
LL |     curly__rhs_dollar__no_round!(a);
   |
   |
   = note: this error originates in the macro `curly__rhs_dollar__no_round` (in Nightly builds, run with -Z macro-backtrace for more info)
error: variable 'i' is still repeating at this depth
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:41:36
   |
   |
LL |     ( $( $i:ident ),* ) => { count($i) };

error: expected expression, found `$`
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:66:9
   |
   |
LL |         ${count() a b c}
...
...
LL |     extra_garbage_after_metavar!(a);
   |
   |
   = note: this error originates in the macro `extra_garbage_after_metavar` (in Nightly builds, run with -Z macro-backtrace for more info)
error: expected expression, found `$`
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:95:30
   |
   |
LL |     ( $( $i:ident ),* ) => { ${ index(IDX) } };
...
...
LL |     metavar_depth_is_not_literal!(a);
   |
   |
   = note: this error originates in the macro `metavar_depth_is_not_literal` (in Nightly builds, run with -Z macro-backtrace for more info)
error: expected expression, found `$`
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:108:30
   |
   |
LL |     ( $( $i:ident ),* ) => { ${ ignore() } };
...
...
LL |     metavar_token_without_ident!(a);
   |
   |
   = note: this error originates in the macro `metavar_token_without_ident` (in Nightly builds, run with -Z macro-backtrace for more info)
error: expected expression, found `$`
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:114:30
   |
   |
LL |     ( $( $i:ident ),* ) => { ${ index(1u32) } };
...
...
LL |     metavar_with_literal_suffix!(a);
   |
   |
   = note: this error originates in the macro `metavar_with_literal_suffix` (in Nightly builds, run with -Z macro-backtrace for more info)
error: expected expression, found `$`
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:120:30
   |
   |
LL |     ( $( $i:ident ),* ) => { ${ count{i} } };
...
...
LL |     metavar_without_parens!(a);
   |
   |
   = note: this error originates in the macro `metavar_without_parens` (in Nightly builds, run with -Z macro-backtrace for more info)
error: expected expression, found `$`
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:126:30
   |
   |
LL |     ( $( $i:ident ),* ) => { ${ {} } };
...
...
LL |     open_brackets_without_tokens!(a);
   |
   |
   = note: this error originates in the macro `open_brackets_without_tokens` (in Nightly builds, run with -Z macro-backtrace for more info)
error: variable `foo` is not recognized in meta-variable expression
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:133:17
   |
   |
LL |         ${count(foo)}


error: variable `bar` is not recognized in meta-variable expression
   |
   |
LL |         ${ignore(bar)}

error: expected expression, found `$`
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:146:30
   |
   |
LL |     ( $( $i:ident ),* ) => { ${ aaaaaaaaaaaaaa(i) } };
...
...
LL |     unknown_metavar!(a);
   |
   |
   = note: this error originates in the macro `unknown_metavar` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0425]: cannot find function `count` in this scope
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:29:30
   |
   |
LL |     ( $( $i:ident ),* ) => { count(i) };
...
...
LL |     no_curly__no_rhs_dollar__round!(a, b, c);
   |
   |
   = note: this error originates in the macro `no_curly__no_rhs_dollar__round` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0425]: cannot find value `i` in this scope
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:29:36
   |
   |
LL |     ( $( $i:ident ),* ) => { count(i) };
...
...
LL |     no_curly__no_rhs_dollar__round!(a, b, c);
   |
   |
   = note: this error originates in the macro `no_curly__no_rhs_dollar__round` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0425]: cannot find function `count` in this scope
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:35:23
   |
   |
LL |     ( $i:ident ) => { count(i) };
...
...
LL |     no_curly__no_rhs_dollar__no_round!(a);
   |
   |
   = note: this error originates in the macro `no_curly__no_rhs_dollar__no_round` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0425]: cannot find value `i` in this scope
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:35:29
   |
   |
LL |     ( $i:ident ) => { count(i) };
...
...
LL |     no_curly__no_rhs_dollar__no_round!(a);
   |
   |
   = note: this error originates in the macro `no_curly__no_rhs_dollar__no_round` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0425]: cannot find function `count` in this scope
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:46:23
   |
   |
LL |     ( $i:ident ) => { count($i) };
...
...
LL |     no_curly__rhs_dollar__no_round!(a);
   |
   |
   = note: this error originates in the macro `no_curly__rhs_dollar__no_round` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0425]: cannot find value `a` in this scope
  --> /checkout/src/test/ui/macros/rfc-3086-metavar-expr/syntax-errors.rs:159:37
   |
   |
LL |     no_curly__rhs_dollar__no_round!(a);

error: aborting due to 41 previous errors

For more information about this error, try `rustc --explain E0425`.
