plain
[01:38:31] failures:
[01:38:31] 
[01:38:31] ---- [ui] ui/for_loop.rs stdout ----
[01:38:31] normalized stderr:
[01:38:31] error: this range is empty so this for loop will never run
[01:38:31]    |
[01:38:31] LL |     for i in 10..0 {
[01:38:31]    |              ^^^^^
[01:38:31]    |
[01:38:31]    |
[01:38:31]    = note: `-D clippy::reverse-range-loop` implied by `-D warnings`
[01:38:31] help: consider using the following if you are attempting to iterate over this range in reverse
[01:38:31]    |
[01:38:31] LL |     for i in (0..10).rev() {
[01:38:31] 
[01:38:31] 
[01:38:31] error: this range is empty so this for loop will never run
[01:38:31]    |
[01:38:31] LL |     for i in 10..=0 {
[01:38:31]    |              ^^^^^^
[01:38:31] help: consider using the following if you are attempting to iterate over this range in reverse
[01:38:31] help: consider using the following if you are attempting to iterate over this range in reverse
[01:38:31]    |
[01:38:31] LL |     for i in (0...10).rev() {
[01:38:31] 
[01:38:31] 
[01:38:31] error: this range is empty so this for loop will never run
[01:38:31]    |
[01:38:31] LL |     for i in MAX_LEN..0 {
[01:38:31]    |              ^^^^^^^^^^
[01:38:31] help: consider using the following if you are attempting to iterate over this range in reverse
[01:38:31] help: consider using the following if you are attempting to iterate over this range in reverse
[01:38:31]    |
[01:38:31] LL |     for i in (0..MAX_LEN).rev() {
[01:38:31] 
[01:38:31] 
[01:38:31] error: this range is empty so this for loop will never run
[01:38:31]    |
[01:38:31] LL |     for i in 5..5 {
[01:38:31]    |              ^^^^
[01:38:31] 
[01:38:31] 
[01:38:31] error: this range is empty so this for loop will never run
[01:38:31]    |
[01:38:31] LL |     for i in 10..5 + 4 {
[01:38:31]    |              ^^^^^^^^^
[01:38:31] help: consider using the following if you are attempting to iterate over this range in reverse
[01:38:31] help: consider using the following if you are attempting to iterate over this range in reverse
[01:38:31]    |
[01:38:31] LL |     for i in (5 + 4..10).rev() {
[01:38:31] 
[01:38:31] 
[01:38:31] error: this range is empty so this for loop will never run
[01:38:31]    |
[01:38:31]    |
[01:38:31] LL |     for i in (5 + 2)..(3 - 1) {
[01:38:31] help: consider using the following if you are attempting to iterate over this range in reverse
[01:38:31]    |
[01:38:31]    |
[01:38:31] LL |     for i in ((3 - 1)..(5 + 2)).rev() {
[01:38:31] 
[01:38:31] 
[01:38:31] error: this range is empty so this for loop will never run
[01:38:31]    |
[01:38:31]    |
[01:38:31] LL |     for i in (5 + 2)..(8 - 1) {
[01:38:31] 
[01:38:31] error: it is more concise to loop over references to containers instead of using explicit iteration methods
[01:38:31]   --> $DIR/for_loop.rs:107:15
[01:38:31]    |
---
[01:38:31] 
[01:38:31] error: it is more concise to loop over containers instead of using explicit iteration methods`
[01:38:31]   --> $DIR/for_loop.rs:112:15
[01:38:31]    |
[01:38:31] LL |     for _v in out_vec.into_iter() {}
[01:38:31]    |               ^^^^^^^^^^^^^^^^^^^ help: to write this more concisely, try: `out_vec`
[01:38:31]    = note: `-D clippy::explicit-into-iter-loop` implied by `-D warnings`
[01:38:31] 
[01:38:31] error: it is more concise to loop over references to containers instead of using explicit iteration methods
[01:38:31]   --> $DIR/for_loop.rs:115:15
[01:38:31]   --> $DIR/for_loop.rs:115:15
[01:38:31]    |
[01:38:31] LL |     for _v in array.into_iter() {}
[01:38:31]    |               ^^^^^^^^^^^^^^^^^ help: to write this more concisely, try: `&array`
[01:38:31] 
[01:38:31] error: it is more concise to loop over references to containers instead of using explicit iteration methods
[01:38:31]   --> $DIR/for_loop.rs:120:15
[01:38:31]    |
[01:38:31] LL |     for _v in [1, 2, 3].iter() {}
[01:38:31]    |               ^^^^^^^^^^^^^^^^ help: to write this more concisely, try: `&[1, 2, 3]`
[01:38:31] error: it is more concise to loop over references to containers instead of using explicit iteration methods
[01:38:31]   --> $DIR/for_loop.rs:124:15
[01:38:31]    |
[01:38:31]    |
[01:38:31] LL |     for _v in [0; 32].iter() {}
[01:38:31]    |               ^^^^^^^^^^^^^^ help: to write this more concisely, try: `&[0; 32]`
[01:38:31] error: it is more concise to loop over references to containers instead of using explicit iteration methods
[01:38:31]   --> $DIR/for_loop.rs:129:15
[01:38:31]    |
[01:38:31]    |
[01:38:31] LL |     for _v in ll.iter() {}
[01:38:31]    |               ^^^^^^^^^ help: to write this more concisely, try: `&ll`
[01:38:31] error: it is more concise to loop over references to containers instead of using explicit iteration methods
[01:38:31]   --> $DIR/for_loop.rs:132:15
[01:38:31]    |
[01:38:31]    |
[01:38:31] LL |     for _v in vd.iter() {}
[01:38:31]    |               ^^^^^^^^^ help: to write this more concisely, try: `&vd`
[01:38:31] error: it is more concise to loop over references to containers instead of using explicit iteration methods
[01:38:31]   --> $DIR/for_loop.rs:135:15
[01:38:31]    |
[01:38:31]    |
[01:38:31] LL |     for _v in bh.iter() {}
[01:38:31]    |               ^^^^^^^^^ help: to write this more concisely, try: `&bh`
[01:38:31] error: it is more concise to loop over references to containers instead of using explicit iteration methods
[01:38:31]   --> $DIR/for_loop.rs:138:15
[01:38:31]    |
[01:38:31]    |
[01:38:31] LL |     for _v in hm.iter() {}
[01:38:31]    |               ^^^^^^^^^ help: to write this more concisely, try: `&hm`
[01:38:31] error: it is more concise to loop over references to containers instead of using explicit iteration methods
[01:38:31]   --> $DIR/for_loop.rs:141:15
[01:38:31]    |
[01:38:31]    |
[01:38:31] LL |     for _v in bt.iter() {}
[01:38:31]    |               ^^^^^^^^^ help: to write this more concisely, try: `&bt`
[01:38:31] error: it is more concise to loop over references to containers instead of using explicit iteration methods
[01:38:31]   --> $DIR/for_loop.rs:144:15
[01:38:31]    |
[01:38:31]    |
[01:38:31] LL |     for _v in hs.iter() {}
[01:38:31]    |               ^^^^^^^^^ help: to write this more concisely, try: `&hs`
[01:38:31] error: it is more concise to loop over references to containers instead of using explicit iteration methods
[01:38:31]   --> $DIR/for_loop.rs:147:15
[01:38:31]    |
[01:38:31]    |
[01:38:31] LL |     for _v in bs.iter() {}
[01:38:31]    |               ^^^^^^^^^ help: to write this more concisely, try: `&bs`
[01:38:31] error: you are iterating over `Iterator::next()` which is an Option; this will compile but is probably not what you want
[01:38:31]   --> $DIR/for_loop.rs:149:15
[01:38:31]    |
[01:38:31] LL |     for _v in vec.iter().next() {}
[01:38:31] LL |     for _v in vec.iter().next() {}
[01:38:31]    |               ^^^^^^^^^^^^^^^^^
[01:38:31]    |
[01:38:31]    = note: `-D clippy::iter-next-loop` implied by `-D warnings`
[01:38:31] error: you are collect()ing an iterator and throwing away the result. Consider using an explicit for loop to exhaust the iterator
[01:38:31]   --> $DIR/for_loop.rs:156:5
[01:38:31]    |
[01:38:31]    |
[01:38:31] LL |     vec.iter().cloned().map(|x| out.push(x)).collect::<Vec<_>>();
[01:38:31]    |
[01:38:31]    = note: `-D clippy::unused-collect` implied by `-D warnings`
[01:38:31] 
[01:38:31] error: aborting due to 22 previous errors
[01:38:31] error: aborting due to 22 previous errors
[01:38:31] 
[01:38:31] 
[01:38:31] 
[01:38:31] expected stderr:
[01:38:31] error: this range is empty so this for loop will never run
[01:38:31]    |
[01:38:31] LL |     for i in 10..0 {
[01:38:31]    |              ^^^^^
[01:38:31]    |
[01:38:31]    |
[01:38:31]    = note: `-D clippy::reverse-range-loop` implied by `-D warnings`
[01:38:31] help: consider using the following if you are attempting to iterate over this range in reverse
[01:38:31]    |
[01:38:31] LL |     for i in (0..10).rev() {
[01:38:31] 
[01:38:31] 
[01:38:31] error: this range is empty so this for loop will never run
[01:38:31]    |
[01:38:31] LL |     for i in 10..=0 {
[01:38:31]    |              ^^^^^^
[01:38:31] help: consider using the following if you are attempting to iterate over this range in reverse
[01:38:31] help: consider using the following if you are attempting to iterate over this range in reverse
[01:38:31]    |
[01:38:31] LL |     for i in (0...10).rev() {
[01:38:31] 
[01:38:31] 
[01:38:31] error: this range is empty so this for loop will never run
[01:38:31]    |
[01:38:31] LL |     for i in MAX_LEN..0 {
[01:38:31]    |              ^^^^^^^^^^
[01:38:31] help: consider using the following if you are attempting to iterate over this range in reverse
[01:38:31] help: consider using the following if you are attempting to iterate over this range in reverse
[01:38:31]    |
[01:38:31] LL |     for i in (0..MAX_LEN).rev() {
[01:38:31] 
[01:38:31] 
[01:38:31] error: this range is empty so this for loop will never run
[01:38:31]    |
[01:38:31] LL |     for i in 5..5 {
[01:38:31]    |              ^^^^
[01:38:31] 
[01:38:31] 
[01:38:31] error: this range is empty so this for loop will never run
[01:38:31]    |
[01:38:31] LL |     for i in 10..5 + 4 {
[01:38:31]    |              ^^^^^^^^^
[01:38:31] help: consider using the following if you are attempting to iterate over this range in reverse
[01:38:31] help: consider using the following if you are attempting to iterate over this range in reverse
[01:38:31]    |
[01:38:31] LL |     for i in (5 + 4..10).rev() {
[01:38:31] 
[01:38:31] 
[01:38:31] error: this range is empty so this for loop will never run
[01:38:31]    |
[01:38:31]    |
[01:38:31] LL |     for i in (5 + 2)..(3 - 1) {
[01:38:31] help: consider using the following if you are attempting to iterate over this range in reverse
[01:38:31]    |
[01:38:31]    |
[01:38:31] LL |     for i in ((3 - 1)..(5 + 2)).rev() {
[01:38:31] 
[01:38:31] 
[01:38:31] error: this range is empty so this for loop will never run
[01:38:31]    |
[01:38:31]    |
[01:38:31] LL |     for i in (5 + 2)..(8 - 1) {
[01:38:31] 
[01:38:31] error: it is more concise to loop over references to containers instead of using explicit iteration methods
[01:38:31]   --> $DIR/for_loop.rs:107:15
[01:38:31]    |
---
[01:38:31] 
[01:38:31] error: it is more concise to loop over containers instead of using explicit iteration methods`
[01:38:31]   --> $DIR/for_loop.rs:112:15
[01:38:31]    |
[01:38:31] LL |     for _v in out_vec.into_iter() {}
[01:38:31]    |
[01:38:31]    = note: `-D clippy::explicit-into-iter-loop` implied by `-D warnings`
[01:38:31] 
[01:38:31] error: it is more concise to loop over references to containers instead of using explicit iteration methods
---
[01:38:31] 
[01:38:31] error: it is more concise to loop over references to containers instead of using explicit iteration methods
[01:38:31]   --> $DIR/for_loop.rs:120:15
[01:38:31]    |
[01:38:31] LL |     for _v in [1, 2, 3].iter() {}
[01:38:31] 
[01:38:31] error: it is more concise to loop over references to containers instead of using explicit iteration methods
[01:38:31]   --> $DIR/for_loop.rs:124:15
[01:38:31]    |
[01:38:31]    |
[01:38:31] LL |     for _v in [0; 32].iter() {}
[01:38:31] 
[01:38:31] error: it is more concise to loop over references to containers instead of using explicit iteration methods
[01:38:31]   --> $DIR/for_loop.rs:129:15
[01:38:31]    |
[01:38:31]    |
[01:38:31] LL |     for _v in ll.iter() {}
[01:38:31] 
[01:38:31] error: it is more concise to loop over references to containers instead of using explicit iteration methods
[01:38:31]   --> $DIR/for_loop.rs:132:15
[01:38:31]    |
[01:38:31]    |
[01:38:31] LL |     for _v in vd.iter() {}
[01:38:31] 
[01:38:31] error: it is more concise to loop over references to containers instead of using explicit iteration methods
[01:38:31]   --> $DIR/for_loop.rs:135:15
[01:38:31]    |
[01:38:31]    |
[01:38:31] LL |     for _v in bh.iter() {}
[01:38:31] 
[01:38:31] error: it is more concise to loop over references to containers instead of using explicit iteration methods
[01:38:31]   --> $DIR/for_loop.rs:138:15
[01:38:31]    |
[01:38:31]    |
[01:38:31] LL |     for _v in hm.iter() {}
[01:38:31] 
[01:38:31] error: it is more concise to loop over references to containers instead of using explicit iteration methods
[01:38:31]   --> $DIR/for_loop.rs:141:15
[01:38:31]    |
[01:38:31]    |
[01:38:31] LL |     for _v in bt.iter() {}
[01:38:31] 
[01:38:31] error: it is more concise to loop over references to containers instead of using explicit iteration methods
[01:38:31]   --> $DIR/for_loop.rs:144:15
[01:38:31]    |
[01:38:31]    |
[01:38:31] LL |     for _v in hs.iter() {}
[01:38:31] 
[01:38:31] error: it is more concise to loop over references to containers instead of using explicit iteration methods
[01:38:31]   --> $DIR/for_loop.rs:147:15
[01:38:31]    |
[01:38:31]    |
[01:38:31] LL |     for _v in bs.iter() {}
[01:38:31] 
[01:38:31] error: you are iterating over `Iterator::next()` which is an Option; this will compile but is probably not what you want
[01:38:31]   --> $DIR/for_loop.rs:149:15
[01:38:31]    |
[01:38:31]    |
[01:38:31] LL |     for _v in vec.iter().next() {}
[01:38:31]    |               ^^^^^^^^^^^^^^^^^
[01:38:31]    |
[01:38:31]    = note: `-D clippy::iter-next-loop` implied by `-D warnings`
[01:38:31] error: you are collect()ing an iterator and throwing away the result. Consider using an explicit for loop to exhaust the iterator
[01:38:31]   --> $DIR/for_loop.rs:156:5
[01:38:31]    |
[01:38:31]    |
[01:38:31] LL |     vec.iter().cloned().map(|x| out.push(x)).collect::<Vec<_>>();
[01:38:31]    |
[01:38:31]    = note: `-D clippy::unused-collect` implied by `-D warnings`
[01:38:31] 
[01:38:31] error: aborting due to 22 previous errors
[01:38:31] error: aborting due to 22 previous errors
[01:38:31] 
[01:38:31] 
[01:38:31] 
[01:38:31] diff of stderr:
[01:38:31] 
[01:38:31]  error: this range is empty so this for loop will never run
[01:38:31]     |
[01:38:31]  LL |     for i in 10..0 {
[01:38:31]     |              ^^^^^
[01:38:31]     |
[01:38:31]     |
[01:38:31]     = note: `-D clippy::reverse-range-loop` implied by `-D warnings`
[01:38:31]  help: consider using the following if you are attempting to iterate over this range in reverse
[01:38:31]     |
[01:38:31]  LL |     for i in (0..10).rev() {
[01:38:31]  
[01:38:31]  
[01:38:31]  error: this range is empty so this for loop will never run
[01:38:31]     |
[01:38:31]  LL |     for i in 10..=0 {
[01:38:31]     |              ^^^^^^
[01:38:31]  help: consider using the following if you are attempting to iterate over this range in reverse
[01:38:31]  help: consider using the following if you are attempting to iterate over this range in reverse
[01:38:31]     |
[01:38:31]  LL |     for i in (0...10).rev() {
[01:38:31]  
[01:38:31]  
[01:38:31]  error: this range is empty so this for loop will never run
[01:38:31]     |
[01:38:31]  LL |     for i in MAX_LEN..0 {
[01:38:31]     |              ^^^^^^^^^^
[01:38:31]  help: consider using the following if you are attempting to iterate over this range in reverse
[01:38:31]  help: consider using the following if you are attempting to iterate over this range in reverse
[01:38:31]     |
[01:38:31]  LL |     for i in (0..MAX_LEN).rev() {
[01:38:31]  
[01:38:31]  
[01:38:31]  error: this range is empty so this for loop will never run
[01:38:31]     |
[01:38:31]  LL |     for i in 5..5 {
[01:38:31]     |              ^^^^
[01:38:31]  
[01:38:31]  
[01:38:31]  error: this range is empty so this for loop will never run
[01:38:31]     |
[01:38:31]  LL |     for i in 10..5 + 4 {
[01:38:31]     |              ^^^^^^^^^
[01:38:31]  help: consider using the following if you are attempting to iterate over this range in reverse
[01:38:31]  help: consider using the following if you are attempting to iterate over this range in reverse
[01:38:31]     |
[01:38:31]  LL |     for i in (5 + 4..10).rev() {
[01:38:31]  
[01:38:31]  
[01:38:31]  error: this range is empty so this for loop will never run
[01:38:31]     |
[01:38:31]     |
[01:38:31]  LL |     for i in (5 + 2)..(3 - 1) {
[01:38:31]  help: consider using the following if you are attempting to iterate over this range in reverse
[01:38:31]     |
[01:38:31]     |
[01:38:31]  LL |     for i in ((3 - 1)..(5 + 2)).rev() {
[01:38:31]  
[01:38:31]  
[01:38:31]  error: this range is empty so this for loop will never run
[01:38:31]     |
[01:38:31]     |
[01:38:31]  LL |     for i in (5 + 2)..(8 - 1) {
[01:38:31]  
[01:38:31]  error: it is more concise to loop over references to containers instead of using explicit iteration methods
[01:38:31]    --> $DIR/for_loop.rs:107:15
[01:38:31]     |
---
[01:38:31]  
[01:38:31]  error: it is more concise to loop over containers instead of using explicit iteration methods`
[01:38:31]    --> $DIR/for_loop.rs:112:15
[01:38:31]     |
[01:38:31]  LL |     for _v in out_vec.into_iter() {}
[01:38:31] +   |               ^^^^^^^^^^^^^^^^^^^ help: to write this more concisely, try: `out_vec`
[01:38:31]     |
[01:38:31]     = note: `-D clippy::explicit-into-iter-loop` implied by `-D warnings`
[01:38:31]  
---
[01:38:31]  
[01:38:31]  error: it is more concise to loop over references to containers instead of using explicit iteration methods
[01:38:31]    --> $DIR/for_loop.rs:120:15
[01:38:31]     |
[01:38:31]  LL |     for _v in [1, 2, 3].iter() {}
[01:38:31] -   |               ^^^^^^^^^^^^^^^^
[01:38:31] +   |               ^^^^^^^^^^^^^^^^ help: to write this more concisely, try: `&[1, 2, 3]`
[01:38:31]  error: it is more concise to loop over references to containers instead of using explicit iteration methods
[01:38:31]    --> $DIR/for_loop.rs:124:15
[01:38:31]     |
[01:38:31]     |
[01:38:31]  LL |     for _v in [0; 32].iter() {}
[01:38:31] -   |               ^^^^^^^^^^^^^^
[01:38:31] +   |               ^^^^^^^^^^^^^^ help: to write this more concisely, try: `&[0; 32]`
[01:38:31]  error: it is more concise to loop over references to containers instead of using explicit iteration methods
[01:38:31]    --> $DIR/for_loop.rs:129:15
[01:38:31]     |
[01:38:31]     |
[01:38:31]  LL |     for _v in ll.iter() {}
[01:38:31] +   |               ^^^^^^^^^ help: to write this more concisely, try: `&ll`
[01:38:31]  
[01:38:31]  error: it is more concise to loop over references to containers instead of using explicit iteration methods
[01:38:31]    --> $DIR/for_loop.rs:132:15
[01:38:31]    --> $DIR/for_loop.rs:132:15
[01:38:31]     |
[01:38:31]  LL |     for _v in vd.iter() {}
[01:38:31] +   |               ^^^^^^^^^ help: to write this more concisely, try: `&vd`
[01:38:31]  
[01:38:31]  error: it is more concise to loop over references to containers instead of using explicit iteration methods
[01:38:31]    --> $DIR/for_loop.rs:135:15
[01:38:31]    --> $DIR/for_loop.rs:135:15
[01:38:31]     |
[01:38:31]  LL |     for _v in bh.iter() {}
[01:38:31] +   |               ^^^^^^^^^ help: to write this more concisely, try: `&bh`
[01:38:31]  
[01:38:31]  error: it is more concise to loop over references to containers instead of using explicit iteration methods
[01:38:31]    --> $DIR/for_loop.rs:138:15
[01:38:31]    --> $DIR/for_loop.rs:138:15
[01:38:31]     |
[01:38:31]  LL |     for _v in hm.iter() {}
[01:38:31] +   |               ^^^^^^^^^ help: to write this more concisely, try: `&hm`
[01:38:31]  
[01:38:31]  error: it is more concise to loop over references to containers instead of using explicit iteration methods
[01:38:31]    --> $DIR/for_loop.rs:141:15
[01:38:31]    --> $DIR/for_loop.rs:141:15
[01:38:31]     |
[01:38:31]  LL |     for _v in bt.iter() {}
[01:38:31] +   |               ^^^^^^^^^ help: to write this more concisely, try: `&bt`
[01:38:31]  
[01:38:31]  error: it is more concise to loop over references to containers instead of using explicit iteration methods
[01:38:31]    --> $DIR/for_loop.rs:144:15
[01:38:31]    --> $DIR/for_loop.rs:144:15
[01:38:31]     |
[01:38:31]  LL |     for _v in hs.iter() {}
[01:38:31] +   |               ^^^^^^^^^ help: to write this more concisely, try: `&hs`
[01:38:31]  
[01:38:31]  error: it is more concise to loop over references to containers instead of using explicit iteration methods
[01:38:31]    --> $DIR/for_loop.rs:147:15
[01:38:31]    --> $DIR/for_loop.rs:147:15
[01:38:31]     |
[01:38:31]  LL |     for _v in bs.iter() {}
[01:38:31] +   |               ^^^^^^^^^ help: to write this more concisely, try: `&bs`
[01:38:31]  
[01:38:31]  error: you are iterating over `Iterator::next()` which is an Option; this will compile but is probably not what you want
[01:38:31]    --> $DIR/for_loop.rs:149:15
[01:38:31]    --> $DIR/for_loop.rs:149:15
[01:38:31]     |
[01:38:31]  LL |     for _v in vec.iter().next() {}
[01:38:31]     |               ^^^^^^^^^^^^^^^^^
[01:38:31]     |
[01:38:31]     = note: `-D clippy::iter-next-loop` implied by `-D warnings`
[01:38:31]  error: you are collect()ing an iterator and throwing away the result. Consider using an explicit for loop to exhaust the iterator
[01:38:31]    --> $DIR/for_loop.rs:156:5
[01:38:31]     |
[01:38:31]     |
[01:38:31]  LL |     vec.iter().cloned().map(|x| out.push(x)).collect::<Vec<_>>();
[01:38:31]     |
[01:38:31]     = note: `-D clippy::unused-collect` implied by `-D warnings`
[01:38:31]  
[01:38:31]  error: aborting due to 22 previous errors
---
[01:38:31] 
[01:38:31] ------------------------------------------
[01:38:31] stderr:
[01:38:31] ------------------------------------------
[01:38:31] {"message":"this range is empty so this for loop will never run","code":{"code":"clippy::reverse_range_loop","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/for_loop.rs","byte_start":913,"byte_end":918,"line_start":40,"line_end":40,"column_start":14,"column_end":19,"is_primary":true,"text":[{"text":"    for i in 10..0 {","highlight_start":14,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::reverse-range-loop` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using the following if you are attempting to iterate over this range in reverse","code":null,"level":"help","spans":[{"file_name":"tests/ui/for_loop.rs","byte_start":913,"byte_end":918,"line_start":40,"line_end":40,"column_start":14,"column_end":19,"is_primary":true,"text":[{"text":"    for i in 10..0 {","highlight_start":14,"highlight_end":19}],"label":null,"suggested_replacement":"(0..10).rev()","suggestion_applicability":"MaybeIncorrect","expansion":{"span":{"file_name":"tests/ui/for_loop.rs","byte_start":913,"byte_end":918,"line_start":40,"line_end":40,"column_start":14,"column_end":19,"is_primary":false,"text":[{"text":"    for i in 10..0 {","highlight_start":14,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for loop`","def_site_span":{"file_name":"tests/ui/for_loop.rs","byte_start":913,"byte_end":918,"line_start":40,"line_end":40,"column_start":14,"column_end":19,"is_primary":false,"text":[{"text":"    for i in 10..0 {","highlight_start":14,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null}],"rendered":"error: this range is empty so this for loop will never run\n  --> tests/ui/for_loop.rs:40:14\n   |\nLL |     for i in 10..0 {\n   |              ^^^^^\n   |\n   = note: `-D clippy::reverse-range-loop` implied by `-D warnings`\nhelp: consider using the following if you are attempting to iterate over this range in reverse\n   |\nLL |     for i in (0..10).rev() {\n   |              ^^^^^^^^^^^^^\n\n"}
[01:38:31] {"message":"this range is empty so this for loop will never run","code":{"code":"clippy::reverse_range_loop","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/for_loop.rs","byte_start":968,"byte_end":974,"line_start":44,"line_end":44,"column_start":14,"column_end":20,"is_primary":true,"text":[{"text":"    for i in 10..=0 {","highlight_start":14,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using the following if you are attempting to iterate over this range in reverse","code":null,"level":"help","spans":[{"file_name":"tests/ui/for_loop.rs","byte_start":968,"byte_end":974,"line_start":44,"line_end":44,"column_start":14,"column_end":20,"is_primary":true,"text":[{"text":"    for i in 10..=0 {","highlight_start":14,"highlight_end":20}],"label":null,"suggested_replacement":"(0...10).rev()","suggestion_applicability":"MaybeIncorrect","expansion":{"span":{"file_name":"tests/ui/for_loop.rs","byte_start":968,"byte_end":974,"line_start":44,"line_end":44,"column_start":14,"column_end":20,"is_primary":false,"text":[{"text":"    for i in 10..=0 {","highlight_start":14,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for loop`","def_site_span":{"file_name":"tests/ui/for_loop.rs","byte_start":968,"byte_end":974,"line_start":44,"line_end":44,"column_start":14,"column_end":20,"is_primary":false,"text":[{"text":"    for i in 10..=0 {","highlight_start":14,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null}],"rendered":"error: this range is empty so this for loop will never run\n  --> tests/ui/for_loop.rs:44:14\n   |\nLL |     for i in 10..=0 {\n   |              ^^^^^^\nhelp: consider using the following if you are attempting to iterate over this range in reverse\n   |\nLL |     for i in (0...10).rev() {\n   |              ^^^^^^^^^^^^^^\n\n"}
[01:38:31] {"message":"this range is empty so this for loop will never run","code":{"code":"clippy::reverse_range_loop","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/for_loop.rs","byte_start":1024,"byte_end":1034,"line_start":48,"line_end":48,"column_start":14,"column_end":24,"is_primary":true,"text":[{"text":"    for i in MAX_LEN..0 {","highlight_start":14,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using the following if you are attempting to iterate over this range in reverse","code":null,"level":"help","spans":[{"file_name":"tests/ui/for_loop.rs","byte_start":1024,"byte_end":1034,"line_start":48,"line_end":48,"column_start":14,"column_end":24,"is_primary":true,"text":[{"text":"    for i in MAX_LEN..0 {","highlight_start":14,"highlight_end":24}],"label":null,"suggested_replacement":"(0..MAX_LEN).rev()","suggestion_applicability":"MaybeIncorrect","expansion":{"span":{"file_name":"tests/ui/for_loop.rs","byte_start":1024,"byte_end":1034,"line_start":48,"line_end":48,"column_start":14,"column_end":24,"is_primary":false,"text":[{"text":"    for i in MAX_LEN..0 {","highlight_start":14,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for loop`","def_site_span":{"file_name":"tests/ui/for_loop.rs","byte_start":1024,"byte_end":1034,"line_start":48,"line_end":48,"column_start":14,"column_end":24,"is_primary":false,"text":[{"text":"    for i in MAX_LEN..0 {","highlight_start":14,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null}],"rendered":"error: this range is empty so this for loop will never run\n  --> tests/ui/for_loop.rs:48:14\n   |\nLL |     for i in MAX_LEN..0 {\n   |              ^^^^^^^^^^\nhelp: consider using the following if you are attempting to iterate over this range in reverse\n   |\nLL |     for i in (0..MAX_LEN).rev() {\n   |              ^^^^^^^^^^^^^^^^^^\n\n"}
[01:38:31] {"message":"this range is empty so this for loop will never run","code":{"code":"clippy::reverse_range_loop","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/for_loop.rs","byte_start":1084,"byte_end":1088,"line_start":52,"line_end":52,"column_start":14,"column_end":18,"is_primary":true,"text":[{"text":"    for i in 5..5 {","highlight_start":14,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: this range is empty so this for loop will never run\n  --> tests/ui/for_loop.rs:52:14\n   |\nLL |     for i in 5..5 {\n   |              ^^^^\n\n"}
[01:38:31] {"message":"this range is empty so this for loop will never run","code":{"code":"clippy::reverse_range_loop","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/for_loop.rs","byte_start":1678,"byte_end":1687,"line_start":77,"line_end":77,"column_start":14,"column_end":23,"is_primary":true,"text":[{"text":"    for i in 10..5 + 4 {","highlight_start":14,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using the following if you are attempting to iterate over this range in reverse","code":null,"level":"help","spans":[{"file_name":"tests/ui/for_loop.rs","byte_start":1678,"byte_end":1687,"line_start":77,"line_end":77,"column_start":14,"column_end":23,"is_primary":true,"text":[{"text":"    for i in 10..5 + 4 {","highlight_start":14,"highlight_end":23}],"label":null,"suggested_replacement":"(5 + 4..10).rev()","suggestion_applicability":"MaybeIncorrect","expansion":{"span":{"file_name":"tests/ui/for_loop.rs","byte_start":1678,"byte_end":1687,"line_start":77,"line_end":77,"column_start":14,"column_end":23,"is_primary":false,"text":[{"text":"    for i in 10..5 + 4 {","highlight_start":14,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for loop`","def_site_span":{"file_name":"tests/ui/for_loop.rs","byte_start":1678,"byte_end":1687,"line_start":77,"line_end":77,"column_start":14,"column_end":23,"is_primary":false,"text":[{"text":"    for i in 10..5 + 4 {","highlight_start":14,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null}],"rendered":"error: this range is empty so this for loop will never run\n  --> tests/ui/for_loop.rs:77:14\n   |\nLL |     for i in 10..5 + 4 {\n   |              ^^^^^^^^^\nhelp: consider using the following if you are attempting to iterate over this range in reverse\n   |\nLL |     for i in (5 + 4..10).rev() {\n   |              ^^^^^^^^^^^^^^^^^\n\n"}
[01:38:31] {"message":"this range is empty so this for loop will never run","code":{"code":"clippy::reverse_range_loop","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/for_loop.rs","byte_start":1737,"byte_end":1753,"line_start":81,"line_end":81,"column_start":14,"column_end":30,"is_primary":true,"text":[{"text":"    for i in (5 + 2)..(3 - 1) {","highlight_start":14,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using the following if you are attempting to iterate over this range in reverse","code":null,"level":"help","spans":[{"file_name":"tests/ui/for_loop.rs","byte_start":1737,"byte_end":1753,"line_start":81,"line_end":81,"column_start":14,"column_end":30,"is_primary":true,"text":[{"text":"    for i in (5 + 2)..(3 - 1) {","highlight_start":14,"highlight_end":30}],"label":null,"suggested_replacement":"((3 - 1)..(5 + 2)).rev()","suggestion_applicability":"MaybeIncorrect","expansion":{"span":{"file_name":"tests/ui/for_loop.rs","byte_start":1737,"byte_end":1753,"line_start":81,"line_end":81,"column_start":14,"column_end":30,"is_primary":false,"text":[{"text":"    for i in (5 + 2)..(3 - 1) {","highlight_start":14,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for loop`","def_site_span":{"file_name":"tests/ui/for_loop.rs","byte_start":1737,"byte_end":1753,"line_start":81,"line_end":81,"column_start":14,"column_end":30,"is_primary":false,"text":[{"text":"    for i in (5 + 2)..(3 - 1) {","highlight_start":14,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null}],"rendered":"error: this range is empty so this for loop will never run\n  --> tests/ui/for_loop.rs:81:14\n   |\nLL |     for i in (5 + 2)..(3 - 1) {\n   |              ^^^^^^^^^^^^^^^^\nhelp: consider using the following if you are attempting to iterate over this range in reverse\n   |\nLL |     for i in ((3 - 1)..(5 + 2)).rev() {\n   |              ^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:38:31] {"message":"this range is empty so this for loop will never run","code":{"code":"clippy::reverse_range_loop","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/for_loop.rs","byte_start":1803,"byte_end":1819,"line_start":85,"line_end":85,"column_start":14,"column_end":30,"is_primary":true,"text":[{"text":"    for i in (5 + 2)..(8 - 1) {","highlight_start":14,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: this range is empty so this for loop will never run\n  --> tests/ui/for_loop.rs:85:14\n   |\nLL |     for i in (5 + 2)..(8 - 1) {\n   |              ^^^^^^^^^^^^^^^^\n\n"}
[01:38:31] {"message":"it is more concise to loop over references to containers instead of using explicit iteration methods","code":{"code":"clippy::explicit_iter_loop","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/for_loop.rs","byte_start":2255,"byte_end":2265,"line_start":107,"line_end":107,"column_start":15,"column_end":25,"is_primary":true,"text":[{"text":"    for _v in vec.iter() {}","highlight_start":15,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/for_loop.rs","byte_start":2255,"byte_end":2265,"line_start":107,"line_end":107,"column_start":15,"column_end":25,"is_primary":false,"text":[{"text":"    for _v in vec.iter() {}","highlight_start":15,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for loop`","def_site_span":{"file_name":"tests/ui/for_loop.rs","byte_start":2255,"byte_end":2265,"line_start":107,"line_end":107,"column_start":15,"column_end":25,"is_primary":false,"text":[{"text":"    for _v in vec.iter() {}","highlight_start":15,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"`-D clippy::explicit-iter-loop` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"to write this more concisely, try","code":null,"level":"help","spans":[{"file_name":"tests/ui/for_loop.rs","byte_start":2255,"byte_end":2265,"line_start":107,"line_end":107,"column_start":15,"column_end":25,"is_primary":true,"text":[{"text":"    for _v in vec.iter() {}","highlight_start":15,"highlight_end":25}],"label":null,"suggested_replacement":"&vec","suggestion_applicability":"MachineApplicable","expansion":{"span":{"file_name":"tests/ui/for_loop.rs","byte_start":2255,"byte_end":2265,"line_start":107,"line_end":107,"column_start":15,"column_end":25,"is_primary":false,"text":[{"text":"    for _v in vec.iter() {}","highlight_start":15,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for loop`","def_site_span":{"file_name":"tests/ui/for_loop.rs","byte_start":2255,"byte_end":2265,"line_start":107,"line_end":107,"column_start":15,"column_end":25,"is_primary":false,"text":[{"text":"    for _v in vec.iter() {}","highlight_start":15,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null}],"rendered":"error: it is more concise to loop over references to containers instead of using explicit iteration methods\n  --> tests/ui/for_loop.rs:107:15\n   |\nLL |     for _v in vec.iter() {}\n   |               ^^^^^^^^^^ help: to write this more concisely, try: `&vec`\n   |\n   = note: `-D clippy::explicit-iter-loop` implied by `-D warnings`\n\n"}
[01:38:31] {"message":"it is more concise to loop over references to containers instead of using explicit iteration methods","code":{"code":"clippy::explicit_iter_loop","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/for_loop.rs","byte_start":2284,"byte_end":2298,"line_start":109,"line_end":109,"column_start":15,"column_end":29,"is_primary":true,"text":[{"text":"    for _v in vec.iter_mut() {}","highlight_start":15,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/for_loop.rs","byte_start":2284,"byte_end":2298,"line_start":109,"line_end":109,"column_start":15,"column_end":29,"is_primary":false,"text":[{"text":"    for _v in vec.iter_mut() {}","highlight_start":15,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for loop`","def_site_span":{"file_name":"tests/ui/for_loop.rs","byte_start":2284,"byte_end":2298,"line_start":109,"line_end":109,"column_start":15,"column_end":29,"is_primary":false,"text":[{"text":"    for _v in vec.iter_mut() {}","highlight_start":15,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"to write this more concisely, try","code":null,"level":"help","spans":[{"file_name":"tests/ui/for_loop.rs","byte_start":2284,"byte_end":2298,"line_start":109,"line_end":109,"column_start":15,"column_end":29,"is_primary":true,"text":[{"text":"    for _v in vec.iter_mut() {}","highlight_start":15,"highlight_end":29}],"label":null,"suggested_replacement":"&mut vec","suggestion_applicability":"MachineApplicable","expansion":{"span":{"file_name":"tests/ui/for_loop.rs","byte_start":2284,"byte_end":2298,"line_start":109,"line_end":109,"column_start":15,"column_end":29,"is_primary":false,"text":[{"text":"    for _v in vec.iter_mut() {}","highlight_start":15,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for loop`","def_site_span":{"file_name":"tests/ui/for_loop.rs","byte_start":2284,"byte_end":2298,"line_start":109,"line_end":109,"column_start":15,"column_end":29,"is_primary":false,"text":[{"text":"    for _v in vec.iter_mut() {}","highlight_start":15,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null}],"rendered":"error: it is more concise to loop over references to containers instead of using explicit iteration methods\n  --> tests/ui/for_loop.rs:109:15\n   |\nLL |     for _v in vec.iter_mut() {}\n   |               ^^^^^^^^^^^^^^ help: to write this more concisely, try: `&mut vec`\n\n"}
[01:38:31] {"message":"it is more concise to loop over containers instead of using explicit iteration methods`","code":{"code":"clippy::explicit_into_iter_loop","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/for_loop.rs","byte_start":2350,"byte_end":2369,"line_start":112,"line_end":112,"column_start":15,"column_end":34,"is_primary":true,"text":[{"text":"    for _v in out_vec.into_iter() {}","highlight_start":15,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/for_loop.rs","byte_start":2350,"byte_end":2369,"line_start":112,"line_end":112,"column_start":15,"column_end":34,"is_primary":false,"text":[{"text":"    for _v in out_vec.into_iter() {}","highlight_start":15,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for loop`","def_site_span":{"file_name":"tests/ui/for_loop.rs","byte_start":2350,"byte_end":2369,"line_start":112,"line_end":112,"column_start":15,"column_end":34,"is_primary":false,"text":[{"text":"    for _v in out_vec.into_iter() {}","highlight_start":15,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"`-D clippy::explicit-into-iter-loop` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"to write this more concisely, try","code":null,"level":"help","spans":[{"file_name":"tests/ui/for_loop.rs","byte_start":2350,"byte_end":2369,"line_start":112,"line_end":112,"column_start":15,"column_end":34,"is_primary":true,"text":[{"text":"    for _v in out_vec.into_iter() {}","highlight_start":15,"highlight_end":34}],"label":null,"suggested_replacement":"out_vec","suggestion_applicability":"MachineApplicable","expansion":{"span":{"file_name":"tests/ui/for_loop.rs","byte_start":2350,"byte_end":2369,"line_start":112,"line_end":112,"column_start":15,"column_end":34,"is_primary":false,"text":[{"text":"    for _v in out_vec.into_iter() {}","highlight_start":15,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for loop`","def_site_span":{"file_name":"tests/ui/for_loop.rs","byte_start":2350,"byte_end":2369,"line_start":112,"line_end":112,"column_start":15,"column_end":34,"is_primary":false,"text":[{"text":"    for _v in out_vec.into_iter() {}","highlight_start":15,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null}],"rendered":"error: it is more concise to loop over containers instead of using explicit iteration methods`\n  --> tests/ui/for_loop.rs:112:15\n   |\nLL |     for _v in out_vec.into_iter() {}\n   |               ^^^^^^^^^^^^^^^^^^^ help: to write this more concisely, try: `out_vec`\n   |\n   = note: `-D clippy::explicit-into-iter-loop` implied by `-D warnings`\n\n"}
[01:38:31] {"message":"it is more concise to loop over references to containers instead of using explicit iteration methods","code":{"code":"clippy::explicit_iter_loop","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/for_loop.rs","byte_start":2415,"byte_end":2432,"line_start":115,"line_end":115,"column_start":15,"column_end":32,"is_primary":true,"text":[{"text":"    for _v in array.into_iter() {}","highlight_start":15,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/for_loop.rs","byte_start":2415,"byte_end":2432,"line_start":115,"line_end":115,"column_start":15,"column_end":32,"is_primary":false,"text":[{"text":"    for _v in array.into_iter() {}","highlight_start":15,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for loop`","def_site_span":{"file_name":"tests/ui/for_loop.rs","byte_start":2415,"byte_end":2432,"line_start":115,"line_end":115,"column_start":15,"column_end":32,"is_primary":false,"text":[{"text":"    for _v in array.into_iter() {}","highlight_start":15,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"to write this more concisely, try","code":null,"level":"help","spans":[{"file_name":"tests/ui/for_loop.rs","byte_start":2415,"byte_end":2432,"line_start":115,"line_end":115,"column_start":15,"column_end":32,"is_primary":true,"text":[{"text":"    for _v in array.into_iter() {}","highlight_start":15,"highlight_end":32}],"label":null,"suggested_replacement":"&array","suggestion_applicability":"MachineApplicable","expansion":{"span":{"file_name":"tests/ui/for_loop.rs","byte_start":2415,"byte_end":2432,"line_start":115,"line_end":115,"column_start":15,"column_end":32,"is_primary":false,"text":[{"text":"    for _v in array.into_iter() {}","highlight_start":15,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for loop`","def_site_span":{"file_name":"tests/ui/for_loop.rs","byte_start":2415,"byte_end":2432,"line_start":115,"line_end":115,"column_start":15,"column_end":32,"is_primary":false,"text":[{"text":"    for _v in array.into_iter() {}","highlight_start":15,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null}],"rendered":"error: it is more concise to loop over references to containers instead of using explicit iteration methods\n  --> tests/ui/for_loop.rs:115:15\n   |\nLL |     for _v in array.into_iter() {}\n   |               ^^^^^^^^^^^^^^^^^ help: to write this more concisely, try: `&array`\n\n"}
[01:38:31] {"message":"it is more concise to loop over references to containers instead of using explicit iteration methods","code":{"code":"clippy::explicit_iter_loop","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/for_loop.rs","byte_start":2536,"byte_end":2552,"line_start":120,"line_end":120,"column_start":15,"column_end":31,"is_primary":true,"text":[{"text":"    for _v in [1, 2, 3].iter() {}","highlight_start":15,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/for_loop.rs","byte_start":2536,"byte_end":2552,"line_start":120,"line_end":120,"column_start":15,"column_end":31,"is_primary":false,"text":[{"text":"    for _v in [1, 2, 3].iter() {}","highlight_start":15,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for loop`","def_site_span":{"file_name":"tests/ui/for_loop.rs","byte_start":2536,"byte_end":2552,"line_start":120,"line_end":120,"column_start":15,"column_end":31,"is_primary":false,"text":[{"text":"    for _v in [1, 2, 3].iter() {}","highlight_start":15,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"to write this more concisely, try","code":null,"level":"help","spans":[{"file_name":"tests/ui/for_loop.rs","byte_start":2536,"byte_end":2552,"line_start":120,"line_end":120,"column_start":15,"column_end":31,"is_primary":true,"text":[{"text":"    for _v in [1, 2, 3].iter() {}","highlight_start":15,"highlight_end":31}],"label":null,"suggested_replacement":"&[1, 2, 3]","suggestion_applicability":"MachineApplicable","expansion":{"span":{"file_name":"tests/ui/for_loop.rs","byte_start":2536,"byte_end":2552,"line_start":120,"line_end":120,"column_start":15,"column_end":31,"is_primary":false,"text":[{"text":"    for _v in [1, 2, 3].iter() {}","highlight_start":15,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for loop`","def_site_span":{"file_name":"tests/ui/for_loop.rs","byte_start":2536,"byte_end":2552,"line_start":120,"line_end":120,"column_start":15,"column_end":31,"is_primary":false,"text":[{"text":"    for _v in [1, 2, 3].iter() {}","highlight_start":15,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null}],"rendered":"error: it is more concise to loop over references to containers instead of using explicit iteration methods\n  --> tests/ui/for_loop.rs:120:15\n   |\nLL |     for _v in [1, 2, 3].iter() {}\n   |               ^^^^^^^^^^^^^^^^ help: to write this more concisely, try: `&[1, 2, 3]`\n\n"}
[01:38:31] error: test failed, to rerun pass '--test compile-test'
[01:38:31] {"message":"it is more concise to loop over references to containers instead of using explicit iteration methods","code":{"code":"clippy::explicit_iter_loop","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/for_loop.rs","byte_start":2625,"byte_end":2639,"line_start":124,"line_end":124,"column_start":15,"column_end":29,"is_primary":true,"text":[{"text":"    for _v in [0; 32].iter() {}","highlight_start":15,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/for_loop.rs","byte_start":2625,"byte_end":2639,"line_start":124,"line_end":124,"column_start":15,"column_end":29,"is_primary":false,"text":[{"text":"    for _v in [0; 32].iter() {}","highlight_start":15,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for loop`","def_site_span":{"file_name":"tests/ui/for_loop.rs","byte_start":2625,"byte_end":2639,"line_start":124,"line_end":124,"column_start":15,"column_end":29,"is_primary":false,"text":[{"text":"    for _v in [0; 32].iter() {}","highlight_start":15,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"to write this more concisely, try","code":null,"level":"help","spans":[{"file_name":"tests/ui/for_loop.rs","byte_start":2625,"byte_end":2639,"line_start":124,"line_end":124,"column_start":15,"column_end":29,"is_primary":true,"text":[{"text":"    for _v in [0; 32].iter() {}","highlight_start":15,"highlight_end":29}],"label":null,"suggested_replacement":"&[0; 32]","suggestion_applicability":"MachineApplicable","expansion":{"span":{"file_name":"tests/ui/for_loop.rs","byte_start":2625,"byte_end":2639,"line_start":124,"line_end":124,"column_start":15,"column_end":29,"is_primary":false,"text":[{"text":"    for _v in [0; 32].iter() {}","highlight_start":15,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for loop`","def_site_span":{"file_name":"tests/ui/for_loop.rs","byte_start":2625,"byte_end":2639,"line_start":124,"line_end":124,"column_start":15,"column_end":29,"is_primary":false,"text":[{"text":"    for _v in [0; 32].iter() {}","highlight_start":15,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null}],"rendered":"error: it is more concise to loop over references to containers instead of using explicit iteration methods\n  --> tests/ui/for_loop.rs:124:15\n   |\nLL |     for _v in [0; 32].iter() {}\n   |               ^^^^^^^^^^^^^^ help: to write this more concisely, try: `&[0; 32]`\n\n"}
[01:38:31] {"message":"it is more concise to loop over references to containers instead of using explicit iteration methods","code":{"code":"clippy::explicit_iter_loop","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/for_loop.rs","byte_start":2751,"byte_end":2760,"line_start":129,"line_end":129,"column_start":15,"column_end":24,"is_primary":true,"text":[{"text":"    for _v in ll.iter() {}","highlight_start":15,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/for_loop.rs","byte_start":2751,"byte_end":2760,"line_start":129,"line_end":129,"column_start":15,"column_end":24,"is_primary":false,"text":[{"text":"    for _v in ll.iter() {}","highlight_start":15,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for loop`","def_site_span":{"file_name":"tests/ui/for_loop.rs","byte_start":2751,"byte_end":2760,"line_start":129,"line_end":129,"column_start":15,"column_end":24,"is_primary":false,"text":[{"text":"    for _v in ll.iter() {}","highlight_start":15,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"to write this more concisely, try","code":null,"level":"help","spans":[{"file_name":"tests/ui/for_loop.rs","byte_start":2751,"byte_end":2760,"line_start":129,"line_end":129,"column_start":15,"column_end":24,"is_primary":true,"text":[{"text":"    for _v in ll.iter() {}","highlight_start":15,"highlight_end":24}],"label":null,"suggested_replacement":"&ll","suggestion_applicability":"MachineApplicable","expansion":{"span":{"file_name":"tests/ui/for_loop.rs","byte_start":2751,"byte_end":2760,"line_start":129,"line_end":129,"column_start":15,"column_end":24,"is_primary":false,"text":[{"text":"    for _v in ll.iter() {}","highlight_start":15,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for loop`","def_site_span":{"file_name":"tests/ui/for_loop.rs","byte_start":2751,"byte_end":2760,"line_start":129,"line_end":129,"column_start":15,"column_end":24,"is_primary":false,"text":[{"text":"    for _v in ll.iter() {}","highlight_start":15,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null}],"rendered":"error: it is more concise to loop over references to containers instead of using explicit iteration methods\n  --> tests/ui/for_loop.rs:129:15\n   |\nLL |     for _v in ll.iter() {}\n   |               ^^^^^^^^^ help: to write this more concisely, try: `&ll`\n\n"}
[01:38:31] {"message":"it is more concise to loop over references to containers instead of using explicit iteration methods","code":{"code":"clippy::explicit_iter_loop","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/for_loop.rs","byte_start":2823,"byte_end":2832,"line_start":132,"line_end":132,"column_start":15,"column_end":24,"is_primary":true,"text":[{"text":"    for _v in vd.iter() {}","highlight_start":15,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/for_loop.rs","byte_start":2823,"byte_end":2832,"line_start":132,"line_end":132,"column_start":15,"column_end":24,"is_primary":false,"text":[{"text":"    for _v in vd.iter() {}","highlight_start":15,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for loop`","def_site_span":{"file_name":"tests/ui/for_loop.rs","byte_start":2823,"byte_end":2832,"line_start":132,"line_end":132,"column_start":15,"column_end":24,"is_primary":false,"text":[{"text":"    for _v in vd.iter() {}","highlight_start":15,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"to write this more concisely, try","code":null,"level":"help","spans":[{"file_name":"tests/ui/for_loop.rs","byte_start":2823,"byte_end":2832,"line_start":132,"line_end":132,"column_start":15,"column_end":24,"is_primary":true,"text":[{"text":"    for _v in vd.iter() {}","highlight_start":15,"highlight_end":24}],"label":null,"suggested_replacement":"&vd","suggestion_applicability":"MachineApplicable","expansion":{"span":{"file_name":"tests/ui/for_loop.rs","byte_start":2823,"byte_end":2832,"line_start":132,"line_end":132,"column_start":15,"column_end":24,"is_primary":false,"text":[{"text":"    for _v in vd.iter() {}","highlight_start":15,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for loop`","def_site_span":{"file_name":"tests/ui/for_loop.rs","byte_start":2823,"byte_end":2832,"line_start":132,"line_end":132,"column_start":15,"column_end":24,"is_primary":false,"text":[{"text":"    for _v in vd.iter() {}","highlight_start":15,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null}],"rendered":"error: it is more concise to loop over references to containers instead of using explicit iteration methods\n  --> tests/ui/for_loop.rs:132:15\n   |\nLL |     for _v in vd.iter() {}\n   |               ^^^^^^^^^ help: to write this more concisely, try: `&vd`\n\n"}
[01:38:31] {"message":"it is more concise to loop over references to containers instead of using explicit iteration methods","code":{"code":"clippy::explicit_iter_loop","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/for_loop.rs","byte_start":2899,"byte_end":2908,"line_start":135,"line_end":135,"column_start":15,"column_end":24,"is_primary":true,"text":[{"text":"    for _v in bh.iter() {}","highlight_start":15,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/for_loop.rs","byte_start":2899,"byte_end":2908,"line_start":135,"line_end":135,"column_start":15,"column_end":24,"is_primary":false,"text":[{"text":"    for _v in bh.iter() {}","highlight_start":15,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for loop`","def_site_span":{"file_name":"tests/ui/for_loop.rs","byte_start":2899,"byte_end":2908,"line_start":135,"line_end":135,"column_start":15,"column_end":24,"is_primary":false,"text":[{"text":"    for _v in bh.iter() {}","highlight_start":15,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"to write this more concisely, try","code":null,"level":"help","spans":[{"file_name":"tests/ui/for_loop.rs","byte_start":2899,"byte_end":2908,"line_start":135,"line_end":135,"column_start":15,"column_end":24,"is_primary":true,"text":[{"text":"    for _v in bh.iter() {}","highlight_start":15,"highlight_end":24}],"label":null,"suggested_replacement":"&bh","suggestion_applicability":"MachineApplicable","expansion":{"span":{"file_name":"tests/ui/for_loop.rs","byte_start":2899,"byte_end":2908,"line_start":135,"line_end":135,"column_start":15,"column_end":24,"is_primary":false,"text":[{"text":"    for _v in bh.iter() {}","highlight_start":15,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for loop`","def_site_span":{"file_name":"tests/ui/for_loop.rs","byte_start":2899,"byte_end":2908,"line_start":135,"line_end":135,"column_start":15,"column_end":24,"is_primary":false,"text":[{"text":"    for _v in bh.iter() {}","highlight_start":15,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null}],"rendered":"error: it is more concise to loop over references to containers instead of using explicit iteration methods\n  --> tests/ui/for_loop.rs:135:15\n   |\nLL |     for _v in bh.iter() {}\n   |               ^^^^^^^^^ help: to write this more concisely, try: `&bh`\n\n"}
[01:38:31] {"message":"it is more concise to loop over references to containers instead of using explicit iteration methods","code":{"code":"clippy::explicit_iter_loop","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/for_loop.rs","byte_start":2973,"byte_end":2982,"line_start":138,"line_end":138,"column_start":15,"column_end":24,"is_primary":true,"text":[{"text":"    for _v in hm.iter() {}","highlight_start":15,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/for_loop.rs","byte_start":2973,"byte_end":2982,"line_start":138,"line_end":138,"column_start":15,"column_end":24,"is_primary":false,"text":[{"text":"    for _v in hm.iter() {}","highlight_start":15,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for loop`","def_site_span":{"file_name":"tests/ui/for_loop.rs","byte_start":2973,"byte_end":2982,"line_start":138,"line_end":138,"column_start":15,"column_end":24,"is_primary":false,"text":[{"text":"    for _v in hm.iter() {}","highlight_start":15,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"to write this more concisely, try","code":null,"level":"help","spans":[{"file_name":"tests/ui/for_loop.rs","byte_start":2973,"byte_end":2982,"line_start":138,"line_end":138,"column_start":15,"column_end":24,"is_primary":true,"text":[{"text":"    for _v in hm.iter() {}","highlight_start":15,"highlight_end":24}],"label":null,"suggested_replacement":"&hm","suggestion_applicability":"MachineApplicable","expansion":{"span":{"file_name":"tests/ui/for_loop.rs","byte_start":2973,"byte_end":2982,"line_start":138,"line_end":138,"column_start":15,"column_end":24,"is_primary":false,"text":[{"text":"    for _v in hm.iter() {}","highlight_start":15,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for loop`","def_site_span":{"file_name":"tests/ui/for_loop.rs","byte_start":2973,"byte_end":2982,"line_start":138,"line_end":138,"column_start":15,"column_end":24,"is_primary":false,"text":[{"text":"    for _v in hm.iter() {}","highlight_start":15,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null}],"rendered":"error: it is more concise to loop over references to containers instead of using explicit iteration methods\n  --> tests/ui/for_loop.rs:138:15\n   |\nLL |     for _v in hm.iter() {}\n   |               ^^^^^^^^^ help: to write this more concisely, try: `&hm`\n\n"}
[01:38:32] {"message":"it is more concise to loop over references to containers instead of using explicit iteration methods","code":{"code":"clippy::explicit_iter_loop","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/for_loop.rs","byte_start":3049,"byte_end":3058,"line_start":141,"line_end":141,"column_start":15,"column_end":24,"is_primary":true,"text":[{"text":"    for _v in bt.iter() {}","highlight_start":15,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/for_loop.rs","byte_start":3049,"byte_end":3058,"line_start":141,"line_end":141,"column_start":15,"column_end":24,"is_primary":false,"text":[{"text":"    for _v in bt.iter() {}","highlight_start":15,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for loop`","def_site_span":{"file_name":"tests/ui/for_loop.rs","byte_start":3049,"byte_end":3058,"line_start":141,"line_end":141,"column_start":15,"column_end":24,"is_primary":false,"text":[{"text":"    for _v in bt.iter() {}","highlight_start":15,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"to write this more concisely, try","code":null,"level":"help","spans":[{"file_name":"tests/ui/for_loop.rs","byte_start":3049,"byte_end":3058,"line_start":141,"line_end":141,"column_start":15,"column_end":24,"is_primary":true,"text":[{"text":"    for _v in bt.iter() {}","highlight_start":15,"highlight_end":24}],"label":null,"suggested_replacement":"&bt","suggestion_applicability":"MachineApplicable","expansion":{"span":{"file_name":"tests/ui/for_loop.rs","byte_start":3049,"byte_end":3058,"line_start":141,"line_end":141,"column_start":15,"column_end":24,"is_primary":false,"text":[{"text":"    for _v in bt.iter() {}","highlight_start":15,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for loop`","def_site_span":{"file_name":"tests/ui/for_loop.rs","byte_start":3049,"byte_end":3058,"line_start":141,"line_end":141,"column_start":15,"column_end":24,"is_primary":false,"text":[{"text":"    for _v in bt.iter() {}","highlight_start":15,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null}],"rendered":"error: it is more concise to loop over references to containers instead of using explicit iteration methods\n  --> tests/ui/for_loop.rs:141:15\n   |\nLL |     for _v in bt.iter() {}\n   |               ^^^^^^^^^ help: to write this more concisely, try: `&bt`\n\n"}
[01:38:32] {"message":"it is more concise to loop over references to containers instead of using explicit iteration methods","code":{"code":"clippy::explicit_iter_loop","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/for_loop.rs","byte_start":3119,"byte_end":3128,"line_start":144,"line_end":144,"column_start":15,"column_end":24,"is_primary":true,"text":[{"text":"    for _v in hs.iter() {}","highlight_start":15,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/for_loop.rs","byte_start":3119,"byte_end":3128,"line_start":144,"line_end":144,"column_start":15,"column_end":24,"is_primary":false,"text":[{"text":"    for _v in hs.iter() {}","highlight_start":15,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for loop`","def_site_span":{"file_name":"tests/ui/for_loop.rs","byte_start":3119,"byte_end":3128,"line_start":144,"line_end":144,"column_start":15,"column_end":24,"is_primary":false,"text":[{"text":"    for _v in hs.iter() {}","highlight_start":15,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"to write this more concisely, try","code":null,"level":"help","spans":[{"file_name":"tests/ui/for_loop.rs","byte_start":3119,"byte_end":3128,"line_start":144,"line_end":144,"column_start":15,"column_end":24,"is_primary":true,"text":[{"text":"    for _v in hs.iter() {}","highlight_start":15,"highlight_end":24}],"label":null,"suggested_replacement":"&hs","suggestion_applicability":"MachineApplicable","expansion":{"span":{"file_name":"tests/ui/for_loop.rs","byte_start":3119,"byte_end":3128,"line_start":144,"line_end":144,"column_start":15,"column_end":24,"is_primary":false,"text":[{"text":"    for _v in hs.iter() {}","highlight_start":15,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for loop`","def_site_span":{"file_name":"tests/ui/for_loop.rs","byte_start":3119,"byte_end":3128,"line_start":144,"line_end":144,"column_start":15,"column_end":24,"is_primary":false,"text":[{"text":"    for _v in hs.iter() {}","highlight_start":15,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null}],"rendered":"error: it is more concise to loop over references to containers instead of using explicit iteration methods\n  --> tests/ui/for_loop.rs:144:15\n   |\nLL |     for _v in hs.iter() {}\n   |               ^^^^^^^^^ help: to write this more concisely, try: `&hs`\n\n"}
[01:38:32] {"message":"it is more concise to loop over references to containers instead of using explicit iteration methods","code":{"code":"clippy::explicit_iter_loop","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/for_loop.rs","byte_start":3191,"byte_end":3200,"line_start":147,"line_end":147,"column_start":15,"column_end":24,"is_primary":true,"text":[{"text":"    for _v in bs.iter() {}","highlight_start":15,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/for_loop.rs","byte_start":3191,"byte_end":3200,"line_start":147,"line_end":147,"column_start":15,"column_end":24,"is_primary":false,"text":[{"text":"    for _v in bs.iter() {}","highlight_start":15,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for loop`","def_site_span":{"file_name":"tests/ui/for_loop.rs","byte_start":3191,"byte_end":3200,"line_start":147,"line_end":147,"column_start":15,"column_end":24,"is_primary":false,"text":[{"text":"    for _v in bs.iter() {}","highlight_start":15,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"to write this more concisely, try","code":null,"level":"help","spans":[{"file_name":"tests/ui/for_loop.rs","byte_start":3191,"byte_end":3200,"line_start":147,"line_end":147,"column_start":15,"column_end":24,"is_primary":true,"text":[{"text":"    for _v in bs.iter() {}","highlight_start":15,"highlight_end":24}],"label":null,"suggested_replacement":"&bs","suggestion_applicability":"MachineApplicable","expansion":{"span":{"file_name":"tests/ui/for_loop.rs","byte_start":3191,"byte_end":3200,"line_start":147,"line_end":147,"column_start":15,"column_end":24,"is_primary":false,"text":[{"text":"    for _v in bs.iter() {}","highlight_start":15,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `for loop`","def_site_span":{"file_name":"tests/ui/for_loop.rs","byte_start":3191,"byte_end":3200,"line_start":147,"line_end":147,"column_start":15,"column_end":24,"is_primary":false,"text":[{"text":"    for _v in bs.iter() {}","highlight_start":15,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null}],"rendered":"error: it is more concise to loop over references to containers instead of using explicit iteration methods\n  --> tests/ui/for_loop.rs:147:15\n   |\nLL |     for _v in bs.iter() {}\n   |               ^^^^^^^^^ help: to write this more concisely, try: `&bs`\n\n"}
[01:38:32] {"message":"you are iterating over `Iterator::next()` which is an Option; this will compile but is probably not what you want","code":{"code":"clippy::iter_next_loop","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/for_loop.rs","byte_start":3219,"byte_end":3236,"line_start":149,"line_end":149,"column_start":15,"column_end":32,"is_primary":true,"text":[{"text":"    for _v in vec.iter().next() {}","highlight_start":15,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::iter-next-loop` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: you are iterating over `Iterator::next()` which is an Option; this will compile but is probably not what you want\n  --> tests/ui/for_loop.rs:149:15\n   |\nLL |     for _v in vec.iter().next() {}\n   |               ^^^^^^^^^^^^^^^^^\n   |\n   = note: `-D clippy::iter-next-loop` implied by `-D warnings`\n\n"}
[01:38:32] {"message":"you are collect()ing an iterator and throwing away the result. Consider using an explicit for loop to exhaust the iterator","code":{"code":"clippy::unused_collect","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/for_loop.rs","byte_start":3379,"byte_end":3439,"line_start":156,"line_end":156,"column_start":5,"column_end":65,"is_primary":true,"text":[{"text":"    vec.iter().cloned().map(|x| out.push(x)).collect::<Vec<_>>();","highlight_start":5,"highlight_end":65}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::unused-collect` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: you are collect()ing an iterator and throwing away the result. Consider using an explicit for loop to exhaust the iterator\n  --> tests/ui/for_loop.rs:156:5\n   |\nLL |     vec.iter().cloned().map(|x| out.push(x)).collect::<Vec<_>>();\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: `-D clippy::unused-collect` implied by `-D warnings`\n\n"}
[01:38:32] 
[01:38:32] ------------------------------------------
[01:38:32] 
[01:38:32] thread '[ui] ui/for_loop.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
---
travis_time:end:1f403d2c:start=1559871704241663354,finish=1559871704248058449,duration=6395095
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:12aebd88
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:012edde3
travis_time:start:012edde3
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:14474d0a
$ dmesg | grep -i kill
