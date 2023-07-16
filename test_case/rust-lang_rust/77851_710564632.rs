
...

 error: use of `or_insert` followed by a function call
   --> $DIR/or_fun_call.rs:62:19
    |
 LL |     map.entry(42).or_insert(String::new());
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `or_insert_with(String::new)`
 
-error: use of `or_insert` followed by a function call
-  --> $DIR/or_fun_call.rs:65:21
-   |
-LL |     btree.entry(42).or_insert(String::new());
-   |                     ^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `or_insert_with(String::new)`
-
 error: use of `unwrap_or` followed by a function call
   --> $DIR/or_fun_call.rs:68:21
    |
 LL |     let _ = stringy.unwrap_or("".to_owned());
    |                     ^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| "".to_owned())`
 
 error: use of `or` followed by a function call
   --> $DIR/or_fun_call.rs:93:35
    |
 LL |     let _ = Some("a".to_string()).or(Some("b".to_string()));
    |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `or_else(|| Some("b".to_string()))`
 
 error: use of `or` followed by a function call
   --> $DIR/or_fun_call.rs:97:10
    |
 LL |         .or(Some(Bar(b, Duration::from_secs(2))));
    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `or_else(|| Some(Bar(b, Duration::from_secs(2))))`
 
-error: aborting due to 15 previous errors
+error: aborting due to 14 previous errors
