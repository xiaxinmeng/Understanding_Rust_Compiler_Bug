plain
[00:46:32] ..............................i.....................................................................
[00:46:37] ....................................................................................................
[00:46:41] ....................................................................................................
[00:46:46] ....................................................................................................
[00:46:51] .................................................................................F.F.....F..........
[00:47:04] ....................................................................................................
[00:47:10] ....................................................................................................
[00:47:17] ....................................i...............................................................
[00:47:23] ............i.......................................................................................
---
[00:47:47]  diff of stderr:
[00:47:47] 
[00:47:47] - error: compilation successful
[00:47:47] -   --> $DIR/borrowing.rs:15:1
[00:47:47] + error[E0597]: `a` does not live long enough
[00:47:47] +   --> $DIR/borrowing.rs:18:18
[00:47:47] 3    |
[00:47:47] - LL | / fn main() { #![rustc_error] // rust-lang/rust#49855
[00:47:47] - LL | |     let _b = {
[00:47:47] - LL | |         let a = 3;
[00:47:47] - LL | |         unsafe { (|| yield &a).resume() }
[00:47:47] - ...  |
[00:47:47] - LL | |     };
[00:47:47] - LL | | }
[00:47:47] -    | |_^
[00:47:47] + LL |         unsafe { (|| yield &a).resume() }
[00:47:47] +    |                  |
[00:47:47] +    |                  |
[00:47:47] +    |                  borrowed value does not live long enough
[00:47:47] +    |                  borrow may end up in a temporary, created here
[00:47:47] + LL |         //~^ ERROR: `a` does not live long enough
[00:47:47] + LL |     };
[00:47:47] +    |     -- temporary later dropped here, potentially using the reference
[00:47:47] +    |     |
[00:47:47] +    |     borrowed value only lives until here
[00:47:47] - error: aborting due to previous error
[00:47:47] - error: aborting due to previous error
[00:47:47] + error[E0597]: `a` does not live long enough
[00:47:47] +   --> $DIR/borrowing.rs:24:9
[00:47:47] +    |
[00:47:47] + LL | /         || {
[00:47:47] + LL | |             yield &a
[00:47:47] + LL | |             //~^ ERROR: `a` does not live long enough
[00:47:47] + LL | |         }
[00:47:47] +    | |_________^ borrowed value does not live long enough
[00:47:47] + LL |       };
[00:47:47] +    |       - borrowed value only lives until here
[00:47:47] + LL |   }
[00:47:47] +    |   - borrow later used here, when `_b` is dropped
[00:47:47] + error: aborting due to 2 previous errors
[00:47:47] + 
[00:47:47] + For more information about this error, try `rustc --explain E0597`.
[00:47:47] 15 
[00:47:47] 15 
[00:47:47] 
[00:47:47] 
[00:47:47] The actual stderr differed from the expected stderr.
[00:47:47] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/borron here, `x` is created before `y` and therefore has a greater lifetime. Always\nkeep in mind that values in a scope are dropped in the opposite order they are\ncreated. So to fix the previous example, just make the `y` lifetime greater than\nthe `x`'s one:\n\n