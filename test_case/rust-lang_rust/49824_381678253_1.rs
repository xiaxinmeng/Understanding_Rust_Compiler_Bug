
[santiago@archlinux rust1 (master)]$ rustc +stage1 src/test/run-pass/issue-49824.rs
note: No external requirements
  --> src/test/run-pass/issue-49824.rs:18:9
   |
18 | /         || {
19 | |             let _y = &mut x;
20 | |         }
   | |_________^
   |
   = note: defining type: DefId(0/1:10 ~ issue_49824[317d]::main[0]::{{closure}}[0]::{{closure}}[0]) with closure substs [
               i16,
               extern "rust-call" fn(()),
               &mut i32
           ]

error: free region `` does not outlive free region `'_#1r`
  --> src/test/run-pass/issue-49824.rs:18:9
   |
18 | /         || {
19 | |             let _y = &mut x;
20 | |         }
   | |_________^

note: External requirements
  --> src/test/run-pass/issue-49824.rs:17:5
   |
17 | /     || {
18 | |         || {
19 | |             let _y = &mut x;
20 | |         }
21 | |     };
   | |_____^
   |
   = note: defining type: DefId(0/1:9 ~ issue_49824[317d]::main[0]::{{closure}}[0]) with closure substs [
               i16,
               extern "rust-call" fn(()) -> [closure@src/test/run-pass/issue-49824.rs:18:9: 20:10 x:&mut i32],
               &mut i32
           ]
   = note: number of external vids: 3
   = note: where '_#2r: '_#1r

note: No external requirements
  --> src/test/run-pass/issue-49824.rs:15:1
   |
15 | / fn main() {
16 | |     let mut x = 0;
17 | |     || {
18 | |         || {
...  |
21 | |     };
22 | | }
   | |_^
   |
   = note: defining type: DefId(0/0:3 ~ issue_49824[317d]::main[0]) with substs []

error: aborting due to previous error
