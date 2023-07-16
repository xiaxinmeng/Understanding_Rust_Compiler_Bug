diff
- let read_line = |_context| -> Poll<String> {
+ let read_line = |_context: &mut _| -> Poll<String> {
