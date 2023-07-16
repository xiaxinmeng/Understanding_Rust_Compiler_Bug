
error[E0597]: borrowed value does not live long enough (Mir)
  --> /mnt/c/Users/David/Projects/personal/rust/src/test/ui/issue-46472.rs:14:10
   |
14 |     &mut 4
   |          ^ temporary value does not live long enough
...
17 | }
   | - temporary value only lives until here
   |
