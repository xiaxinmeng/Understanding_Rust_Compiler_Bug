diff
 error[E0597]: `z` does not live long enough
   --> $DIR/regions-escape-loop-via-vec.rs:7:17
    |
 LL |         _y.push(&mut z);
-   |         --      ^^^^^^ borrowed value does not live long enough
-   |         |
+   |         --------^^^^^^-
+   |         |       |
+   |         |       borrowed value does not live long enough
    |         borrow later used here
