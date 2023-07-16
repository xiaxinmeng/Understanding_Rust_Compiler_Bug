diff
- if let Some(res) = Some(item) {
+ if let Some(res) = Some(&mut *item) {
