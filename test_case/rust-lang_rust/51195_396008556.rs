
error[E0597]: `y` does not live long enough
  --> src/test/ui/error-codes/E0597.rs:18:16
   |
18 |     x.x = Some(&y);
   |                ^^ borrowed value does not live long enough
19 |     //~^ `y` does not live long enough [E0597]
20 | }
   | -
   | |
   | borrowed value only lives until here
   | borrow later used here, when `x` is dropped
