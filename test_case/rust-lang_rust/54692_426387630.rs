bash
> rustc ~/tmp/issue-54692.rs   -Zverbose
note: External requirements
 --> /home/nmatsakis/tmp/issue-54692.rs:6:5
  |
6 | /     |x: &'a i32| -> &'static i32 {
7 | |         return &x;
8 | |     };
  | |_____^
  |
  = note: defining type: DefId(0/1:10 ~ issue_54692[317d]::foo[0]::{{closure}}[0]) with closure substs [
              i8,
              extern "rust-call" fn((&'_#1r i32,)) -> &'_#2r i32
          ]
  = note: late-bound region is '_#3r
  = note: number of external vids: 4
  = note: where '_#1r: '_#2r
