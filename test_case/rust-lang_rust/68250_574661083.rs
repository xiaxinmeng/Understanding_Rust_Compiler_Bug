
warning: irrefutable if-let pattern
 --> src/main.rs:3:5
  |
3 | /     if let wrong_doing = Some(3) { // if let Some(3) = wrong_doing
4 | |        println!("Three!!");
5 | |     } else {
6 | |         println!("Not Three");
7 | |     }
  | |_____^
