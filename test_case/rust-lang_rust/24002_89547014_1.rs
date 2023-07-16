
/home/nmatsakis/tmp/lifetime-test-1.rs:11:17: 11:33 error: borrowed value does not live long enough
/home/nmatsakis/tmp/lifetime-test-1.rs:11     for line in std::io::stdin().lock().lines().map(|l| l.unwrap()) {
                                                          ^~~~~~~~~~~~~~~~
/home/nmatsakis/tmp/lifetime-test-1.rs:11:5: 19:2 note: reference must be valid for the destruction scope surrounding statement at 11:4...
/home/nmatsakis/tmp/lifetime-test-1.rs:11     for line in std::io::stdin().lock().lines().map(|l| l.unwrap()) {
/home/nmatsakis/tmp/lifetime-test-1.rs:12         println!("{}", line.split(' ')
/home/nmatsakis/tmp/lifetime-test-1.rs:13                  .map(|seg| if map.contains_key(seg) {
/home/nmatsakis/tmp/lifetime-test-1.rs:14                      map[seg.as_slice()].to_string()
/home/nmatsakis/tmp/lifetime-test-1.rs:15                  } else {
/home/nmatsakis/tmp/lifetime-test-1.rs:16                      seg.to_string()
                                          ...
/home/nmatsakis/tmp/lifetime-test-1.rs:11:5: 19:2 note: ...but borrowed value is only valid for the statement at 11:4
/home/nmatsakis/tmp/lifetime-test-1.rs:11     for line in std::io::stdin().lock().lines().map(|l| l.unwrap()) {
/home/nmatsakis/tmp/lifetime-test-1.rs:12         println!("{}", line.split(' ')
/home/nmatsakis/tmp/lifetime-test-1.rs:13                  .map(|seg| if map.contains_key(seg) {
/home/nmatsakis/tmp/lifetime-test-1.rs:14                      map[seg.as_slice()].to_string()
/home/nmatsakis/tmp/lifetime-test-1.rs:15                  } else {
/home/nmatsakis/tmp/lifetime-test-1.rs:16                      seg.to_string()
                                          ...
/home/nmatsakis/tmp/lifetime-test-1.rs:11:5: 19:2 help: consider using a `let` binding to increase its lifetime
/home/nmatsakis/tmp/lifetime-test-1.rs:11     for line in std::io::stdin().lock().lines().map(|l| l.unwrap()) {
/home/nmatsakis/tmp/lifetime-test-1.rs:12         println!("{}", line.split(' ')
/home/nmatsakis/tmp/lifetime-test-1.rs:13                  .map(|seg| if map.contains_key(seg) {
/home/nmatsakis/tmp/lifetime-test-1.rs:14                      map[seg.as_slice()].to_string()
/home/nmatsakis/tmp/lifetime-test-1.rs:15                  } else {
/home/nmatsakis/tmp/lifetime-test-1.rs:16                      seg.to_string()
                                          ...
