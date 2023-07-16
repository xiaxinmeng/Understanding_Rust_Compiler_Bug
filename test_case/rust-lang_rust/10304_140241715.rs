
test.rs:9:9: 9:11 warning: pattern binding `V1` is named the same as one of the variants of the type `A::E` [E0170]
test.rs:9         V1 => 1,
                  ^~
test.rs:9:9: 9:11 help: run `rustc --explain E0170` to see a detailed explanation
test.rs:9:9: 9:11 help: if you meant to match on a variant, consider making the path in the pattern qualified: `A::E::V1`
test.rs:10:9: 10:11 warning: pattern binding `V2` is named the same as one of the variants of the type `A::E` [E0170]
test.rs:10         V2 => 2,
                   ^~
test.rs:10:9: 10:11 help: run `rustc --explain E0170` to see a detailed explanation
test.rs:10:9: 10:11 help: if you meant to match on a variant, consider making the path in the pattern qualified: `A::E::V2`
test.rs:10:9: 10:11 error: unreachable pattern [E0001]
test.rs:10         V2 => 2,
                   ^~
test.rs:10:9: 10:11 help: run `rustc --explain E0001` to see a detailed explanation
error: aborting due to previous error
