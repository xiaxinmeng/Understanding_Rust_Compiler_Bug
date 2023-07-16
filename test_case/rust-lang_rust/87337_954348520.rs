
 error: binding's name is too similar to existing binding
   --> $DIR/similar_names.rs:72:9
    |
 LL |     let parsee: i32;
    |         ^^^^^^
    |
 note: existing binding defined here
   --> $DIR/similar_names.rs:70:9
    |
 LL |     let parser: i32;
    |         ^^^^^^
-help: separate the discriminating character by an underscore like: `parse_e`
-  --> $DIR/similar_names.rs:72:9
-   |
-LL |     let parsee: i32;
-   |         ^^^^^^
