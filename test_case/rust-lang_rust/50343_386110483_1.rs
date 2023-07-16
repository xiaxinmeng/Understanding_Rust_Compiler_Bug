rust
error[E0597]: `**dur` does not live long enough
   --> src/timer.rs:164:40
    |
162 |             .map(|client| client.take(u64::from(TICKS)).collect().and_then(|_| {
    |                                                                            --- capture occurs here
163 |                     let elapsed = start.elapsed();
164 |                     assert!(elapsed >= dur * TICKS * 2 / 3);
    |                                        ^^^ borrowed value does not live long enough
...
168 |             .collect::<Vec<_>>();
    |                                - borrowed value only lives until here
...
173 |     }
    |     - borrowed value needs to live until here

