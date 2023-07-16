
fn() -> () @ src/main.rs:2:21: 2:26
fn(_) -> _ @ src/main.rs:2:21: 2:26
fn(u32, u32) -> u32 @ src/main.rs:2:21: 2:26
lambda(u32, u32) -> u32 @ src/main.rs:2:21: 2:26
closure fn(u32, u32) -> u32 @ src/main.rs:2:21: 2:26
closure(u32, u32) -> u32 @ src/main.rs:2:21: 2:26
closure fn(u32, u32) -> u32 from src/main.rs:2:21: 2:26
fn(u32, u32) -> u32  # Given that in type errors the found type is always highlighted,
                     # the span for a closure in found type wouldn't need to be displayed.
                     # I don't believe there's a case where a specific closure would
                     # be the expected type.
fn(u32, u32) -> u32 { [src/main.rs:2:21: 2:26] }
