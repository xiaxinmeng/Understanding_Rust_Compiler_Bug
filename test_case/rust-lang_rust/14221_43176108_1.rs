
% rustc /tmp/e.rs && ./e
Hello World A
% rustc --cfg inner /tmp/e.rs && ./e
Hello World A
% rustc --cfg field /tmp/e.rs && ./e
Hello World A
% rustc --cfg inner --cfg field /tmp/e.rs && ./e
/tmp/e.rs:19:23: 19:24 error: unresolved enum variant, struct or const `C`
/tmp/e.rs:19         #[cfg(field)] C(_) => "C",
                                   ^
error: aborting due to previous error
% 
