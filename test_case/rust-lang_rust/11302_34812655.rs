
Running /home/nmatsakis/versioned/rust-2/build/i686-unknown-linux-gnu/stage2/bin/rustc:
/home/nmatsakis/tmp/issue-11302.rs:6:11: 6:17 error: cannot borrow `a` as mutable more than once at a time
/home/nmatsakis/tmp/issue-11302.rs:6      push(&mut a);                
                                               ^~~~~~
/home/nmatsakis/tmp/issue-11302.rs:3:17: 5:7 note: previous borrow of `a` occurs here due to use in closure; the mutable borrow prevents subsequent moves, borrows, or modification of `a` until the borrow ends
/home/nmatsakis/tmp/issue-11302.rs:3      let push = |b: &mut ~[int]| {
/home/nmatsakis/tmp/issue-11302.rs:4          a.push(b[0]);            
/home/nmatsakis/tmp/issue-11302.rs:5      };                           
/home/nmatsakis/tmp/issue-11302.rs:7:3: 7:3 note: previous borrow ends here
/home/nmatsakis/tmp/issue-11302.rs:1  fn foo() {                       
...
/home/nmatsakis/tmp/issue-11302.rs:7  } 
                                      ^
error: aborting due to previous error
