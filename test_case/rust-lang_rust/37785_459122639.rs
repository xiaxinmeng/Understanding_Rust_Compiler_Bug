
error[E0597]: borrowed value does not live long enough
  --> src/main.rs:18:9
   |
18 |         Document { url: RefCell::new("hi".into()) }.url().clone()
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ temporary value does not live long enough
19 |     }
   |     - temporary value dropped here while still borrowed
   |
   = note: values in a scope are dropped in the opposite order they are created
