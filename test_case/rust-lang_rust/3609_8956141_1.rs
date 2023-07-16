
/Users/nmatsakis/tmp/foo.rs:15:32: 19:13 error: mismatched types: expected `SamplesFn` but found `fn~(&RingBuffer)` (lifetime mismatch)
/Users/nmatsakis/tmp/foo.rs:15         let callback: SamplesFn =
/Users/nmatsakis/tmp/foo.rs:16             |buffer|
/Users/nmatsakis/tmp/foo.rs:17             {
/Users/nmatsakis/tmp/foo.rs:18                 for buffer.len().timesi |i| {error!("%?: %f", i, buffer[i])}
/Users/nmatsakis/tmp/foo.rs:19             };
note: the static lifetime...
note: ...does not necessarily outlive lifetime re_bound(br_anon(0))
error: aborting due to previous error
