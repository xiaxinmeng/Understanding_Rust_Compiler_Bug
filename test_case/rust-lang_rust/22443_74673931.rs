 diff
 struct Digit<T> {
     elem: T,
 }

 enum Node<T> {}

 enum FingerTree<T> {
     Single(T),
     Deep(
-        Digit<T>,
         Box<FingerTree<Node<T>>>,
+        Digit<T>,
     )
 }

 fn main() {
     let ft = FingerTree::Single(1);
 }
