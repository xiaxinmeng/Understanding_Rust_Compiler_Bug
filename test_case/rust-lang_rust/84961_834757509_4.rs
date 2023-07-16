

Mismatch at tests/source/macro_rules.rs:1:
 // rustfmt-format_macro_matchers: true
 
 
 macro_rules! m {
-    () => {};
-    ($x:ident) => {};
-    ($m1:ident, $m2:ident, $x:ident) => {};
-    ($($beginning:ident),*; $middle:ident; $($end:ident),*) => {};
-    (
-        $($beginning:ident),*;
-        $middle:ident;
-        $($end:ident),*;
-        $($beginning:ident),*;
-        $middle:ident;
-        $($end:ident),*
+    () => ();
+    ( $ x : ident ) => ();
+    ( $ m1 : ident , $ m2 : ident , $ x : ident ) => ();
+    ( $($beginning:ident),*;$middle:ident;$($end:ident),* ) => ();
+    ( $($beginning: ident),*; $middle: ident; $($end: ident),*; $($beginning: ident),*; $middle: ident; $($end: ident),* ) => {};
+    ( $ name : ident ( $ ( $ dol : tt $ var : ident ) * ) $ ( $ body : tt ) * ) => ();
+    ( $( $ i : ident : $ ty : ty , $def : expr , $stb : expr , $ ( $ dstring : tt ) , + ) ; + $ ( ; ) *
+      $( $ i : ident : $ ty : ty , $def : expr , $stb : expr , $ ( $ dstring : tt ) , + ) ; + $ ( ; ) *
     ) => {};
-    ($name:ident($($dol:tt $var:ident)*) $($body:tt)*) => {};
-    (
-        $($i:ident : $ty:ty, $def:expr, $stb:expr, $($dstring:tt),+);+ $(;)*
-        $($i:ident : $ty:ty, $def:expr, $stb:expr, $($dstring:tt),+);+ $(;)*
-    ) => {};
-    ($foo:tt foo[$attr:meta] $name:ident) => {};
-    ($foo:tt[$attr:meta] $name:ident) => {};
-    ($foo:tt &'a[$attr:meta] $name:ident) => {};
-    ($foo:tt foo #[$attr:meta] $name:ident) => {};
-    ($foo:tt #[$attr:meta] $name:ident) => {};
-    ($foo:tt &'a #[$attr:meta] $name:ident) => {};
-    ($x:tt foo bar foo bar foo bar $y:tt => x * y * z $z:tt, $($a:tt),*) => {};
+    ( $foo: tt foo [$ attr : meta] $name: ident ) => {};
+    ( $foo: tt [$ attr: meta] $name: ident ) => {};
+    ( $foo: tt &'a [$attr : meta] $name: ident ) => {};
+    ( $foo: tt foo # [ $attr : meta] $name: ident ) => {};
+    ( $foo: tt # [ $attr : meta] $name: ident) => {};
+    ( $foo: tt &'a # [ $attr : meta] $name: ident ) => {};
+    ( $ x : tt foo bar foo bar foo bar $ y : tt => x*y*z $ z : tt , $ ( $a: tt ) , * ) => {};
 
 macro_rules! impl_a_method {

Mismatch at tests/source/macro_rules.rs:31:
Mismatch at tests/source/macro_rules.rs:31:
-    ($n:ident($a:ident : $ta:ty) -> $ret:ty { $body:expr }) => {
-        fn $n($a: $ta) -> $ret {
-            $body
-        macro_rules! $n {
-            ($va: expr) => {
-            ($va: expr) => {
-                $n($va)
-        }
-        }
+    ($n:ident ( $a:ident : $ta:ty ) -> $ret:ty { $body:expr }) => {
+        fn $n($a:$ta) -> $ret { $body }
+        macro_rules! $n { ($va:expr) => { $n($va) } }
     };
-    ($n:ident($a:ident : $ta:ty, $b:ident : $tb:ty) -> $ret:ty { $body:expr }) => {
-        fn $n($a: $ta, $b: $tb) -> $ret {
-            $body
-        macro_rules! $n {
-        macro_rules! $n {
-            ($va: expr,$vb: expr) => {
-                $n($va, $vb)
-        }
-        }
+    ($n:ident ( $a:ident : $ta:ty, $b:ident : $tb:ty ) -> $ret:ty { $body:expr }) => {
+        fn $n($a:$ta, $b:$tb) -> $ret { $body }
+        macro_rules! $n { ($va:expr, $vb:expr) => { $n($va, $vb) } }
-    (
-    (
-        $n:ident($a:ident : $ta:ty, $b:ident : $tb:ty, $c:ident : $tc:ty) -> $ret:ty { $body:expr }
-    ) => {
-        fn $n($a: $ta, $b: $tb, $c: $tc) -> $ret {
-            $body
-        macro_rules! $n {
-        macro_rules! $n {
-            ($va: expr,$vb: expr,$vc: expr) => {
-                $n($va, $vb, $vc)
-        }
-        }
+    ($n:ident ( $a:ident : $ta:ty, $b:ident : $tb:ty, $c:ident : $tc:ty ) -> $ret:ty { $body:expr }) => {
+        fn $n($a:$ta, $b:$tb, $c:$tc) -> $ret { $body }
+        macro_rules! $n { ($va:expr, $vb:expr, $vc:expr) => { $n($va, $vb, $vc) } }
-    (
-    (
-        $n:ident($a:ident : $ta:ty, $b:ident : $tb:ty, $c:ident : $tc:ty, $d:ident : $td:ty) ->
-        $ret:ty { $body:expr }
-    ) => {
-        fn $n($a: $ta, $b: $tb, $c: $tc, $d: $td) -> $ret {
-            $body
-        macro_rules! $n {
-        macro_rules! $n {
-            ($va: expr,$vb: expr,$vc: expr,$vd: expr) => {
-                $n($va, $vb, $vc, $vd)
-        }
-        }
+    ($n:ident ( $a:ident : $ta:ty, $b:ident : $tb:ty, $c:ident : $tc:ty, $d:ident : $td:ty ) -> $ret:ty { $body:expr }) => {
+        fn $n($a:$ta, $b:$tb, $c:$tc, $d:$td) -> $ret { $body }
+        macro_rules! $n { ($va:expr, $vb:expr, $vc:expr, $vd:expr) => { $n($va, $vb, $vc, $vd) } }
 }
 

Mismatch at tests/source/macro_rules.rs:78:
Mismatch at tests/source/macro_rules.rs:78:
 macro_rules! m {
-    // a
-    ($expr:expr, $($func:ident)*) => {{
-        let x = $expr;
-        $func(x)
-    }};
+ // a
+ ($expr :expr,  $( $func : ident    ) *   ) => {
+  {
+  let    x =    $expr;
+                         $func (
+           )
+ }
+ };
 
 
-    /* b */
-    () => {
-        /* c */
-    };
+    /* b */
 
-    (@tag) => {};
+    ()           => {/* c */};
-    // d
-    // d
-    ($item:ident) => {
-        mod macro_item {
-            struct $item;
-    };
-    };
+       (@tag)   =>
+
+       };
+
+// d
+// d
+( $item:ident  ) =>      {
+ mod macro_item    {  struct $item ; }
 }
 
 macro m2 {


Mismatch at tests/source/macro_rules.rs:101:
-    // a
-    ($expr:expr, $($func:ident)*) => {{
-        let x = $expr;
-        $func(x)
+ // a
+ // a
+ ($expr :expr,  $( $func : ident    ) *   ) => {
+  {
+  let    x =    $expr;
+                         $func (
+           )
+ }
+ }
 
 
-    /* b */
-    () => {
-        /* c */
-    }
+    /* b */
 
-    (@tag) => {}
+    ()           => {/* c */}
-    // d
-    // d
-    ($item:ident) => {
-        mod macro_item {
-            struct $item;
-    }
-    }
+       (@tag)   =>
+
+       }
+
+// d
+// d
+( $item:ident  ) =>      {
+ mod macro_item    {  struct $item ; }
+}
 
 // #2438, #2476
 macro_rules! m {
 macro_rules! m {

Mismatch at tests/source/macro_rules.rs:124:
     () => {
         fn foo() {
-            this_line_is_98_characters_long_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx();
+            this_line_is_98_characters_long_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx(
         }
-    };
+    }
 }
 }
 macro_rules! m {
     () => {

Mismatch at tests/source/macro_rules.rs:132:
         fn foo() {
             this_line_is_99_characters_long_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx(
+);
         }
     };
 }
 }

Mismatch at tests/source/macro_rules.rs:139:
     () => {
         fn foo() {
             this_line_is_100_characters_long_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx(
+);
         }
     };
 }
 }

Mismatch at tests/source/macro_rules.rs:154:
 
 // #2439
 macro_rules! m {
-    (
-        $line0_xxxxxxxxxxxxxxxxx:expr,
-        $line1_xxxxxxxxxxxxxxxxx:expr,
-        $line2_xxxxxxxxxxxxxxxxx:expr,
-        $line3_xxxxxxxxxxxxxxxxx:expr,
-    ) => {};
+    ($line0_xxxxxxxxxxxxxxxxx: expr, $line1_xxxxxxxxxxxxxxxxx: expr, $line2_xxxxxxxxxxxxxxxxx: expr, $line3_xxxxxxxxxxxxxxxxx: expr,) => {};
 
 // #2466

Mismatch at tests/source/macro_rules.rs:172:
Mismatch at tests/source/macro_rules.rs:172:
 ];
 
 // #2470
-macro foo($type_name:ident, $docs:expr) {
+macro foo($type_name: ident, $docs: expr) {
     #[allow(non_camel_case_types)]
     #[doc=$docs]
     #[derive(Debug, Clone, Copy)]
Mismatch at tests/source/macro_rules.rs:198:
                 line = line,
             ));
         }
         }
-    }};
+    }}
 }
 
 // #2560

Mismatch at tests/source/macro_rules.rs:205:
 macro_rules! binary {
-    ($_self:ident, $expr:expr, $lhs:expr, $func:ident) => {
+    ($_self:ident,$expr:expr, $lhs:expr,$func:ident) => {
         while $_self.matched($expr) {
             let op = $_self.get_binary_op()?;

Mismatch at tests/source/macro_rules.rs:210:
Mismatch at tests/source/macro_rules.rs:210:
             let rhs = Box::new($_self.$func()?);
 
-            $lhs = Spanned {
+           $lhs = Spanned {
                 span: $lhs.get_span().to(rhs.get_span()),
                 value: Expression::Binary {
                     lhs: Box::new($lhs),
Mismatch at tests/source/macro_rules.rs:230:
 // #2749
 macro_rules! foo {
 macro_rules! foo {
     ($(x)* {}) => {};
-    ($(x)*()) => {};
-    ($(x)*[]) => {};
+    ($(x)* ()) => {};
+    ($(x)* []) => {};
 }
 macro_rules! __wundergraph_expand_sqlite_mutation {
-    (
-        $mutation_name:ident $((context = $($context:tt)*))* {
-            $(
-                $entity_name:ident(
-                    $(insert = $insert:ident,)*
-                    $(update = $update:ident,)*
-                    $(delete = $($delete:tt)+)*
-            )*
-        }
-    ) => {};
-    ) => {};
+    ( $mutation_name:ident $((context = $($context:tt)*))*{ $( $entity_name:ident( $(insert = $insert:ident,)* $(update = $update:ident,)* $(delete = $($delete:tt)+)* ), )* } ) => {};
 
 // #2607

Mismatch at tests/source/macro_rules.rs:311:
Mismatch at tests/source/macro_rules.rs:311:
     {
         macro_rules! touch_value {
             ($func:ident, $value:expr) => {{
-                let result = API::get_cached().$func(
-                    key.as_ptr(),
-                    $value,
-                    $value,
-                    ffi::VSPropAppendMode::paTouch,
-                );
+                let result = API::get_cached().$func(self, key.as_ptr(), $value, ffi::VSPropAppendMode::paTouch);
                 let result = API::get_cached().$func(self, key.as_ptr(), $value, ffi::VSPropAppend);
-                let result =
-                    API::get_cached().$func(self, key.as_ptr(), $value, ffi::VSPropAppendM);
-                let result =
-                    APIIIIIIIII::get_cached().$func(self, key.as_ptr(), $value, ffi::VSPropAppendM);
-                let result = API::get_cached().$func(
-                    key.as_ptr(),
-                    $value,
-                    $value,
-                    ffi::VSPropAppendMMMMMMMMMM,
-                );
+                let result = API::get_cached().$func(self, key.as_ptr(), $value, ffi::VSPropAppendM);
+                let result = APIIIIIIIII::get_cached().$func(self, key.as_ptr(), $value, ffi::VSPropAppendM);
+                let result = API::get_cached().$func(self, key.as_ptr(), $value, ffi::VSPropAppendMMMMMMMMMM);
                 debug_assert!(result == 0);
             }};

Mismatch at tests/source/macro_rules.rs:336:
 
 // #2642
 // #2642
 macro_rules! template {
-    ($name:expr) => {
-            r##"
+    ($name: expr) => {
+        format_args!(r##"
 "http://example.com"
---
-"##,
-            $name
-        )
-    };
+"##, $name)
 }
 
 macro_rules! template {

---

Mismatch at tests/source/issue-2520.rs:2:
 // rustfmt-format_code_in_doc_comments: true
 
 //! 