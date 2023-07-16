
% ./build/x86_64-unknown-linux-gnu/stage1/bin/rustc ../../issue-49824-reduced.rs  -Z borrowck=mir -Z verbose
note: No external requirements
  --> ../../issue-49824-reduced.rs:8:9
   |
8  | /         || {
9  | |             let _y = &mut x;
10 | |         }
   | |_________^
   |
   = note: defining type: DefId(0/1:10 ~ issue_49824_reduced[317d]::main[0]::{{closure}}[0]::{{closure}}[0]) with closure substs [
               i16,
               extern "rust-call" fn(()),
               &'_#1r mut i32
           ]

error: free region `ReFree(DefId(0/1:9 ~ issue_49824_reduced[317d]::main[0]::{{closure}}[0]), BrEnv)` does not outlive free region `'_#1r`
  --> ../../issue-49824-reduced.rs:8:9
   |
8  | /         || {
9  | |             let _y = &mut x;
10 | |         }
   | |_________^

note: External requirements
  --> ../../issue-49824-reduced.rs:7:5
   |
7  | /     || {
8  | |         || {
9  | |             let _y = &mut x;
10 | |         }
11 | |     };
   | |_____^
   |
   = note: defining type: DefId(0/1:9 ~ issue_49824_reduced[317d]::main[0]::{{closure}}[0]) with closure substs [
               i16,
               extern "rust-call" fn(()) -> [closure@../../issue-49824-reduced.rs:8:9: 10:10 x:&'_#1r mut i32],
               &'_#2r mut i32
           ]
   = note: number of external vids: 3
   = note: where '_#2r: '_#1r

note: No external requirements
  --> ../../issue-49824-reduced.rs:5:1
   |
5  | / fn main() {
6  | |     let mut x = 0;
7  | |     || {
8  | |         || {
...  |
11 | |     };
12 | | }
   | |_^
   |
   = note: defining type: DefId(0/0:3 ~ issue_49824_reduced[317d]::main[0]) with substs []

error: aborting due to previous error
