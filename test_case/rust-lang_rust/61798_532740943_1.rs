
  --> src\operations.rs:82:29
   |
82 |       async fn run(&mut self) {
   |  _____________________________^
83 | |         loop {
84 | |             match self.target_status {
85 | |                 Some(_) => self.update_status().await,
...  |
95 | |         }
96 | |     }
   | |_____^
error: unreachable expression
  --> src\operations.rs:82:29
   |
82 |       async fn run(&mut self) {
   |  _____________________________^
83 | |         loop {
84 | |             match self.target_status {
85 | |                 Some(_) => self.update_status().await,
...  |
95 | |         }
96 | |     }
   | |_____^

