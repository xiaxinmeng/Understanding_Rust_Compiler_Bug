
[00:43:39] diff of stderr:
[00:43:39] 
[00:43:39] 2   --> $DIR/issue-20261.rs:14:11
[00:43:39] 3    |
[00:43:39] 4 LL |     for (ref i,) in [].iter() {
[00:43:39] -    |         -------- consider giving `__next` a type
[00:43:39] +    |                     --------- the element type for this iterator is not specified
[00:43:39] 6 LL |         i.clone();
[00:43:39] 7    |           ^^^^^ cannot infer type for `_`
[00:43:39] 
