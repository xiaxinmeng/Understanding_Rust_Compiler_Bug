diff
-trait A where Self::B: Q, <Self::B as Q>::U: From<usize> {
+trait A {
+    type U: From<usize>;
+    type B: Q<U=Self::U>;
 
     fn b() -> Self::B;
 }
