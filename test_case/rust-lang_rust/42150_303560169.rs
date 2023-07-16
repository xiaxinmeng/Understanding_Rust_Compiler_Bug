
[01:04:45] ---- [ui] ui/suggestions/confuse-field-and-method/private-field.rs stdout ----
[01:04:45] 	ui: /checkout/src/test/ui/suggestions/confuse-field-and-method/private-field.rs
[01:04:45] normalized stderr:
[01:04:45] error: no method named `dog_age` found for type `animal::Dog` in the current scope
[01:04:45]   --> $DIR/private-field.rs:26:23
[01:04:45]    |
[01:04:45] 26 |     let dog_age = dog.dog_age();
[01:04:45]    |                       ^^^^^^^ private field, not a method
[01:04:45] 
[01:04:45] error: aborting due to previous error(s)
[01:04:45] 
[01:04:45] 
[01:04:45] 
[01:04:45] expected stderr:
[01:04:45] error: no method named `dog_age` found for type `animal::Dog` in the current scope
[01:04:45]   --> $DIR/private-field.rs:26:23
[01:04:45]    |
[01:04:45] 26 |     let dog_age = dog.dog_age();
[01:04:45]    |                       ^^^^^^^ private field, not a method
[01:04:45] 
[01:04:45] error: aborting due to previous error
[01:04:45] 
[01:04:45] 
[01:04:45] 
[01:04:45] diff of stderr:
[01:04:45] 
[01:04:45]  error: no method named `dog_age` found for type `animal::Dog` in the current scope
[01:04:45]    --> $DIR/private-field.rs:26:23
[01:04:45]     |
[01:04:45]  26 |     let dog_age = dog.dog_age();
[01:04:45]     |                       ^^^^^^^ private field, not a method
[01:04:45]  
[01:04:45] -error: aborting due to previous error
[01:04:45] +error: aborting due to previous error(s)
