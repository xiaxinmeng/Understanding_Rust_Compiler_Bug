 shell
$ rustc main.rs
main.rs:7:22: 7:29 error: unsupported cyclic reference between types/traits detected
main.rs:7     I: Iterator<Item=I::Item>
                               ^~~~~~~
note: the cycle begins when computing the bounds for type parameter `I`...
note: ...which then again requires computing the bounds for type parameter `I`, completing the cycle.
error: aborting due to previous error
