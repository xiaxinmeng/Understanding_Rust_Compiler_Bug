diff
 error[E0507]: cannot move out of indexed content
   --> $DIR/issue-40402-1.rs:19:13
    |
 19 |     let e = f.v[0];
-   |             ^^^^^^
-   |             |
-   |             cannot move out of indexed content
-   |             help: consider using a reference instead: `&f.v[0]`
+   |         -   ^^^^^^ cannot move out of indexed content
+   |         |
+   |         hint: to prevent move, use `ref e` or `ref mut e`
 
 error: aborting due to previous error
