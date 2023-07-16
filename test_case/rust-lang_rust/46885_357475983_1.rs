
error[E0597]: borrowed value does not live long enough (Ast)
  --> /mnt/c/Users/David/Projects/personal/rust/src/test/ui/issue-46472.rs:14:10
   |
14 |       &mut 4
   |            ^ temporary value does not live long enough
15 |       //~^ ERROR borrowed value does not live long enough (Ast) [E0597]
16 |       //~| ERROR borrowed value does not live long enough (Mir) [E0597]
   |  ______________________________________________________________________-
17 | | }
   | |_ temporary value only lives until here
   |
